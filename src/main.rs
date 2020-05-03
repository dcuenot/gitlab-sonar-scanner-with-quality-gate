extern crate sonar_qg;

use structopt::StructOpt;
      
#[derive(StructOpt, Debug)]
#[structopt(name = "Some CLI tool")]
struct Options {
    // normal comments are just comments
    /// doc comments get turned into help
    #[structopt(short = "e", long = "example")]
    example: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    let options = Options::from_args();
    println!("{:?}", options);
}