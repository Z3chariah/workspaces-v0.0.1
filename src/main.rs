use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read, Write};

use std::process::exit;

#[allow(unused_assignments)]
#[allow(unused)]
fn main() {
    let mut current_workspace: Workspaces = Workspaces::Default;
    let version = String::from("V0.0.1");
    println!(
        "Welcome to Workspaces version {}.\n Press [k] for the Study Space [j] for the game development workspace.",
        version
    );

    while current_workspace != Workspaces::Exit {
        enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
            let keypress = b.unwrap() as char;

            match keypress {
                'k' => current_workspace = Workspaces::Study,
                'j' => current_workspace = Workspaces::Gamedev,
                'l' => current_workspace = Workspaces::Plugins,
                'h' => current_workspace = Workspaces::Default,
                'q' => current_workspace = Workspaces::Exit,
                _ => {
                    disable_raw_mode().unwrap();
                    exit(2);
                }
            }

            if current_workspace == Workspaces::Exit {
                exit(2)
            } else {
                println!("\x1B[2J\x1B[H");
                handleworkspace(&current_workspace);
                continue;
            }
        }
    }
}

#[allow(unused)]
fn handleworkspace(ws: &Workspaces) {
    match ws {
        Workspaces::Study => {
            println!("Press [K] to open Obsidian [J] to open Remnote. and [L] to open Notion");
            io::stdout().flush().unwrap();
            use std::process::Command;

            for b in io::stdin().bytes() {
                let keypress = b.unwrap() as char;

                match keypress {
                    'k' => {
                        Command::new("Open").arg("-a").arg("Obsidian").spawn();
                    }
                    'j' => {
                        Command::new("Open").arg("-a").arg("RemNote").spawn();
                    }

                    'l' => {
                        Command::new("Open").arg("-a").arg("Notion").spawn();
                    }
                    _ => handleworkspace(&Workspaces::Exit),
                }
            }
        }

        Workspaces::Gamedev => {
            println!("Press [K] to open Godot [J] to open Aseprite. and [L] to open Zed");
            use std::process::Command;
            for b in io::stdin().bytes() {
                let keypress = b.unwrap() as char;

                match keypress {
                    'k' => {
                        Command::new("Open").arg("-a").arg("Godot").spawn();
                    }
                    'j' => {
                        Command::new("Open").arg("-a").arg("Aseprite").spawn();
                    }

                    'l' => {
                        Command::new("Open").arg("-a").arg("Zed").spawn();
                    }
                    _ => handleworkspace(&Workspaces::Exit),
                }
            }
        }

        Workspaces::Plugins => println!("inside of the plugins workspace."),
        Workspaces::Default => println!("inside of the default workspace."),
        Workspaces::Exit => {
            exit(1);
        }
    }
}

#[derive(PartialEq)]
enum Workspaces {
    Default,
    Study,
    Plugins,
    Gamedev,
    Exit,
}
