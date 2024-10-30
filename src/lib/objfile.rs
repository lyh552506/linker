use bytemuck::NoUninit;
use goblin::elf::section_header;
use itertools::Itertools;
use std::{mem::*, rc::Rc};

use crate::{
    elf_file::{self, MyElf, MyFile, Shdr, Sym},
    symbol::Symbol,
    utils::{self, read_struct},
};

#[derive(Clone)]
pub struct ObjFile {
    pub objfile: MyElf,
    pub sym_tab: Vec<u8>,
    pub sym_str_tab: Option<Shdr>,
    pub global_pos: i32,
    pub symbols: Vec<elf_file::Sym>,
    pub is_alive: bool,
    pub local_symbols: Vec<Rc<Symbol>>,
    pub global_symbols: Vec<Rc<Symbol>>,
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
            local_symbols: vec![],
            global_symbols: vec![],
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
            local_symbols: vec![],
            global_symbols: vec![],
        }
    }

    pub fn update_local_symbols(&mut self, local_sym: Vec<Rc<Symbol>>) {
        self.local_symbols = local_sym;
    }

    pub fn update_global_symbols(&mut self, global_sym: Vec<Rc<Symbol>>) {
        self.global_symbols = global_sym;
    }

    pub fn initialize_symbols(&mut self,elf:&mut MyElf) {
        if let Some(symtab) = self.sym_str_tab {
            assert!(self.global_pos != -1);
            println!("{}", self.objfile.file.file_name);
            let mut local_sym: Vec<Rc<Symbol>> = Vec::with_capacity(self.global_pos as usize);
            let mut global_sym: Vec<Rc<Symbol>> =
                Vec::with_capacity(self.symbols.len() - self.global_pos as usize);
            for i in 0..self.global_pos {
                let name_index = self.symbols[i as usize].name;
                let val = self.symbols[i as usize].val;
                let o = self.clone();
                let name = elf_file::get_name(&self.sym_tab, name_index as usize);
                let sym = Symbol::new(Box::new(o), name, val, i as i32);
                local_sym.push(Rc::new(sym));
            }
            self.update_local_symbols(local_sym);

            //update global
            for i in self.global_pos..self.symbols.len() as i32 {
                let name_index = self.symbols[i as usize].name;
                let name = elf_file::get_name(&self.sym_tab, name_index as usize);
                let sym = elf.GetCorrespondSym(&name); //make sure only has one symbol
                global_sym.push(sym);
            }

            self.update_global_symbols(global_sym);
        } else {
            return;
        }
    }
}
pub fn parse_symtab(f: &mut MyElf, alive: bool) -> ObjFile {
    if let Some(symtab_hdr) = utils::find_section(&f, section_header::SHT_SYMTAB) {
        //find a symtab header, now get his symbols
        let mut symbols: Vec<elf_file::Sym> = vec![];
        let mut source = utils::get_target_section_content(&f.file, symtab_hdr);
        let sym_size = size_of::<elf_file::Sym>();
        let symbols_num = source.len() / sym_size;
        for i in 0..symbols_num {
            symbols.push(read_struct(&source[i * sym_size..].to_vec()));
        }
        //get the firat global symbol's position
        let global_pos = symtab_hdr.Info as i32;
        //get the str section
        let symtab: Vec<u8> = utils::get_target_section_from_index(&f, symtab_hdr.Link);
        let mut objfile = ObjFile::new(f.clone(), symtab, symtab_hdr, global_pos, symbols, alive);
        objfile.initialize_symbols(f);
        objfile
    } else {
        ObjFile::new_null(f.clone(), alive)
    }
}
