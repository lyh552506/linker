use goblin::{self, elf::Elf};
use bytemuck::{Pod,Zeroable};
pub struct MyFile {
    pub file_name: String,
    pub ctx: Vec<u8>,
}

pub struct MyElf {
    pub file: MyFile,
    pub ElfHdr: Ehdr,
    pub Sections:Vec<Shdr>,
}

impl MyElf {
    pub fn new(f: MyFile,e:Ehdr,Sec:Vec<Shdr>) -> MyElf {
        MyElf {
            file: f,
            ElfHdr: e,
            Sections: Sec,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ehdr {
    pub Ident: [u8; 16],
    pub Type: u16,
    pub Machine: u16,
    pub Version: u32,
    pub Entry: u64,
    pub PhOff: u64,
    pub ShOff: u64,
    pub Flags: u32,
    pub EhSize: u16,
    pub PhEntSize: u16,
    pub PhNum: u16,
    pub ShEntSize: u16,
    pub ShNum: u16,
    pub ShStrndx: u16,
}
unsafe impl Zeroable for Ehdr {}
unsafe impl Pod for Ehdr {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Shdr {
    pub Name: u32,
    pub Type: u32,
    pub Flags: u64,
    pub Addr: u64,
    pub Offset: u64,
    pub Size: u64,
    pub Link: u32,
    pub Info: u32,
    pub AddrAlign: u64,
    pub EntSize: u64,
}
unsafe impl Zeroable for Shdr {}
unsafe impl Pod for Shdr {}