use bytemuck::NoUninit;
use core::num;
use goblin::{
    archive::Index,
    elf::{self, section_header},
};
use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, mem::*, process::id, rc::Rc};

use crate::{
    elf_file::{self, MyElf, MyFile, Shdr, Sym},
    symbol::Symbol,
    utils::{self, read_struct},
	mergeable_section,
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
    pub symtab_shndx_section: Vec<u32>,
    pub ind: usize,
	pub mergeable_sections:Vec<Option<mergeable_section::MergeableSec>>,
}

pub struct ObjFileMapping {
    ind2obj: HashMap<usize, Rc<RefCell<ObjFile>>>,
}

impl ObjFileMapping {
    pub fn new() -> Self {
        ObjFileMapping {
            ind2obj: HashMap::new(),
        }
    }
    pub fn add_obj(&mut self, objfile: &Rc<RefCell<ObjFile>>) {
        self.ind2obj
            .insert(objfile.borrow().ind, Rc::clone(objfile));
    }

    pub fn get_obj_by_ind(&self, ind: usize) -> Option<Rc<RefCell<ObjFile>>> {
        self.ind2obj.get(&ind).cloned()
    }
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
            symtab_shndx_section: vec![],
			mergeable_sections:vec![],
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
            symtab_shndx_section: vec![],
			mergeable_sections:vec![],
        }
    }

    pub fn fill_symtab_shndx_section(&mut self) {
        let section = utils::find_section(&self.objfile, section_header::SHT_SYMTAB_SHNDX);
        if let Some(shdr) = section {
            let source = utils::get_target_section_content(&self.objfile.file, &shdr);
            let nums = source.len() / 4;
            for i in 0..nums {
                let tmp = read_struct(&source[i * 4..].to_vec());
                self.symtab_shndx_section.push(tmp);
            }
        }
    }

    pub fn get_section_index(&self, sym: &Sym, ind: usize) -> u64 {
        if sym.shndx == goblin::elf::section_header::SHN_XINDEX as u16 {
            return self.symtab_shndx_section[ind] as u64;
        }
        return sym.shndx as u64;
    }

    fn initialize_symbols(&mut self, elf: &mut MyElf) {
        // let mut name="".to_string();
		// if elf.file.file_name == "ioputs.o"||elf.file.file_name == "out/hello.o" {
		// 	name=elf.file.file_name.to_string();
		// }
        if let Some(symtab) = self.sym_str_tab {
            assert!(self.global_pos != -1);
            // println!("{}", self.objfile.file.file_name);
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
                let sym = utils::get_correspond_sym(&name); //make sure only has one symbol
                self.global_symbols.push(sym);
            }
        } else {
            return;
        }
    }

	fn initialize_mergeable_sections(&mut self, elf: &MyElf){
		self.mergeable_sections.resize(elf.Sections.len(), None);
		for i in 0..elf.Sections.len(){
			let sec=elf.Sections[i];
		}
	}
}
pub fn parse_symtab(f: &mut MyElf, alive: bool, objmaping: &ObjFileMapping) -> ObjFile {
    if let Some(symtab_hdr) = utils::find_section(&f, section_header::SHT_SYMTAB) {
        //find a symtab header, now get his symbols
        let mut symbols: Vec<elf_file::Sym> = vec![];
        fill_syms(f, &symtab_hdr, &mut symbols);
        //get the firat global symbol's position
        let global_pos = symtab_hdr.Info as i32;
        //get the str section
        let symtab: Vec<u8> = utils::get_target_section_from_index(&f, symtab_hdr.Link);
        let mut objfile = ObjFile::new(f.clone(), symtab, symtab_hdr, global_pos, symbols, alive);
        objfile.fill_symtab_shndx_section();
        objfile.initialize_symbols(f);
		objfile.initialize_mergeable_sections(f);
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
