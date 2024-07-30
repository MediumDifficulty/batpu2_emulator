pub mod cli;
pub mod transpiler;
pub mod interface;

use std::{fs, path::PathBuf, process::Command};

use anyhow::Result;
use clap::Parser;
use cli::Args;

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

    let output = transpiler::transpile(&assembly);
    compile_asm_and(&output).unwrap();

    let mut memory: [u8; 256] = [0; 256];

    unsafe {
        let lib = libloading::Library::new("temp/temp.dll").unwrap();
        let main: libloading::Symbol<CompiledMain> = lib.get(b"_main").unwrap();
        println!("Running");
        let mem_ptr = memory.as_mut_ptr();
        println!("{:?}", mem_ptr);
        main(mem_ptr, interface::on_mem_read, interface::on_mem_write)
    }
    println!("{:?}", memory)
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

fn compile_asm_and(src: &str) -> Result<()> {
    fs::write("temp/temp.asm", src).unwrap();

    if !Command::new("nasm")
        .arg("-f")
        .arg("win64")
        .arg("temp/temp.asm")
        .arg("-O0")
        .arg("-o")
        .arg("temp/temp.obj")
        .status().expect("Nasm failed to run")
        .success() {
            panic!("Error: Failed to compile assembly");
        }

    if !Command::new("golink")
        .args(["/dll", "/entry", "_DllMain", "temp/temp.obj"])
        .status().expect("Golink failed to run")
        .success() {
            panic!("Error: Failed to link assembly");
        }

    Ok(())
}

type MemoryHandler = unsafe extern "C" fn(mem_space: *mut u8, addr: usize);

type CompiledMain = unsafe extern "C" fn(mem_space: *mut u8, on_mem_read: MemoryHandler, on_mem_write: MemoryHandler);