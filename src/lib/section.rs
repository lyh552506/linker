use crate::{elf_file, objfile, utils::read_file};
use std::{cell::RefCell, rc::Rc};
#[derive(Clone)]
pub struct Section {
    pub file: Rc<RefCell<objfile::ObjFile>>,
    pub ctx: Vec<u8>,
    pub shndx: u32,
    pub sh_size: u32,
    pub is_alive: bool,
    pub power2align: u32,
}

impl Section {
    pub fn new(f: &Rc<RefCell<objfile::ObjFile>>, shndx: u32) -> Self {
        let obj = f.borrow();
        let shdr = &obj.objfile.Sections[shndx as usize];
        let mut start = shdr.Offset;
        let mut end = shdr.Offset + shdr.Size;
        let mut contxt = &obj.objfile.file.ctx;
        let mut ctx = vec![];
        if end > obj.objfile.file.ctx.len() as u64 && !obj.objfile.ar_content.is_none() {
            let ar = read_file(obj.objfile.ar_content.as_ref().unwrap().as_str()).unwrap();
            let off = obj.objfile.file.ctx.len();
            start = start + off as u64;
            end = start + end as u64;
            ctx = ar.ctx[start as usize..end as usize].to_vec();
			println!("ctx:{:?}",ctx);
        } else {
            ctx = contxt[start as usize..end as usize].to_vec();
        }
        let get_p2align = |alignaddr: u64| -> u32 {
            if alignaddr == 0 {
                return 0;
            }
            alignaddr.trailing_zeros()
        };
        Section {
            file: Rc::clone(f),
            ctx: ctx,
            shndx: shndx,
            sh_size: shdr.Size as u32,
            is_alive: true,
            power2align: get_p2align(shdr.AddrAlign),
        }
    }
    pub fn new_null(f: &Rc<RefCell<objfile::ObjFile>>) -> Self {
        Section {
            file: Rc::clone(f),
            ctx: vec![],
            shndx: 0,
            sh_size: 0,
            is_alive: true,
            power2align: 0,
        }
    }
}
