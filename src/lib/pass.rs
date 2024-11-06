use goblin::elf;
use std::{cell::RefCell, rc::Rc};

use crate::{
    elf_file, link_info,
    objfile::{self, ObjFile},
};
pub fn mark_live(linker: &mut link_info::LinkInfo, mapping: &objfile::ObjFileMapping) {
    let mut live_objs = vec![];
    for obj in &linker.object_file {
        collect_global_syms(obj);
        if obj.borrow().is_alive {
            live_objs.push(Rc::clone(obj));
        }
    }
    recursive_marking(&mut live_objs, mapping);
    println!("Before prunning {}", linker.object_file.len());

    //get rid of all unlive objfiles
    linker.object_file.retain(|obj| obj.borrow().is_alive);

    println!("After prunning {}", linker.object_file.len());
}

pub fn recursive_marking(objs: &mut Vec<Rc<RefCell<ObjFile>>>, mapping: &objfile::ObjFileMapping) {
    while !objs.is_empty() {
        let obj_file = objs.remove(0);
        println!("target file:{}", obj_file.borrow().objfile.file.file_name);
        for i in obj_file.borrow().global_pos as usize..obj_file.borrow().symbols.len() {
            let sym = &obj_file.borrow().symbols[i];
            let symbol =
                &obj_file.borrow().global_symbols[i - obj_file.borrow().global_pos as usize];
			println!("symbols:{}",symbol.borrow().name);
            if symbol.borrow().objfile.is_none() {
                continue;
            }

            let obj_ind = symbol.borrow().objfile.unwrap();
            let o = mapping.get_obj_by_ind(obj_ind);
            if o.is_none() {
                assert!(false, "what");
            }
            let obj = o.unwrap();
			println!("belongs to file:{}",obj.borrow().objfile.file.file_name);
            if sym.is_undef() && !obj.borrow().is_alive {
                obj.borrow_mut().is_alive = true;
                objs.push(Rc::clone(&obj));
            }
        }
    }
}

pub fn collect_global_syms(obj: &Rc<RefCell<objfile::ObjFile>>) {
    for i in obj.borrow().global_pos as usize..obj.borrow().symbols.len() {
        let symbol = &obj.borrow().global_symbols[i - obj.borrow().global_pos as usize];
        let sym = &obj.borrow().symbols[i];
		if symbol.borrow().name=="puts"{
			let f=&obj.borrow().objfile.file.file_name;
			let tmp=symbol.borrow().name.to_string();
			if tmp.is_empty(){
				continue;
			}
		}
        if sym.is_undef() {
            continue;
        }
        let mut shdr = None;
        if !sym.is_abs() {
            let sh = obj.borrow().objfile.Sections[obj.borrow().get_section_index(sym, i) as usize];
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
                obj.borrow().objfile.Sections[obj.borrow().get_section_index(sym, i) as usize],
                i,
            ));
        }
        if symbol.borrow().objfile.is_none() {
            symbol.borrow_mut().set_file(obj.borrow().ind);
            if let Some(s) = shdr {
                symbol.borrow_mut().set_section(s);
            }
            symbol.borrow_mut().set_value(sym.val);
            symbol.borrow_mut().set_ind(i as i32);
        }
    }
}
