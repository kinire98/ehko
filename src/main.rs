use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Message to print
    msg: String
}


fn main() {
    let args = Args::parse();
    println!("{}", args.msg);
}
