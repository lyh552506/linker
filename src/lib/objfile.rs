use bytemuck::NoUninit;
use goblin::elf::section_header;
use itertools::Itertools;
use std::mem::*;

use crate::{
    elf_file::{self, MyElf, MyFile, Shdr, Sym},
    symbol::Symbol,
    utils::{self, read_struct},
};

pub struct ObjFile {
    pub objfile: MyElf,
    pub sym_tab: Vec<u8>,
    pub sym_str_tab: Option<Shdr>,
    pub global_pos: i32,
    pub symbols: Vec<elf_file::Sym>,
    pub is_alive: bool,
    pub local_symbols: Option<Symbol>,
}

impl ObjFile {
    pub fn new(
        obj: MyElf,
        symtab: Vec<u8>,
        sym_str: Shdr,
        pos: i32,
        symbol: Vec<elf_file::Sym>,
        alive: bool,
    ) -> Self {
        ObjFile {
            objfile: obj,
            sym_tab: symtab,
            sym_str_tab: Some(sym_str),
            global_pos: pos,
            symbols: symbol,
            is_alive: alive,
            local_symbols: None,
        }
    }

    pub fn new_null(obj: MyElf, alive: bool) -> Self {
        ObjFile {
            objfile: obj,
            sym_tab: vec![],
            sym_str_tab: None,
            global_pos: -1,
            symbols: vec![],
            is_alive: alive,
            local_symbols: None,
        }
    }

    pub fn initialize_symbols(&self) {
        if let Some(symtab) = self.sym_str_tab {
            assert!(self.global_pos != -1);
			println!("{}",self.objfile.file.file_name);
            let mut local_sym: Vec<Symbol> = Vec::with_capacity(self.global_pos as usize);
            for i in 0..self.global_pos {
                let name_index = self.symbols[i as usize].name;
                let name = elf_file::get_name(&self.sym_tab, name_index as usize);
				println!("{}",name);
				let a=name.as_str();
            }
        } else {
            return;
        }
    }
}
pub fn parse_symtab(f: &MyElf, alive: bool) -> ObjFile {
    if let Some(symtab_hdr) = utils::find_section(&f, section_header::SHT_SYMTAB) {
        //find a symtab header, now get his symbols
        let mut symbols: Vec<Sym> = vec![];
        let mut source = utils::get_target_section_content(&f.file, symtab_hdr);
        let sym_size = size_of::<Sym>();
        let symbols_num = source.len() / sym_size;
        for i in 0..symbols_num {
            symbols.push(read_struct(&source[i * sym_size..].to_vec()));
        }
        //get the firat global symbol's position
        let global_pos = symtab_hdr.Info as i32;
        //get the str section
        let str_sec = f.Sections[symtab_hdr.Link as usize];
        let symtab: Vec<u8> = utils::get_target_section_from_index(&f, symtab_hdr.Link);
        let objfile = ObjFile::new(f.clone(), symtab, symtab_hdr, global_pos, symbols, alive);
        objfile.initialize_symbols();
        objfile
    } else {
        ObjFile::new_null(f.clone(), alive)
    }
}
