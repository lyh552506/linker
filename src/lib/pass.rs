use goblin::elf;

use crate::{
    elf_file, link_info,
    objfile::{self, ObjFile},
};
pub fn mark_live(linker: &mut link_info::LinkInfo) {
    let mut live_objs = vec![];
    for obj in &mut linker.object_file {
        collect_global_syms(obj);
        if obj.is_alive {
            live_objs.push(obj);
        }
    }
    RecursiveMarking(&mut live_objs);
}

pub fn RecursiveMarking(objs: &mut Vec<&mut ObjFile>) {
    while !objs.is_empty() {
        let obj_file = objs.remove(0);
        for i in obj_file.global_pos as usize..obj_file.symbols.len() {
            let sym = &obj_file.symbols[i];
            let symbol = &obj_file.global_symbols[i - obj_file.global_pos as usize];
            if symbol.borrow().objfile.is_none() {
                continue;
            }
            if is_alive(obj_file, sym) {
				//TODO
			}
            // if sym.is_undef() && !obj_file.is_alive {
            // 	obj_file.is_alive=true;
            // 	objs.push(obj_file);
            // }
        }
    }
}

pub fn is_alive(obj: &ObjFile, sym: &elf_file::Sym) -> bool {
    if sym.is_undef() && !obj.is_alive {
        // obj.is_alive = true;
        return true;
    }
    return false;
}

pub fn collect_global_syms(obj: &objfile::ObjFile) {
    for i in obj.global_pos as usize..obj.symbols.len() {
        let symbol = &obj.global_symbols[i - obj.global_pos as usize];
        let sym = &obj.symbols[i];
        if sym.is_undef() {
            continue;
        }
        let mut shdr = None;
        if !sym.is_abs() {
            let sh = obj.objfile.Sections[obj.get_section_index(sym, i) as usize];
            if sh.Type == elf::section_header::SHT_GROUP
                || sh.Type == elf::section_header::SHT_SYMTAB
                || sh.Type == elf::section_header::SHT_STRTAB
                || sh.Type == elf::section_header::SHT_REL
                || sh.Type == elf::section_header::SHT_RELA
                || sh.Type == elf::section_header::SHT_NULL
            {
                continue;
            }
            shdr = Some((
                obj.objfile.Sections[obj.get_section_index(sym, i) as usize],
                i,
            ));
        }

        if symbol.borrow().objfile.is_none() {
            symbol.borrow_mut().set_file(obj.ind);
            if let Some(s) = shdr {
                symbol.borrow_mut().set_section(s);
            }
            symbol.borrow_mut().set_value(sym.val);
            symbol.borrow_mut().set_ind(i as i32);
        }
    }
}
