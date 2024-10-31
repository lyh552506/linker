use bytemuck::NoUninit;
use goblin::{archive::Index, elf::section_header};
use itertools::Itertools;
use std::{mem::*, rc::Rc,cell::RefCell};

use crate::{
    elf_file::{self, MyElf, MyFile, Shdr, Sym},
    symbol::Symbol,
    utils::{self, read_struct},
};
static mut INDEX: usize = 0;
#[derive(Clone)]
pub struct ObjFile {
    pub objfile: MyElf,
    pub sym_tab: Vec<u8>,
    pub sym_str_tab: Option<Shdr>,
    pub global_pos: i32,
    pub symbols: Vec<elf_file::Sym>,
    pub is_alive: bool,
    pub local_symbols: Vec<Rc<RefCell<Symbol>>>,
    pub global_symbols: Vec<Rc<RefCell<Symbol>>>,
    pub ind: usize,
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
        let tmp;
        unsafe {
            tmp = INDEX;
            INDEX += 1;
        }
        ObjFile {
            objfile: obj,
            sym_tab: symtab,
            sym_str_tab: Some(sym_str),
            global_pos: pos,
            symbols: symbol,
            is_alive: alive,
            local_symbols: vec![],
            global_symbols: vec![],
            ind: tmp,
        }
    }

    pub fn new_null(obj: MyElf, alive: bool) -> Self {
        let tmp;
        unsafe {
            tmp = INDEX;
            INDEX += 1;
        }
        ObjFile {
            objfile: obj,
            sym_tab: vec![],
            sym_str_tab: None,
            global_pos: -1,
            symbols: vec![],
            is_alive: alive,
            local_symbols: vec![],
            global_symbols: vec![],
            ind: tmp,
        }
    }

    // pub fn update_local_symbols(&mut self, local_sym: Vec<Rc<Symbol>>) {
    //     self.local_symbols = local_sym;
    // }

    // pub fn update_global_symbols(&mut self, global_sym: Vec<Rc<Symbol>>) {
    //     self.global_symbols = global_sym;
    // }

    pub fn initialize_symbols(&mut self, elf: &mut MyElf) {
        if let Some(symtab) = self.sym_str_tab {
            assert!(self.global_pos != -1);
            println!("{}", self.objfile.file.file_name);
            for i in 0..self.global_pos {
                let name_index = self.symbols[i as usize].name;
                let val = self.symbols[i as usize].val;
                let name = elf_file::get_name(&self.sym_tab, name_index as usize);
                let sym = Symbol::new(self.ind, name, val, i as i32);
                self.local_symbols.push(Rc::new(RefCell::new(sym)));
            }

            //update global
            for i in self.global_pos..self.symbols.len() as i32 {
                let name_index = self.symbols[i as usize].name;
                let name = elf_file::get_name(&self.sym_tab, name_index as usize);
                let sym = elf.GetCorrespondSym(&name); //make sure only has one symbol
                self.global_symbols.push(sym);
            }
        } else {
            return;
        }
    }
}
pub fn parse_symtab(f: &mut MyElf, alive: bool) -> ObjFile {
    if let Some(symtab_hdr) = utils::find_section(&f, section_header::SHT_SYMTAB) {
        //find a symtab header, now get his symbols
        let mut symbols: Vec<elf_file::Sym> = vec![];
        // let mut source = utils::get_target_section_content(&f.file, symtab_hdr);
        // let sym_size = size_of::<elf_file::Sym>();
        // let symbols_num = source.len() / sym_size;
        // for i in 0..symbols_num {
        //     symbols.push(read_struct(&source[i * sym_size..].to_vec()));
        // }
        fill_syms(f, &symtab_hdr, &mut symbols);
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

pub fn fill_syms(f: &mut MyElf, symtab_hdr: &Shdr, symbols: &mut Vec<elf_file::Sym>) {
    let mut source = utils::get_target_section_content(&f.file, symtab_hdr);
    let sym_size = std::mem::size_of::<elf_file::Sym>();
    let nums = source.len() / sym_size;
    // let mut symbols: Vec<Sym> = vec![];
    for i in 0..nums {
        let v = source[i * sym_size..].to_vec();
        let tmp = utils::read_struct(&v);
        symbols.push(tmp);
    }
}
