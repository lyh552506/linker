#![allow(unused)]
use goblin;
use std::fs::File;
use std::env;
use std::io::{self, Read};

pub struct Elf{
    fileName:String,
    ctx: Vec<String>,
}


pub fn read_file(file_name: &String)->io::Result<Elf>{
    let mut file=File::open(file_name)?;
    
    let mut content=String::new();
    file.read_to_string(&mut content)?;
    let ctx=vec![content];
    Ok(Elf{
        fileName:file_name.to_string(),
        ctx,
    })
}