use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::{
    io::{self, Read},
    process::exit,
};

#[derive(Debug)]
struct Components {
    title: String,
    description: String,
    tooltip: String,
}

fn main() {
    let mut current_workspace: Workspaces = Workspaces::Default;
    let menu_items = vec![
        Components {
            title: String::from("Enter home workspace."),
            description: String::from(
                "A workspace which allows you to run various terminal commands and start files!",
            ),
            tooltip: String::from(""),
        },
        Components {
            title: String::from("Enter home workspace."),
            description: String::from("Enter home workspace."),
            tooltip: String::from(""),
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
                _ => disable_raw_mode().unwrap(),
            }
            print!("\x1B[2J\x1B[H");
            handleworkspace(&current_workspace);

            if current_workspace == Workspaces::Exit {
                break;
            }
        }
    }
}

fn print_component(_c: &Components) {
    print!("{} {} {}\n", _c.title, _c.description, _c.tooltip);
}

fn handleworkspace(ws: &Workspaces) {
    match ws {
        Workspaces::Study => print!("inside of the study workspace\n"),
        Workspaces::Gamedev => print!("inside of the gamedev workspace\n"),
        Workspaces::Plugins => print!("inside of the plugins workspace\n"),
        Workspaces::Default => print!("inside of the default workspace\n"),
        Workspaces::Exit => exit(1),
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
