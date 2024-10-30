use crate::objfile::ObjFile;
#[derive(Clone)]
pub struct Symbol {
    pub file: Option<Box<ObjFile>>,
    pub name: String,
    pub value: u64,
    pub sym_idx: i32,
}

impl Symbol {
    pub fn new(f: Box<ObjFile>, Name: String, val: u64, sym_id: i32) -> Self {
        Symbol {
            file: Some(f),
            name: Name,
            value: val,
            sym_idx: sym_id,
        }
    }

    pub fn new_null(Name: String) -> Self {
        Symbol {
            file: None,
            name: Name,
            value: 0,
            sym_idx: 0,
        }
    }
}
