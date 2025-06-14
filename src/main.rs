use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};
use std::process::exit;

#[allow(unused_assignments)]
#[allow(unused)]
fn main() {
    loop {
        enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
            let keypress = b.unwrap() as char;

            let mut ws: WorkSpaces = match keypress {
                'k' => WorkSpaces::Study,
                'j' => WorkSpaces::Gamedev,
                'l' => WorkSpaces::Plugins,
                'h' => WorkSpaces::Default,
                _ => {
                    disable_raw_mode().unwrap();
                    exit(1)
                }
            };
            print!("\x1B[2J\x1B[H");
            ws.state_machine();
        }
    }
}

enum WorkSpaces {
    Default,
    Study,
    Plugins,
    Gamedev,
}

#[allow(unused)]
impl WorkSpaces {
    fn state_machine(&self) {
        match self {
            WorkSpaces::Study => {
                println!("I am in the Study Workspace")
            }
            WorkSpaces::Plugins => {
                println!("I am in the Plugins Workspace")
            }
            WorkSpaces::Default => {
                println!("I am in the Default Workspace")
            }
            WorkSpaces::Gamedev => {
                println!("I am in the Game Developer Workspace")
            }
            _ => exit(0),
        };
    }
}
