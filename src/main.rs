use init::info_gather;

mod init;

fn main() {
    let args = init::argument_init::argument_init(); 

   match  init::info_gather::get_os(args.path.unwrap()){
    Ok(_) => println!("OS information gathered!"),
    Err(e)=>panic!("{}",e),
   };
    // if let Some(path) = args.path.as_deref(){
    //     println!("{}", path);
    // }
    // if let Some(os) = args.os.as_deref(){
    //     println!("{}", os);
    // }



}