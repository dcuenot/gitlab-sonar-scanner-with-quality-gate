#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;

use std::path::PathBuf;

use anyhow::Error;

use domain::sonar::QualityStatus;
use infra::sonar_client::SonarClient;

use crate::infra::sonar_analysis_params::read_scanner_work::SonarAnalysisParams;
use std::env;
use tokio::runtime::Runtime;

pub mod domain;
pub mod infra;

pub fn process_quality_gate(
    report_task_path: PathBuf,
    _gitlab_private_token: Option<String>,
) -> Result<QualityStatus, Error> {
    let params = SonarAnalysisParams::from_report_task(report_task_path);

    let sonar_token = env::var("SONAR_TOKEN").expect("Env variable SONAR_TOKEN is missing");

    // TODO: Define Trait for Sonar Client
    let sonar_client = SonarClient::new(&params.server_url, &sonar_token);
    let task = sonar_client.clone().analysis_id(&params.ce_task_id)?;

    let mut rt = Runtime::new().expect("tokio runtime can be initialized");
    rt.block_on(async move {
        sonar_client
            .clone()
            .quality_gate_status(&task.analysis_id)
            .await

        // TODO: Add if gitlab private token and in merge request => push to gitlab comments
    })
}
