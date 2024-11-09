mod onnx;
mod utils;

use std::io::{self, Write};

#[derive(Debug)]
enum Class {
    CT,
    T,
    Any,
}

impl Default for Class {
    fn default() -> Self {
        Self::Any
    }
}
impl TryFrom<u8> for Class {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Class::CT),
            1 => Ok(Class::T),
            2 => Ok(Class::Any),
            _ => Err("Invalid value for Class"),
        }
    }
}

struct App {
    class: Class,
    capturing: bool,
    onnx: onnx::Onnx,
}

impl App {
    fn new() -> Self {
        Self {
            capturing: false,
            class: Class::default(),
            onnx: onnx::Onnx::new().unwrap(),
        }
    }

    fn display_status(&self) {
        println!("Class: {:?}", self.class);
        println!("Capturing: {}\n", self.capturing);
    }

    fn start_capturing(&mut self) {
        self.capturing = true;
        println!("Started capturing.");
    }

    fn stop_capturing(&mut self) {
        self.capturing = false;
        println!("Ended capturing.");
    }

    fn change_class(&mut self, input: Class) {
        self.class = input;
        println!("Changed class to {:?}", self.class);
    }

    fn handle_input(&mut self) {
        loop {
            utils::clear();
            self.display_status();
            println!("1. Start capturing");
            println!("2. Stop capturing");
            println!("3. Change class: (0 - CT, 1 - T, 2 - ANY)");
            println!("q. Quit");
            print!("Command: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "1" => self.start_capturing(),
                "2" => self.stop_capturing(),
                "3" => {
                    println!("Enter class (0 - CT, 1 - T, 2 - ANY): ");
                    let mut class_input = String::new();
                    io::stdin().read_line(&mut class_input).unwrap();

                    if let Ok(num) = class_input.trim().parse::<u8>() {
                        match Class::try_from(num) {
                            Ok(class) => self.change_class(class),
                            Err(msg) => println!("{msg}"),
                        }
                    } else {
                        println!("Invalid input, please enter a number.");
                    }
                }
                "q" => {
                    if self.capturing {
                        !todo!("Stop windows_capture")
                    }
                    break;
                }
                _ => println!("Invalid command, try again."),
            }
            utils::sleep(1);
        }
    }
}

fn main() {
    let mut app = App::new();
    app.handle_input();
}
