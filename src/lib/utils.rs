#![allow(unused)]
use crate::elf_file::{MyElf, MyFile};
use goblin::elf;
use goblin::{self, elf::Elf};
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process::exit;

pub fn read_file(file_name: &str) -> MyFile {
    let mut file = File::open(file_name).unwrap_or_else(|err| {
        eprintln!("Failed to open file {}: {}", file_name, err);
        exit(1);
    });

    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap_or_else(|err| {
        eprintln!("Failed to read file {}: {}", file_name, err);
        exit(1);
    });

    MyFile {
        file_name: file_name.to_string(),
        ctx: content,
    }
}

fn read_sruct<T>() {}

fn check_magnum(content: &Vec<u8>) -> bool {
    let magic_hdr: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
    if content[0..4] != magic_hdr {
        return false;
    }
    true
}

pub fn deal_target_file(file_name: &str) -> MyElf {
    //read file
    let f = read_file(file_name);
    //check
    if !check_magnum(&f.ctx) {
        eprint!("Not a linkable file\n");
        exit(1);
    }
    MyElf::new(f)
}
