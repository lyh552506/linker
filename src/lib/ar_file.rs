use std::process::exit;

use crate::elf_file::MyFile;
use crate::objfile::ObjFile;
use crate::utils::*;
use bytemuck::{Pod, Zeroable};
use goblin::{self, elf::Elf};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Arhdr {
    pub name: [u8; 16],
    pub date: [u8; 12],
    pub uid: [u8; 6],
    pub gid: [u8; 6],
    pub mode: [u8; 8],
    pub size: [u8; 10],
    pub fmag: [u8; 2],
}
unsafe impl Zeroable for Arhdr {}
unsafe impl Pod for Arhdr {}

impl Arhdr {
    pub fn is_str_tab(&self) -> bool {
        let nm = from_u8_to_str(&self.name[..].to_vec());
        // println!("Ar header name: {}", nm);
        if nm.starts_with("//") {
            return true;
        }
        false
    }

    pub fn is_symtab(&self) -> bool {
        let nm = from_u8_to_str(&self.name[..].to_vec());
        if nm.starts_with("/") || nm.starts_with("/SYM64/") {
            return true;
        }
        false
    }

    pub fn get_name(&self, strtab: &Vec<u8>) -> String {
        let nm = from_u8_to_str(&self.name[..].to_vec());
        if nm.starts_with("/") {
            let start = std::str::from_utf8(&self.name[1..])
                .expect("Not utf8")
                .parse::<usize>()
                .expect("Fail");
            let target_strtab = &strtab[start..];
            if let Some(end) = target_strtab.iter().position(|&x| x == b'/') {
                return from_u8_to_str(&strtab[start..start + end].to_vec());
            }
            exit(1);
        }

        let end = match self.name.iter().position(|&x| x == b'/') {
            Some(ind) => ind,
            None => {
                exit(0);
            }
        };

        from_u8_to_str(&self.name[..end].to_vec())
    }
}
