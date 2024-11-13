use crate::elf_file::{MyElf, MyFile};
use crate::objfile::ObjFile;
use crate::{ar_file::*, utils};
use std::{cell::RefCell, rc::Rc};

pub struct LinkInfo {
    pub output_path: String,
    pub library_path: Vec<String>,
    pub object_file: Vec<Rc<RefCell<ObjFile>>>,
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

    pub fn analysis_ar(&self, f: MyFile) -> Vec<MyElf> {
        assert!(utils::check_ar(&f.ctx));
        // println!("Name:{}", f.file_name);
        let mut objs: Vec<MyElf> = vec![];
        let mut strtab: Vec<u8> = vec![];
        let mut cur = 8; //pass magic number size
        let mut ctx = f.ctx.to_vec();
        while f.ctx.len() - cur > 1 {
            //对齐规则
            if cur % 2 == 1 {
                cur = cur + 1;
                continue;
            }
            let hdr: Arhdr = utils::read_struct(&ctx[cur..].to_vec());
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
            let mut file = MyFile::new(hdr.get_name(&strtab.to_vec()), content);

            // println!("Name:{}", file.file_name);
            objs.push(crate::utils::get_elf(file, Some(&f)));
        }

        objs
    }
}
