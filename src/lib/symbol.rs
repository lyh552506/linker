use crate::{elf_file::Shdr, objfile::ObjFile};
use std::{rc::Rc,cell::RefCell,collections::HashMap};


thread_local! {
    pub static SYMBOL_MAP: RefCell<HashMap<String, Rc<RefCell<Symbol>>>> = RefCell::new(HashMap::new());
}
#[derive(Clone)]
pub struct Symbol {
    pub objfile: Option<usize>,
    pub section: Option<(Shdr, usize)>,
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
            section: None,
        }
    }

    pub fn new_null(Name: String) -> Self {
        Symbol {
            objfile: None,
            name: Name,
            value: 0,
            sym_idx: 0,
            section: None,
        }
    }

    pub fn set_file(&mut self, kind: usize) {
        self.objfile = Some(kind);
    }

    pub fn set_section(&mut self, sec: (Shdr, usize)) {
        self.section = Some(sec);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }
    pub fn set_ind(&mut self, ind: i32) {
        self.sym_idx = ind;
    }

	pub fn add_symbol(name: String, symbol: Rc<RefCell<Symbol>>) {
        SYMBOL_MAP.with(|map| {
            map.borrow_mut().insert(name, symbol);
        })
    }

    pub fn get_symbol(name: &str) -> Option<Rc<RefCell<Symbol>>> {
        SYMBOL_MAP.with(|map| {
            map.borrow().get(name).cloned()
        })
    }
}
