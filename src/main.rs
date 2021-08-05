// use std::fs;
// use std::fs::File;
// use std::io::prelude::*;
use std::path::Path;
use std::env;
mod commands;

// let r = fs::remove_file(filepath);

fn main() {
    match try_main() {
        Some(_) => {}, //everything went well
        None => {
            println!("Something went wrong");
        }
    }
}

fn try_main() -> Option<()> {
    let args: Vec<String> = env::args().collect();

    let command: &String = args.get(1)?;


    let lmfao: &str = command;
    match lmfao {
        "create" => {
            let file_name = format!("{}", args.get(2)?);
            let path = Path::new(&file_name);
            let text: &str = args.get(3)?;
            let display = path.display();
            commands::create(path, text, display);
        },
        "math" => {
            let num1: &u128 = &args.get(2)?.parse::<u128>().ok()?;
            let num2: &u128 = &args.get(3)?.parse::<u128>().ok()?;
            let op: &str = &args.get(4)?;
            commands::math(num1, num2, op);
        },
        "greet" => {
            let query: &String = args.get(2)?;
            commands::hello(query);
        },
        "delete" => {
            let file_name: &str = args.get(2)?;
            commands::delete(file_name).ok()?;
        },
        "reveal" => {
            commands::reveal();
        }
        "date" => {
            commands::date();
        }
        &_ => commands::no_cmd()
    }
    Some(())
}