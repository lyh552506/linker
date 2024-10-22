use crate::ar_file::*;
use crate::elf_file::MyFile;
use crate::objfile::ObjFile;
use crate::utils::*;

pub struct LinkInfo {
    pub output_path: String,
    pub library_path: Vec<String>,
    pub object_file: Vec<ObjFile>,
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

    pub fn analysis_ar(&self, f: MyFile) -> Vec<ObjFile> {
        assert!(check_ar(&f.ctx));
        println!("Name:{}", f.file_name);
        let mut objs: Vec<ObjFile> = vec![];
        let mut strtab: Vec<u8> = vec![];
        let mut cur = 8; //pass magic number size
        let mut ctx = f.ctx.to_vec();
        while f.ctx.len() - cur > 1 {
            //对齐规则
            if cur % 2 == 1 {
                cur = cur + 1;
                continue;
            }
            let hdr: Arhdr = read_struct(&ctx[cur..].to_vec());
            let start = cur + std::mem::size_of::<Arhdr>();
            let end = start
                + std::str::from_utf8(&hdr.size[..])
                    .expect("Invalid UTF-8")
                    .trim()
                    .parse::<usize>()
                    .expect("Fail");
            cur = end;
            let content = f.ctx[start..end].to_vec();
            if hdr.is_str_tab() {
                strtab = content;
                continue;
            }
            if hdr.is_symtab() {
                continue;
            }
            objs.push(ObjFile::new(hdr.get_name(&strtab.to_vec()), content));
        }

        objs
    }
}
