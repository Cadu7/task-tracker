use clap::Parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: String
}


fn main() {
    // let args = std::env::args().collect::<Vec<String>>();
    // println!("args: {:?}", args);

    let args = Args::parse();
    println!("args: {:?}", args);

}
