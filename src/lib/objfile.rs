use std::{fs::exists, process::exit};

use goblin::elf::section_header;

use crate::{
    elf_file::{self, MyElf, MyFile, Shdr, Sym},
    utils::{self, read_struct},
};

pub struct ObjFile {
    pub objfile: MyElf,
    pub sym_str_tab: Shdr,
    pub global_pos: u32,
    pub symbols: Vec<elf_file::Sym>,
}

impl ObjFile {
    pub fn new(f: MyElf, sym_str: Shdr, pos: u32, symbol: Vec<elf_file::Sym>) -> ObjFile {
        let obj = f;
        ObjFile {
            objfile: obj,
            sym_str_tab: sym_str,
            global_pos: pos,
            symbols: symbol,
        }
    }
}
pub fn parse_symtab(f: &MyElf) -> Option<ObjFile> {
    if let Some(symtab_hdr) = utils::find_section(&f, section_header::SHT_SYMTAB) {
        //find a symtab header, now get his symbols
        let mut symbols: Vec<Sym> = vec![];
        let mut source = utils::get_target_section_content(&f.file, symtab_hdr);
        let sym_size = size_of::<Sym>();
        let mut symbols_num = source.len() / sym_size;
        for i in 0..symbols_num {
            symbols.push(read_struct(&source[i * sym_size..].to_vec()));
        }
        //get the firat global symbol's position
        let global_pos = symtab_hdr.Info;
        //get the str section
        let str_sec = f.Sections[symtab_hdr.Link as usize];
        Some(ObjFile::new(f.clone(), symtab_hdr, global_pos, symbols))
    } else {
		None
	}
}
