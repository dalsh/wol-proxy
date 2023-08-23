use std::io::Error;
use std::process::exit;

use std::thread;
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;

pub fn start() -> Result<(), Error> {
    let mut signals = Signals::new([SIGINT, SIGTERM])?;

    thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                SIGINT | SIGTERM => {
                    println!("Terminating ...");
                    exit(0)
                }
                _ => {}
            }
        }
    });

    Ok(())
}
