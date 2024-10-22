use crate::elf_file::MyFile;

pub struct ObjFile {
    pub file: MyFile,
}

impl ObjFile {
    pub fn new(name: String, content: Vec<u8>) -> ObjFile {
        let _file = MyFile::new(name, content);
        ObjFile { file: _file }
    }
}
