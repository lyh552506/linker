#![allow(unused)]
use crate::elf_file::{Ehdr, MyElf, MyFile, Shdr};
use crate::link_info::LinkInfo;
use bytemuck::*;
use goblin::elf;
use goblin::{self, elf::Elf};
use itertools::Itertools;
use std::env::{self, Args};
use std::fmt::format;
use std::fs::*;
use std::io::{self, Read};
use std::path::Path;
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

pub fn check_magnum(content: &Vec<u8>) -> bool {
    let magic_hdr: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
    if content[0..4] != magic_hdr {
        return false;
    }
    true
}

pub fn check_ar(content: &Vec<u8>) -> bool {
    let file_signature: Vec<u8> = b"!<arch>".to_vec();
    if content.starts_with(&file_signature) {
        return true;
    }
    false
}

pub fn read_struct<T: Pod>(src: &Vec<u8>) -> T {
    assert!(
        src.len() >= std::mem::size_of::<T>(),
        "Src byte size must larger than struct"
    );
    let target_struct = &src[0..std::mem::size_of::<T>()];
    bytemuck::from_bytes::<T>(target_struct).clone()
}

pub fn get_elf(file_name: &str) -> MyElf {
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

pub fn parse_args() -> LinkInfo {
    let mut linker = LinkInfo::new();
    let mut args: Vec<String> = env::args().collect();
    let useful_args: Vec<String> = vec![];
    let mut args_num: usize = 0;

    //parse args
    while args.len() > args_num {
        if let Some(val) = get_target_arg(&format!("o"), &args) {
            args.retain(|x| x != &val && x != &format!("-o"));
            linker.output_path = val;
        } else if let Some(val) = get_target_arg(&format!("L"), &args) {
            args.retain(|x| x != &val);
            let path = val[2..].to_string();
            linker.library_path.push(path);
        } else if let Some(val) = get_target_arg(&format!("l"), &args) {
            if let Some(file_path) = find_ar_file(&val[2..].to_string(), &linker) {
                args.retain(|x| x != &val);
				linker.analysis_ar(read_file(&val));
            }
        } else {
            args_num += 1;
        }
    }
    //parse obj file
    for remain in &args {
		// println!("Name:{}", remain);
        if remain.ends_with(".o") {
            linker.append_obj(read_file(remain));
        }
    }
    linker
}

pub fn get_target_arg(name: &String, args: &Vec<String>) -> Option<String> {
    let mut flag: Vec<String> = vec![];
    if name.len() == 1 && name != &format!("*") {
        flag = vec![format!("-{}", name)];
    } else if name.len() == 1 {
        flag = vec![format!("-")];
    } else {
        flag = vec![format!("-{}", name), format!("--{}", name)];
    }
    for (ind, arg_2) in args.iter().enumerate() {
        if flag.contains(&arg_2) {
            return Some(args[ind + 1].to_string());
        }
        //capture args like -L....
        for (_, n) in flag.iter().enumerate() {
            if arg_2.starts_with(n) {
                return Some(arg_2.to_string());
            }
        }
    }
    None
}

pub fn find_ar_file(name: &String, linkinfo: &LinkInfo) -> Option<String> {
    let file_name = format!("lib{}.a", name);
    for lib_path in &linkinfo.library_path {
        let path = Path::new(lib_path).join(&file_name);
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
    }
    None
}
