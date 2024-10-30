#![allow(unused)]
use crate::elf_file::{Ehdr, MyElf, MyFile, Shdr};
use crate::link_info::LinkInfo;
use crate::{objfile, WorkSpaceFolderPlace};
use bytemuck::*;
use goblin::elf::{self, section_header};
use goblin::{self, elf::Elf};
use itertools::Itertools;
use std::env::{self, Args};
use std::fmt::format;
use std::fs::*;
use std::io::{self, Read};
use std::path::Path;
use std::process::exit;
use std::ptr::null;

pub fn read_file(file_name: &str) -> Option<MyFile> {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(err) => {
            return None;
        }
    };
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap_or_else(|err| {
        eprintln!("Failed to read file {}: {}", file_name, err);
        exit(1);
    });

    Some(MyFile {
        file_name: file_name.to_string(),
        ctx: content,
    })
}

pub fn check_magnum(content: &Vec<u8>) -> bool {
    let magic_hdr: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
    if content[0..4] != magic_hdr {
        return false;
    }
    true
}

pub fn check_ar(content: &Vec<u8>) -> bool {
    let file_signature: Vec<u8> = b"!<arch>\n".to_vec();
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

pub fn get_elf(f: MyFile) -> MyElf {
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
    // let mut args: Vec<String> = env::args().collect();
    let mut args: Vec<String> = vec![
        "./ld".to_string(),
        "-plugin".to_string(),
        "/usr/lib/gcc-cross/riscv64-linux-gnu/11/liblto_plugin.so".to_string(),
        "-plugin-opt=/usr/lib/gcc-cross/riscv64-linux-gnu/11/lto-wrapper".to_string(),
        "-plugin-opt=-fresolution=/tmp/cciA4DXw.res".to_string(),
        "-plugin-opt=-pass-through=-lgcc".to_string(),
        "-plugin-opt=-pass-through=-lgcc_eh".to_string(),
        "-plugin-opt=-pass-through=-lc".to_string(),
        "--sysroot=/".to_string(),
        "--build-id".to_string(),
        "-hash-style=gnu".to_string(),
        "--as-needed".to_string(),
        "-melf64lriscv".to_string(),
        "-static".to_string(),
        "-z".to_string(),
        "relro".to_string(),
        "-o".to_string(),
        "out/hello.out".to_string(),
        "/usr/lib/gcc-cross/riscv64-linux-gnu/11/../../../../riscv64-linux-gnu/lib/crt1.o"
            .to_string(),
        "/usr/lib/gcc-cross/riscv64-linux-gnu/11/crti.o".to_string(),
        "/usr/lib/gcc-cross/riscv64-linux-gnu/11/crtbeginT.o".to_string(),
        "-L.".to_string(),
        "-L/usr/lib/gcc-cross/riscv64-linux-gnu/11".to_string(),
        "-L/usr/lib/gcc-cross/riscv64-linux-gnu/11/../../../../riscv64-linux-gnu/lib".to_string(),
        "-L/lib/riscv64-linux-gnu".to_string(),
        "-L/usr/lib/riscv64-linux-gnu".to_string(),
        "out/hello.o".to_string(),
        "--start-group".to_string(),
        "-lgcc".to_string(),
        "-lgcc_eh".to_string(),
        "-lc".to_string(),
        "--end-group".to_string(),
        "/usr/lib/gcc-cross/riscv64-linux-gnu/11/crtend.o".to_string(),
        "/usr/lib/gcc-cross/riscv64-linux-gnu/11/crtn.o".to_string(),
    ];
    let mut object_file: Vec<MyElf> = vec![];
    let useful_args: Vec<String> = vec![];
    let mut args_num: usize = 0;
    let mut alive = 0;
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
            //deal ar file
            if let Some(file_path) = find_ar_file(&val[2..].to_string(), &linker) {
                args.retain(|x| x != &val);
                let mut objs = vec![];
                if let Some(f) = read_file(&file_path) {
                    objs = linker.analysis_ar(f);
                }
                object_file.append(&mut objs);
            }
        } else {
            args_num += 1;
        }
    }
    alive = object_file.len();
    //parse obj file (Alive)
    for remain in &args {
        if remain.ends_with(".o") {
            if let Some(f) = read_file(&remain) {
                object_file.push(get_elf(f));
            }
        }
    }
    for (index, obj) in object_file.iter_mut().enumerate() {
        linker
            .object_file
            .push(objfile::parse_symtab(obj, alive <= index));
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

pub fn from_u8_to_str(_str: &Vec<u8>) -> String {
    std::str::from_utf8(&_str[..])
        .expect("Invalid UTF-8")
        .trim_end_matches(char::from(0))
        .to_string()
}

pub fn find_section(elf: &MyElf, section_flag: u32) -> Option<Shdr> {
    for (index, section) in elf.Sections.iter().enumerate() {
        if section.Type == section_flag {
            return Some(elf.Sections[index]);
        }
    }
    None
}

pub fn get_target_section_content(file: &MyFile, section: Shdr) -> Vec<u8> {
    let start = section.Offset as usize;
    let end = (section.Offset + section.Size) as usize;
    assert!(end <= file.ctx.len());
    file.ctx[start..end].to_vec()
}

pub fn get_target_section_from_index(elf: &MyElf, section_index: u32) -> Vec<u8> {
    return get_target_section_content(&elf.file, elf.Sections[section_index as usize]);
}
