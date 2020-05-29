#[macro_use]
extern crate log;
extern crate simplelog;

extern crate sonar_qg;

use std::path::PathBuf;

use structopt::StructOpt;

use simplelog::{ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use sonar_qg::process_quality_gate;

#[derive(StructOpt, Debug)]
#[structopt(name = "Sonar Quality Gate CLI")]
struct Options {
    #[structopt(default_value = ".scannerwork/report-task.txt", parse(from_os_str))]
    report_task_path: PathBuf,

    #[structopt(
        short = "g",
        long = "gitlab_personal_token",
        env = "GITLAB_PERSONAL_TOKEN"
    )]
    gitlab_personal_token: Option<String>,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    let options = Options::from_args();
    config_logger(options.verbose);

    match process_quality_gate(options.report_task_path, options.gitlab_personal_token) {
        Ok(result) => {
            println!("{}", result.display());
        }
        Err(e) => error!("Error: {:#?}", e),
    };
}

fn config_logger(verbose: u8) {
    let log_level = match verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let config = ConfigBuilder::new()
        .set_time_format_str("%Y-%m-%dT%H:%M:%SZ")
        .set_target_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Error)
        .add_filter_allow_str("sonar_qg")
        .build();
    TermLogger::init(log_level, config, TerminalMode::Mixed).unwrap();
}
