use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read, Write};
use std::process::exit;
#[derive(Debug)]
struct Components {
    title: String,
    description: String,
}
#[allow(unused_assignments)]
#[allow(unused)]
fn main() {
    let mut current_workspace: Workspaces = Workspaces::Default;
    let menu_items = vec![
        Components {
            title: String::from("Enter home workspace."),
            description: String::from(
                "A workspace which allows you to run various terminal commands and start files!",
            ),
        },
        Components {
            title: String::from("Enter home workspace."),
            description: String::from("Enter home workspace."),
        },
    ];

    for item in &menu_items {
        print_component(&item);
    }

    loop {
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
            print!("\x1B[2J\x1B[H");
            handleworkspace(&current_workspace);

            if current_workspace != Workspaces::Exit {
                print!("\x1B[2J\x1B[H");
                handleworkspace(&current_workspace);
                continue;
            } else {
                disable_raw_mode().unwrap()
            }
        }
    }
}

fn print_component(_c: &Components) {
    println!("{} {}", _c.title, _c.description);
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
                    _ => {
                        exit(1);
                    }
                }
            }
        }

        Workspaces::Gamedev => println!("insid of the game development workspace"),
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
