use crate::objfile::ObjFile;

pub struct Symbol {
    pub file: Box<ObjFile>,
    pub name: String,
    pub value: u64,
    pub sym_idx: i32,
}

impl Symbol {
    pub fn new(f: Box<ObjFile>, Name: String, val: u64, sym_id: i32) -> Self {
        Symbol {
            file: f,
            name: Name,
            value: val,
            sym_idx: sym_id,
        }
    }
}
