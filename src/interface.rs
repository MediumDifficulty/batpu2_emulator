use std::{mem, sync::Mutex};

use arrayvec::ArrayString;
use bitvec::{array::BitArray, order::Lsb0};
use once_cell::sync::Lazy;
use rand::{rngs::StdRng, Rng, SeedableRng};

static RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::from_entropy()));
static PIXEL_BUFFER: Lazy<Mutex<[[bool; 32]; 32]>> = Lazy::new(|| Mutex::new([[false; 32]; 32]));
static CONTROLLER_INFO: Lazy<Mutex<ControllerInfo>> = Lazy::new(|| Mutex::new(ControllerInfo::default()));
static CHARACTER_BUFFER: Lazy<Mutex<ArrayString<20>>> = Lazy::new(|| Mutex::new(ArrayString::default()));

#[derive(Default)]
struct ControllerInfo(BitArray<[u8; 1], Lsb0>);

pub unsafe extern "C" fn on_mem_read(mem: *mut u8, addr: usize) {
    match addr {
        244 => { // Load Pixel at (Pixel X, Pixel Y)
            let (pixel_x, pixel_y) = get_pixel_coords(mem);

            let pixel_buffer = PIXEL_BUFFER.lock().unwrap();

            *mem.add(addr) = pixel_buffer[pixel_y][pixel_x] as u8;
        }
        254 => { // Load a random 8 bit number
            let mut rng = RNG.lock().unwrap();
            *mem.add(addr) = rng.gen::<u8>();
        }
        255 => { // Load controller info
            let info = CONTROLLER_INFO.lock().unwrap();
            *mem.add(addr) = info.0.data[0];
        }
        _ => {}
    }
}

pub unsafe extern "C" fn on_mem_write(mem: *mut u8, addr: usize) {
    println!("{addr}");
    match addr {
        242 => { // Draw pixel at (Pixel X, Pixel Y) to buffer
            let (pixel_x, pixel_y) = get_pixel_coords(mem);

            let mut pixel_buffer = PIXEL_BUFFER.lock().unwrap();
            pixel_buffer[pixel_y][pixel_x] = true;
        }
        243 => { // Clear pixel at (Pixel X, Pixel Y) to buffer
            let (pixel_x, pixel_y) = get_pixel_coords(mem);

            let mut pixel_buffer = PIXEL_BUFFER.lock().unwrap();
            pixel_buffer[pixel_y][pixel_x] = false;
        }
        245 => { // Push screen buffer

        }
        246 => { // Clear screen buffer
            let mut pixel_buffer = PIXEL_BUFFER.lock().unwrap();
            *pixel_buffer = [[false; 32]; 32];
        }
        247 => { // Write character to buffer
            let mut char_buffer = CHARACTER_BUFFER.lock().unwrap();
            char_buffer.push(*mem.add(addr) as char);
        }
        248 => { // Push character buffer

        }
        249 => { // Clear character buffer
            let mut char_buffer = CHARACTER_BUFFER.lock().unwrap();
            char_buffer.clear();
        }
        250 => { // Show number display

        }
        251 => { // Clear number display

        }
        252 => { // Interpret number as 2s comp [-128, 127]

        }
        253 => { // Interpret number as unsigned int [0, 255]

        }
        _ => {}
    }
}

unsafe fn get_pixel_coords(mem: *mut u8) -> (usize, usize) {
    let pixel_x = (*mem.offset(240) & 0b11111) as usize;
    let pixel_y = (*mem.offset(241) & 0b11111) as usize;

    (pixel_x, pixel_y)
}