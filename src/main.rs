mod init;

fn main() {
    let args = init::argument_init::argument_init(); 

    if let Some(path) = args.path.as_deref(){
        println!("{}", path);
    }
    if let Some(os) = args.os.as_deref(){
        println!("{}", os);
    }

}