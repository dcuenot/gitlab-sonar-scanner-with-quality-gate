extern crate sonar_qg;

use sonar_qg::yolo;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Sonar Quality Gate CLI")]
struct Options {
    #[structopt(short = "u", long = "sonar_url", env = "SONAR_URL")]
    sonar_url: String,

    #[structopt(short = "t", long = "sonar_token", env = "SONAR_TOKEN")]
    sonar_token: String,

    #[structopt(short = "i", long = "sonar_analysis_id")]
    sonar_analysis_id: String,

    #[structopt(
        short = "g",
        long = "gitlab_private_token",
        env = "GITLAB_PRIVATE_TOKEN"
    )]
    gitlab_private_token: Option<String>,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    let options = Options::from_args();

    match yolo(
        options.sonar_url,
        options.sonar_token,
        options.sonar_analysis_id,
        options.gitlab_private_token,
    ) {
        Ok(result) => {
            // println!("{:#?}", &result);
            println!("{}", result.display());
        }
        Err(e) => println!("MY Error: {:#?}", e),
    };
}
