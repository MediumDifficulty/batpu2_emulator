use std::{io::stdout, time::{Duration, Instant}};

use crossterm::{event, terminal::{disable_raw_mode, enable_raw_mode}};

use crate::interface::PixelBuffer;

pub fn ui_main() {
    enable_raw_mode().unwrap();

    let tick_interval = Duration::from_millis(50);

    loop {
        if let Ok(event) = event::poll(tick_interval) {
            if event {
                let event = event::read().unwrap();
                println!("{:?}", event);

                match event {
                    event::Event::Key(key_event) => {
                        if key_event.code == event::KeyCode::Esc {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode().unwrap();

    println!("Exiting");
}
