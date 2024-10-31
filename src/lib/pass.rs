use crate::{
    link_info,
    objfile::{self, ObjFile},
};
pub fn mark_live(linker: &link_info::LinkInfo) {
    let mut live_objs = vec![];
    for obj in &linker.object_file {
        collect_global_syms(obj);
        if obj.is_alive {
            live_objs.push(obj);
        }
    }
    RecursiveMarking(&mut live_objs);
}

pub fn RecursiveMarking(objs: &mut Vec<&ObjFile>) {
    while !objs.is_empty() {
        let obj_file = objs.remove(0);
    }
}

pub fn collect_global_syms(obj: &objfile::ObjFile) {
    for i in obj.global_pos as usize..obj.symbols.len() {
        let symbol = &obj.global_symbols[i - obj.global_pos as usize];
        let sym = &obj.symbols[i];
        if sym.is_undef() {
            continue;
        }
        if !sym.is_abs() {
			
		}

        if symbol.borrow().objfile.is_none() {
            symbol.borrow_mut().set_file(obj.ind);
        }
    }
}
