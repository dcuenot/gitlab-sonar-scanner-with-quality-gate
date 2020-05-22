#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;

use log::*;
use std::path::PathBuf;

use anyhow::Error;

use domain::sonar::QualityStatus;
use infra::sonar_client::SonarClient;

use crate::infra::gitlab_client::GitlabClient;
use crate::infra::sonar_analysis_params::read_scanner_work::SonarAnalysisParams;
use std::env;
use tokio::runtime::Runtime;

pub mod domain;
pub mod infra;

const ENV_NAME_SONAR_TOKEN: &str = "SONAR_TOKEN";

pub fn process_quality_gate(
    report_task_path: PathBuf,
    _gitlab_private_token: Option<String>,
) -> Result<QualityStatus, Error> {
    let params = SonarAnalysisParams::from_report_task(report_task_path);

    if env::var(ENV_NAME_SONAR_TOKEN).is_err() {
        bail!("Environment variable {} is missing", ENV_NAME_SONAR_TOKEN);
    }
    let sonar_token = env::var(ENV_NAME_SONAR_TOKEN).unwrap();

    // TODO: Define Trait for Sonar Client
    let sonar_client = SonarClient::new(&params.server_url, &sonar_token);
    let task = sonar_client.clone().analysis_id(&params.ce_task_id)?;

    let mut rt = Runtime::new().expect("tokio runtime can be initialized");
    rt.block_on(async move {
        let quality_status = sonar_client
            .clone()
            .quality_gate_status(&task.analysis_id)
            .await?;

        let gitlab_client = GitlabClient::new(
            &env::var("GITLAB_URL").ok().unwrap(),
            &env::var("GITLAB_PRIVATE_TOKEN").ok().unwrap(),
            12737,
        );

        let opened_mr = gitlab_client
            .clone()
            .list_opened_merge_requests("test")
            .await?;

        gitlab_client
            .write_quality_gate_report(opened_mr[0].iid, quality_status.clone())
            .await;
        // TODO: Add if gitlab_client private token and in merge request => push to gitlab_client comments

        Ok(quality_status)
    })
}
