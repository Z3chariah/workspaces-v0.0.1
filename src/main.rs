use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read, Write};
use std::process::exit;
#[derive(Debug)]
struct Components {
    title: String,
    description: String,
}
#[allow(unused_assignments)]
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
            } else {
                disable_raw_mode();
                break;
            }
        }
    }
}

fn print_component(_c: &Components) {
    println!("{} {}", _c.title, _c.description);
}

fn handleworkspace(ws: &Workspaces) {
    match ws {
        Workspaces::Study => {
            println!(r#"Press [K] to open Obsidian an Remnote."#);
            io::stdout().flush().unwrap();
        }

        Workspaces::Gamedev => println!("inside of the gamedev workspace."),
        Workspaces::Plugins => println!("inside of the plugins workspace."),
        Workspaces::Default => println!("inside of the default workspace."),
        Workspaces::Exit => {
            disable_raw_mode().unwrap();
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
