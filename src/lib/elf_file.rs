use crate::{objfile::ObjFile, symbol::Symbol};
use bytemuck::{Pod, Zeroable};
use goblin::{
    self,
    elf::{self, section_header::SHN_UNDEF, Elf},
    strtab,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
#[derive(Clone)]
pub struct MyFile {
    pub file_name: String,
    pub ctx: Vec<u8>,
}

impl MyFile {
    pub fn new(name: String, content: Vec<u8>) -> MyFile {
        MyFile {
            file_name: name,
            ctx: content,
        }
    }
}
#[derive(Clone)]
pub struct MyElf {
    pub file: MyFile,
    pub ElfHdr: Ehdr,
    pub Sections: Vec<Shdr>,
    pub symbol_map: HashMap<String, Rc<RefCell<Symbol>>>,
}

impl MyElf {
    pub fn new(f: MyFile, e: Ehdr, Sec: Vec<Shdr>) -> MyElf {
        let map = HashMap::new();
        MyElf {
            file: f,
            ElfHdr: e,
            Sections: Sec,
            symbol_map: map,
        }
    }

    pub fn GetCorrespondSym(&mut self, name: &String) -> Rc<RefCell<Symbol>> {
        if let Some(name) = self.symbol_map.get(name) {
            return Rc::clone(name);
        }
        let sym = Rc::new(RefCell::new(Symbol::new_null(name.to_string())));
        self.symbol_map.insert(name.to_string(), Rc::clone(&sym));
        sym
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ehdr {
    pub Ident: [u8; 16],
    pub Type: u16,
    pub Machine: u16,
    pub Version: u32,
    pub Entry: u64,
    pub PhOff: u64,
    pub ShOff: u64,
    pub Flags: u32,
    pub EhSize: u16,
    pub PhEntSize: u16,
    pub PhNum: u16,
    pub ShEntSize: u16,
    pub ShNum: u16,
    pub ShStrndx: u16,
}
unsafe impl Zeroable for Ehdr {}
unsafe impl Pod for Ehdr {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Shdr {
    pub Name: u32,
    pub Type: u32,
    pub Flags: u64,
    pub Addr: u64,
    pub Offset: u64,
    pub Size: u64,
    pub Link: u32,
    pub Info: u32,
    pub AddrAlign: u64,
    pub EntSize: u64,
}
unsafe impl Zeroable for Shdr {}
unsafe impl Pod for Shdr {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Sym {
    pub name: u32,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
    pub val: u64,
    pub size: u64,
}
unsafe impl Zeroable for Sym {}
unsafe impl Pod for Sym {}

impl Sym {
    pub fn is_undef(&self) -> bool {
        self.shndx == elf::section_header::SHN_UNDEF as u16
    }

    pub fn is_abs(&self) -> bool {
        self.shndx == elf::section_header::SHN_ABS as u16
    }
}

pub fn get_name(strtab: &Vec<u8>, offset: usize) -> String {
    let res = match std::str::from_utf8(&strtab) {
        Ok(v) => Some(v),
        Err(_) => None,
    };
    let s = res.unwrap();
    let res = s[offset..].find("\0").map(|pos| offset + pos);
    let end = res.unwrap();
    s[offset..end].to_string()
}
