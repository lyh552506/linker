use goblin::{self, elf::Elf};

pub struct MyFile {
    pub file_name: String,
    pub ctx: Vec<u8>,
}

pub struct MyElf {
    pub file: MyFile,
    pub ElfHdr: Option<Ehdr>,
    pub Sections: Option<Vec<Shdr>>,
}

impl MyElf {
    pub fn new(f: MyFile) -> MyElf {
        MyElf {
            file: f,
            ElfHdr: None,
            Sections: None,
        }
    }
}

pub struct Ehdr {}

pub struct Shdr {}
