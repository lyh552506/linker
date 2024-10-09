use std::{
    env::{self, Args},
    process::exit,
};

use my_project_lib::utils;

fn main() {
    let Args: Vec<String> = env::args().collect();
    if Args.len() < 2 {
        eprintln!("Missing args!");
        exit(0);
    }
    utils::read_file(&Args[1]);
    println!("Hello, world!");
}
