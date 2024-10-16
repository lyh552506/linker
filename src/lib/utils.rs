#![allow(unused)]
use crate::elf_file::{Ehdr, MyElf, MyFile, Shdr};
use bytemuck::*;
use goblin::elf;
use goblin::{self, elf::Elf};
use itertools::Itertools;
use std::env::{self, Args};
use std::fs::File;
use std::io::{self, Read};
use std::process::exit;
use std::ptr::null;

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

fn check_magnum(content: &Vec<u8>) -> bool {
    let magic_hdr: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
    if content[0..4] != magic_hdr {
        return false;
    }
    true
}

pub fn read_struct<T: Pod>(src: &Vec<u8>) -> T {
    assert!(
        src.len() >= std::mem::size_of::<T>(),
        "Src byte size must larger than struct"
    );
    let target_struct = &src[0..std::mem::size_of::<T>()];
    bytemuck::from_bytes::<T>(target_struct).clone()
}

pub fn GetElf(file_name: &str) -> MyElf {
    //read file
    let f = read_file(file_name);
    //check
    if !check_magnum(&f.ctx) {
        eprint!("Not a linkable file\n");
        exit(1);
    }
    //get elh_header
    let ehdr: Ehdr = read_struct(&f.ctx);
    //get all sec_header
    let mut curr_ctx = &f.ctx[ehdr.ShOff as usize..].to_vec();
    let shdr: Shdr = read_struct(&curr_ctx);
    //get the number of section headers
    let mut section_num = ehdr.ShNum as i64;
    if section_num == 0 {
        section_num = shdr.Size as i64;
    }

    let mut sections: Vec<Shdr> = Vec::new();
    sections.push(shdr);
    let section_size = std::mem::size_of::<Shdr>();
    let mut tmp = Vec::new();
    for i in 1..section_num {
        tmp = curr_ctx[section_size..].to_vec();
        curr_ctx = &tmp;
        let tmp_shdr = read_struct(&curr_ctx);
        sections.push(tmp_shdr);
    }
    // create my_elf
    MyElf::new(f, ehdr, sections)
}

pub fn parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut capture = "".to_string();
    let useful_args: Vec<String> = vec![];

    let mut get_target_arg = |name: &String| {
        let mut flag: Vec<String> = vec![];
        if name.len() == 1 {
            flag = vec!["-".to_string() + name];
        } else {
            flag = vec!["-".to_string() + name, "--".to_string() + name];
        }
        for (arg_1, arg_2) in args.iter().tuple_windows() {
            if flag.contains(&arg_1) {
                // return Some(arg_2);
				capture=arg_2.to_string();
            }
        }
        capture="".to_string();
    };

    get_target_arg(&"o".to_string());

    useful_args
}
