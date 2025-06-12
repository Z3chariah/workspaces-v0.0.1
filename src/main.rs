// use crossterm::Command;
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

            let mut ws: workspaces = match keypress {
                'k' => workspaces::Study,
                'j' => workspaces::Gamedev,
                'l' => workspaces::Plugins,
                'h' => workspaces::Default,
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

#[allow(non_camel_case_types)]
enum workspaces {
    Default,
    Study,
    Plugins,
    Gamedev,
}

#[allow(unused)]
impl workspaces {
    fn state_machine(&self) {
        match self {
            workspaces::Study => {
                println!("I am in the Study Workspace")
            }
            workspaces::Plugins => {
                println!("I am in the Plugins Workspace")
            }
            workspaces::Default => {
                println!("I am in the Default Workspace")
            }
            workspaces::Gamedev => {
                println!("I am in the Game Developer Workspace")
            }
            _ => exit(0),
        };
    }
}
