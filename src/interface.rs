use std::sync::Mutex;

use arrayvec::ArrayString;
use bitvec::{array::BitArray, order::Lsb0};
use once_cell::sync::Lazy;
use rand::{rngs::StdRng, Rng, SeedableRng};

pub type PixelBuffer = [[bool; 32]; 32];

static RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(StdRng::from_entropy()));
static PIXEL_BUFFER: Lazy<Mutex<PixelBuffer>> = Lazy::new(|| Mutex::new([[false; 32]; 32]));
pub static SCREEN_BUFFER: Lazy<Mutex<PixelBuffer>> = Lazy::new(|| Mutex::new([[false; 32]; 32]));
pub static SCREEN_BUFFER_DIRTY: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub static CONTROLLER_INFO: Lazy<Mutex<ControllerInfo>> = Lazy::new(|| Mutex::new(ControllerInfo::default()));
static CHARACTER_BUFFER: Lazy<Mutex<ArrayString<20>>> = Lazy::new(|| Mutex::new(ArrayString::default()));
pub static CHARACTER_DISPLAY: Lazy<Mutex<ArrayString<20>>> = Lazy::new(|| Mutex::new(ArrayString::default()));
pub static CHARACTER_DISPLAY_DIRTY: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub static NUMBER_DISPLAY: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(0));
pub static NUMBER_DISPLAY_DIRTY: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static SHOW_NUMBER_DISPLAY: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static NUMBER_DISPLAY_SETTINGS: Lazy<Mutex<NumberDisplaySettings>> = Lazy::new(|| Mutex::new(NumberDisplaySettings::TwosCompliment));

pub enum NumberDisplaySettings {
    TwosCompliment,
    Unsigned
}

#[derive(Default)]
pub struct ControllerInfo(BitArray<[u8; 1], Lsb0>);

pub unsafe extern "C" fn on_mem_read(mem: *mut u8, addr: usize) {
    println!("Read {addr}");
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
    println!("Wrote to {addr}");
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
            let pixel_buffer = PIXEL_BUFFER.lock().unwrap();
            let mut screen_buffer = SCREEN_BUFFER.lock().unwrap();
            *screen_buffer = *pixel_buffer;
            *SCREEN_BUFFER_DIRTY.lock().unwrap() = true;
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
            let char_buffer = CHARACTER_BUFFER.lock().unwrap();
            let mut char_display = CHARACTER_DISPLAY.lock().unwrap();
            *char_display = *char_buffer;
            *CHARACTER_DISPLAY_DIRTY.lock().unwrap() = true;
        }
        249 => { // Clear character buffer
            let mut char_buffer = CHARACTER_BUFFER.lock().unwrap();
            char_buffer.clear();
        }
        250 => { // Show number display
            *NUMBER_DISPLAY.lock().unwrap() = *mem.add(addr);
            *SHOW_NUMBER_DISPLAY.lock().unwrap() = true;
            *NUMBER_DISPLAY_DIRTY.lock().unwrap() = true;
        }
        251 => { // Clear number display
            *SHOW_NUMBER_DISPLAY.lock().unwrap() = false;
            *NUMBER_DISPLAY_DIRTY.lock().unwrap() = true;
        }
        252 => { // Interpret number as 2s comp [-128, 127]
            *NUMBER_DISPLAY_SETTINGS.lock().unwrap() = NumberDisplaySettings::TwosCompliment;
            *NUMBER_DISPLAY_DIRTY.lock().unwrap() = true;
        }
        253 => { // Interpret number as unsigned int [0, 255]
            *NUMBER_DISPLAY_SETTINGS.lock().unwrap() = NumberDisplaySettings::Unsigned;
            *NUMBER_DISPLAY_DIRTY.lock().unwrap() = true;
        }
        _ => {}
    }
}

unsafe fn get_pixel_coords(mem: *mut u8) -> (usize, usize) {
    let pixel_x = (*mem.offset(240) & 0b11111) as usize;
    let pixel_y = (*mem.offset(241) & 0b11111) as usize;

    (pixel_x, pixel_y)
}