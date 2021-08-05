use std::fs;
use std::fs::File;
use std::io::prelude::*;
use chrono::Local;

pub fn hello(query: &str) {
    println!("Hello {}", query);
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

pub fn delete(path: &str) -> std::io::Result<()> {
    fs::remove_file(&path)?;
    println!("Sucessfully deleted {}", path);
    Ok(())
}

pub fn reveal() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}

pub fn date() {
    let local_time = Local::now();
    println!("{}", local_time);
}