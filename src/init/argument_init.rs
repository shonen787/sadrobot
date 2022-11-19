use clap::Parser;

//Struct for arguments
#[derive(Parser, Debug)]
#[command(
    author = "Christopher Sanchez", 
    version="1.0.0", 
    about = "Application to pull linux data", 
    long_about = None)]

pub struct Args {
    #[arg(short,long)]
    pub path: Option<String>,
    #[arg(short,long)]
    pub os: Option<String>,
}

pub fn argument_init() -> Args{
    let args = Args::parse();
    args
}