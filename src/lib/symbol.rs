use crate::objfile::ObjFile;
#[derive(Clone)]
pub struct Symbol {
    pub objfile: Option<usize>,
    pub name: String,
    pub value: u64,
    pub sym_idx: i32,
}

impl Symbol {
    pub fn new(f: usize, Name: String, val: u64, sym_id: i32) -> Self {
        Symbol {
            objfile: Some(f),
            name: Name,
            value: val,
            sym_idx: sym_id,
        }
    }

    pub fn new_null(Name: String) -> Self {
        Symbol {
            objfile: None,
            name: Name,
            value: 0,
            sym_idx: 0,
        }
    }

    pub fn set_file(&mut self, kind: usize) {
        self.objfile = Some(kind);
    }
}
