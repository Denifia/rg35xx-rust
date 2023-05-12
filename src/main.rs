use std::fs::{File};
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("/sys/class/power_supply/battery/moto")?;
    println!("We're vibing!");
    file.write_all(b"50")?;
    thread::sleep(Duration::from_millis(1000));
    file.write_all(b"0")?;
    Ok(())
}