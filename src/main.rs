extern crate sonar_qg;

use std::path::PathBuf;

use structopt::StructOpt;

use sonar_qg::process_quality_gate;

#[derive(StructOpt, Debug)]
#[structopt(name = "Sonar Quality Gate CLI")]
struct Options {
    #[structopt(default_value = ".scannerwork/report-task.txt", parse(from_os_str))]
    report_task_path: PathBuf,

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
    println!("{}", options.verbose);

    match process_quality_gate(options.report_task_path, options.gitlab_private_token) {
        Ok(result) => {
            // println!("{:#?}", &result);
            println!("{}", result.display());
        }
        Err(e) => println!("MY Error: {:#?}", e),
    };
}
