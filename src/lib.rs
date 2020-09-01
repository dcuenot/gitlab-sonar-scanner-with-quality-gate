#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use std::path::PathBuf;

use domain::sonar::QualityStatus;
use infra::sonar_client::SonarClient;

use crate::infra::gitlab_client::GitlabClient;
use crate::infra::sonar_analysis_params::read_scanner_work::SonarAnalysisParams;
use std::env;
use tokio::runtime::Runtime;

pub mod domain;
pub mod infra;

const ENV_NAME_SONAR_TOKEN: &str = "SONAR_TOKEN";
const ENV_NAME_GITLAB_URL: &str = "CI_SERVER_URL";
const ENV_NAME_GITLAB_PROJECT_ID: &str = "CI_PROJECT_ID";
const ENV_NAME_GITLAB_BRANCH_NAME: &str =  "CI_COMMIT_BRANCH";

pub fn process_quality_gate(
    report_task_path: PathBuf,
    gitlab_personal_token: Option<String>,
) -> anyhow::Result<QualityStatus> {
    let params = SonarAnalysisParams::from_report_task(report_task_path);

    if env::var(ENV_NAME_SONAR_TOKEN).is_err() {
        bail!("Environment variable {} is missing", ENV_NAME_SONAR_TOKEN);
    }
    let sonar_token = env::var(ENV_NAME_SONAR_TOKEN).unwrap();

    let sonar_client = SonarClient::new(&params.server_url, &sonar_token);
    let task = sonar_client.analysis_id(&params.ce_task_id)?;

    // TODO: Define Trait for Sonar Client
    let mut rt = Runtime::new().expect("tokio runtime can be initialized");
    rt.block_on(async move {
        let quality_status = sonar_client
            .clone()
            .quality_gate_status(&task.analysis_id)
            .await?;

        if let Some(personal_token) = gitlab_personal_token {
            let hint = "It seems this script is not running in a Gitlab pipeline";
            if env::var(ENV_NAME_GITLAB_URL).is_err() {
                bail!(
                    "Environment variable {} is missing. {}",
                    ENV_NAME_GITLAB_URL,
                    hint
                );
            }
            if env::var(ENV_NAME_GITLAB_PROJECT_ID).is_err() {
                bail!(
                    "Environment variable {} is missing. {}",
                    ENV_NAME_GITLAB_PROJECT_ID,
                    hint
                );
            }

            let gitlab_client = GitlabClient::new(
                &env::var(ENV_NAME_GITLAB_URL).unwrap(),
                &personal_token,
                env::var(ENV_NAME_GITLAB_PROJECT_ID)
                    .unwrap()
                    .parse::<i64>()?,
            );

            let opened_mr = gitlab_client
                .clone()
                .list_opened_merge_requests(&env::var(ENV_NAME_GITLAB_BRANCH_NAME).unwrap())
                .await?;

            for mr in opened_mr.into_iter() {
                gitlab_client
                    .clone()
                    .write_quality_gate_report(mr.project_id, mr.iid, quality_status.clone())
                    .await?;
            }
        }

        Ok(quality_status)
    })
}
