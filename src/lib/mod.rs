#![allow(unused)]
use goblin;
use std::env;
use std::fs;
use std::str;

pub mod link_info;
pub mod elf_file;
pub mod objfile;
pub mod utils;
pub mod ar_file;
pub mod symbol;
static WorkSpaceFolderPlace:&str="/home/lyh_irie/Arch_Learning/linker/my_linker";