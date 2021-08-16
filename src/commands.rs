#![allow(non_camel_case_types)]
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use chrono::Local;


pub fn hello(query: &str) {
    println!("Hello {}", query);
    // name: String::from("greet"), desc: String::from("Greets the name of the person")
}

pub fn no_cmd() {
    println!("Invalid command please try again");
}

pub fn math(num1: &u128,num2: &u128,op: &str) {
    match op {
        "+" => println!("{}", num1 + num2),
        "-" => println!("{}", num1 - num2),
        "*" => println!("{}", num1 * num2),
        "/" => println!("{}", num1 / num2),
        "**" => println!("{}", num1 ** num2),
        &_ => println!("An Error occured because you are a useless person and don't understand that you cannot add words :[")
    }
}

pub fn create(file: &std::path::Path , text: &str, display: std::path::Display) {
    let mut file = match File::create(&file) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}


pub fn delete(path: &str) -> std::io::Result<()>{
    fs::remove_file(&path).expect("Something went wrong");
    println!("Sucessfully deleted {}", path);
    Ok(())
}

pub fn reveal() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}

pub fn date() {
    let local_time = Local::now();
    println!("{}", local_time);
}


pub fn help() {
    println!("All commands: ");
    println!("  greet   Greets the name of the person specified");
    println!("  math    Do simple 2 number mathematics");
    println!("  create  Create a file in the current directory with some text to be inserted in it");
    println!("  reveal  Shows all the files and subdirectories in the current directory");
    println!("  delete  Delete the specified file in the current directory");
    println!("  date    Show the local date and time");
    println!("  --help  Shows this message")
}
