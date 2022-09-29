use chrono::{Local};
// for hook
use signal_hook::{iterator::Signals, SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use std::error::Error;
use std::thread;
use std::process::exit;

#[rustfmt::skip]
// https://www.w3.org/TR/xml-entity-names/025.html
const DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ ","  ╻  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┏━┛ ", " ┣━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╹ "],
    ["┗━┛ ","  ╹  "," ┗━━ ", " ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
];

/// hook to trap signals
pub fn hook() -> Result<(), Box<dyn Error>> {
    let signals = Signals::new(&[SIGINT, SIGHUP, SIGTERM, SIGQUIT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            // return the cursor
            print!("\x1b[?25h");
            // exit with the signal
            exit(sig);
        }
    });
    Ok(())
}

fn main() {
    // hide the cursor
    print!("\x1b[2J");
    print!("\x1b[?25l");
    // hook in the signal listener
    let _ = hook();    
    loop {
        let t = Local::now();
        let time = t.format("%H:%M:%S").to_string();
        for row in &DIGITS {
            for c in time.chars() {
                let col = match c {
                    '0'..='9' => c as usize - '0' as usize,
                    ':' => 10,
                    _ => 10,
                };
                print!("{} ", row[col]);
            }
            println!();
        }
        std::thread::sleep(std::time::Duration::from_millis(999));
        print!("\x1b[7A");
    }
}
