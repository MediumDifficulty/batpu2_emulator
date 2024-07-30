use std::{io::stdout, time::Duration};

use crossterm::{cursor, event, execute, style::{self, Color, Print}, terminal::{self, disable_raw_mode, enable_raw_mode}};

use crate::interface::{self, NumberDisplaySettings, PixelBuffer, NUMBER_DISPLAY, NUMBER_DISPLAY_SETTINGS};

type CharPos = (u16, u16);

pub fn ui_main() {
    enable_raw_mode().unwrap();

    let tick_interval = Duration::from_millis(50);
    
    let mut w = stdout();

    let mut origin = (0u16, terminal::size().unwrap().1 - 34);

    execute!(
        w,
        cursor::Hide,
        cursor::MoveTo(0, origin.1),
        terminal::Clear(terminal::ClearType::FromCursorDown),
        Print(include_str!("ui/framing.txt"))
    ).unwrap();

    loop {
        if let Ok(event) = event::poll(tick_interval) {
            if event {
                let event = event::read().unwrap();
                // println!("{:?}", event);

                match event {
                    event::Event::Key(key_event) => {
                        if key_event.code == event::KeyCode::Esc {
                            break;
                        }
                    }
                    event::Event::Resize(_, y) => {
                        if y >= 34 {
                            origin.1 = y - 34;
                        }
                    }
                    _ => {}
                }
            }
        }
        
        if *interface::NUMBER_DISPLAY_DIRTY.lock().unwrap() {
            draw_number_display(origin, *NUMBER_DISPLAY.lock().unwrap(), *NUMBER_DISPLAY_SETTINGS.lock().unwrap());
            *interface::NUMBER_DISPLAY_DIRTY.lock().unwrap() = false;
        }
        if *interface::CHARACTER_DISPLAY_DIRTY.lock().unwrap() {
            draw_text_display(origin, &interface::CHARACTER_DISPLAY.lock().unwrap());
            *interface::CHARACTER_DISPLAY_DIRTY.lock().unwrap() = false;
        }
        if *interface::SCREEN_BUFFER_DIRTY.lock().unwrap() {
            draw_screen(origin, &interface::SCREEN_BUFFER.lock().unwrap());
            *interface::SCREEN_BUFFER_DIRTY.lock().unwrap() = false;
        }
    }

    execute!(w,
        style::ResetColor,
        cursor::Show,
        cursor::MoveTo(origin.0, origin.1),
        terminal::Clear(terminal::ClearType::FromCursorDown)
    ).unwrap();

    disable_raw_mode().unwrap();
}

const NUMBER_DISPLAY_POS: CharPos = (75, 1);

fn draw_number_display(origin: CharPos, value: u8, settings: NumberDisplaySettings) {
    let mut negative = false;
    let value = match settings {
        NumberDisplaySettings::TwosCompliment => {
            let signed = value as i8;
            negative = signed < 0;
            u8_to_decimal_array(signed.unsigned_abs())
        },
        NumberDisplaySettings::Unsigned => u8_to_decimal_array(value),
    };

    if negative {
        execute!(stdout(), cursor::MoveTo(NUMBER_DISPLAY_POS.0 - 1, NUMBER_DISPLAY_POS.1 + 1), Print("-")).unwrap();
    } else {
        execute!(stdout(), cursor::MoveTo(NUMBER_DISPLAY_POS.0 - 1, NUMBER_DISPLAY_POS.1 + 1), Print(" ")).unwrap();
    }

    const DIGIT_SHEET: &str = include_str!("ui/numbers.txt");
    let lines = DIGIT_SHEET.lines().collect::<Vec<_>>();

    for (i, digit) in value.iter().enumerate() {
        let x = NUMBER_DISPLAY_POS.0 + i as u16 * 5;
        let digit_offset = *digit as usize * 3;

        for y in 0..3 {
            let line = lines[digit_offset + y];
            // println!("{:?}", origin);
            execute!(stdout(), cursor::MoveTo(x, origin.1 + NUMBER_DISPLAY_POS.1 + y as u16), Print(line)).unwrap();
        }
    }
}

fn u8_to_decimal_array(mut n: u8) -> [u8; 3] {
    let mut digits =[0; 3];

    if n == 0 {
        return digits;
    }

    let mut i = 0;
    while n > 0 {
        digits[i] = n % 10;
        n /= 10;
        i += 1;
    }

    digits.reverse();
    digits
}

const TEXT_DISPLAY_POS: CharPos = (72, 8);

fn draw_text_display(origin: CharPos, data: &str) {
    execute!(
        stdout(),
        cursor::MoveTo(origin.0 + TEXT_DISPLAY_POS.0, origin.1 + TEXT_DISPLAY_POS.1),
        Print(format!("{data: <20}"))
    ).unwrap();
}

const SCREEN_POS: CharPos = (1, 1);

fn draw_screen(origin: CharPos, data: &PixelBuffer) {
    for y in 0..32 {
        for x in 0..32 {
            let pixel = data[31 - y][x];
            let x = origin.0 + SCREEN_POS.0 + x as u16 * 2;
            let y = origin.1 + SCREEN_POS.1 + y as u16;
            
            execute!(
                stdout(),
                cursor::MoveTo(x, y),
                style::SetBackgroundColor(if pixel {
                    Color::White
                } else {
                    Color::DarkGrey
                }),
                Print("  ")
            ).unwrap();
        }
    }

    execute!(
        stdout(),
        style::ResetColor,
    ).unwrap();
}