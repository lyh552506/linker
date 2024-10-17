use crate::elf_file::MyFile;
use crate::objfile::ObjFile;
use crate::utils::*;

pub struct LinkInfo {
    pub output_path: String,
    pub library_path: Vec<String>,
    pub object_file: Vec<ObjFile>,
}

pub struct Arhdr {
    pub name: [u8; 16],
    pub date: [u8; 12],
    pub uid: [u8; 6],
    pub gid: [u8; 6],
    pub mode: [u8; 8],
    pub size: [u8; 10],
    pub fmag: [u8; 2],
}

impl LinkInfo {
    pub fn new() -> LinkInfo {
        LinkInfo {
            output_path: "".to_string(),
            library_path: vec![],
            object_file: vec![],
        }
    }

    pub fn print(&self) {
        println!("\x1b[34mOutput path: {}\x1b[0m", self.output_path);
        println!("\x1b[34mlib path:\x1b[0m");
        for (ind, path) in self.library_path.iter().enumerate() {
            println!("{}", path);
        }
    }

    pub fn append_obj(&mut self, f: MyFile) {
        self.object_file.push(ObjFile { file: f });
    }

    pub fn analysis_ar(&self, f: MyFile) {
        assert!(check_ar(&f.ctx));
        println!("Name:{}", f.file_name);
    }
}
