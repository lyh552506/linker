use crate::objfile::ObjFile;

pub struct Symbol {
    pub file: ObjFile,
    pub name: String,
    pub value: u64,
    pub sym_idx: i32,
}

