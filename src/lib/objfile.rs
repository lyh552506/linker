use crate::elf_file::MyFile;

pub struct ObjFile {
    pub file: MyFile,
}

impl ObjFile {
    pub fn new(f: MyFile) -> ObjFile {
        ObjFile { file: f }
    }
}
