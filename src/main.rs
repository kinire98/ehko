use clap::Parser;



/// This crate is nothing special. 
/// I just wanted to test clap and crates.io


#[derive(Parser)]
struct Args {
    /// Message to print
    msg: String
}


fn main() {
    let args = Args::parse();
    println!("{}", args.msg);
}
