use my_project_lib::{pass, utils};
use std::{
    env::{self},
};

fn main() {
    let Args: Vec<String> = env::args().collect();
    for i in 0..Args.len(){
    	println!("{}",Args[i]);
    }
    let linkinfo = utils::parse_args();
	linkinfo.print();
    
	pass::mark_live(&linkinfo);

    return;
    // if Args.len() < 2 {
    //     eprintln!("Missing args!");
    //     exit(0);
    // }
    // let elf = utils::deal_target_file(Args[1].as_str());
    // let elf = utils::get_elf("/home/lyh_irie/Arch_Learning/linker/my_linker/out/hello.o");
    // let a = elf.ElfHdr;
    println!("Hello, world!");
}
