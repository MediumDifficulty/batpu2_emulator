pub mod cli;
pub mod transpiler;
pub mod interface;
pub mod ui;

use std::{fs, path::PathBuf, process::Command, thread, time::{Duration, Instant}};

use anyhow::Result;
use clap::Parser;
use cli::Args;
use ui::ui_main;

fn main() {
    let args = Args::parse();

    let extension = match args.input.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => {
            println!("Error: Input file must be an assembly or .mc file");
            return;
        }
    };

    fs::create_dir_all("temp").unwrap();

    let mut assembled_file = args.input.clone();

    if extension != "mc" {
        assemble_file(args.input.to_str().unwrap());
        assembled_file = PathBuf::from("temp/assembled.mc");
    }

    let assembly = fs::read_to_string(assembled_file).unwrap();

    let output = transpiler::transpile(&assembly, false);
    compile_asm(&output, "compiled").unwrap();

    if args.benchmark {
        let output = transpiler::transpile(&assembly, true);
        compile_asm(&output, "instruction_count").unwrap();
        println!("Counting instructions...");
        let (instruction_count, _) = emulator_main("instruction_count", 1);
        let instruction_count = instruction_count * args.iterations;
        println!("Instruction count: {instruction_count}");

        let emulator_thread = thread::spawn(move || {
            emulator_main("compiled", args.iterations).1
        });

        if !args.no_gui {
            ui_main();
        }

        let time = emulator_thread.join().unwrap();
        println!("Emulator ran {instruction_count} instructions in {}ms ({}mips)", time.as_millis(), ((instruction_count as u128 / time.as_millis()) / 1000))

    } else {
        let emulator_thread = thread::spawn(move || emulator_main("compiled", args.iterations));
        if !args.no_gui {
            ui_main();
        }
        emulator_thread.join().unwrap();
    }
}

fn emulator_main(name: &str, iterations: usize) -> (usize, Duration) {
    let mut memory: [u8; 256] = [0; 256];
    let mut registers: [u8; 16] = [0; 16];

    let mut instruction_count = 0;

    let execution_time;
    unsafe {
        let lib = libloading::Library::new(format!("temp/{name}.dll")).unwrap();
        let main: libloading::Symbol<CompiledMain> = lib.get(b"_main").unwrap();
        // println!("Running");
        let mem_ptr = memory.as_mut_ptr();
        let reg_ptr = registers.as_mut_ptr();
        // println!("{:?}", mem_ptr);
        let start_time = Instant::now();
        #[allow(unused_assignments)]
        for _ in 0..iterations {
            memory = [0; 256];
            registers = [0; 16];
            main(mem_ptr, reg_ptr, interface::on_mem_read, interface::on_mem_write, &mut instruction_count as *mut usize);
        }
        execution_time = start_time.elapsed();
    }
    // println!("{:?}", registers)
    (instruction_count, execution_time)
}

fn assemble_file(file: &str) {
    if !Command::new("python")
        .arg("assembler/main.py")
        .arg(file)
        .arg("temp/assembled.mc")
        .status().expect("Python failed to run")
        .success() {
        panic!("Error: Failed to assemble file");
    }
}

fn compile_asm(src: &str, name: &str) -> Result<()> {
    fs::write(format!("temp/{name}.asm"), src).unwrap();

    if !Command::new("nasm")
        .arg("-f")
        .arg("win64")
        .arg(format!("temp/{name}.asm"))
        .arg("-O0")
        .arg("-o")
        .arg(format!("temp/{name}.obj"))
        .status().expect("Nasm failed to run")
        .success() {
            panic!("Error: Failed to compile assembly");
        }

    if !Command::new("golink")
        .args(["/dll", "/entry", "_DllMain", format!("temp/{name}.obj").as_str()])
        .status().expect("Golink failed to run")
        .success() {
            panic!("Error: Failed to link assembly");
        }

    Ok(())
}

type MemoryHandler = unsafe extern "C" fn(mem_space: *mut u8, addr: usize);

type CompiledMain = unsafe extern "C" fn(mem_space: *mut u8, registers: *mut u8, on_mem_read: MemoryHandler, on_mem_write: MemoryHandler, instruction_count: *mut usize);