use std::{
    io::{self, Write},
    process::Command,
    thread,
    time::Duration,
};

pub fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg("cls")
            .spawn()
            .expect("Failed to clear the terminal");
    } else {
        print!("\x1B[2J\x1B[H");
    }
    io::stdout().flush().unwrap();
}

pub fn sleep(seconds: u64) {
    thread::sleep(Duration::new(seconds, 0))
}
