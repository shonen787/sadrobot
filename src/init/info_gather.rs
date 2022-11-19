use std::fs::File;
use std::io::prelude::*;

pub fn get_os(path: String) -> std::io::Result<()>{
    let mut os_relase = path;
    &os_relase.push_str("etc/os-release");
    let file_result = File::open(os_relase);

    let mut file = match file_result {
        Ok(input_file) => input_file,
        Err(error) => panic!("Problem opening the file: {:?}",error)  ,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for line in contents.lines(){
        println!("line");
    }

    Ok(())

}