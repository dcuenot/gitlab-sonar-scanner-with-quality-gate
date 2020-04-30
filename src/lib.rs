#[macro_use]
extern crate serde_derive;

use anyhow::Error;
use domain::sonar::QualityStatus;
use infra::sonar_client::SonarClient;
use std::env::var;
use tokio::runtime::Runtime;

pub mod domain;
pub mod infra;

pub fn yolo(analysis_id: &str) -> Result<QualityStatus, Error> {
    let sonar_client = SonarClient::new(
        &get_env_var("SONAR_URL", "-"),
        &get_env_var("SONAR_TOKEN", "-"),
    );

    let mut rt = Runtime::new().expect("tokio runtime can be initialized");
    rt.block_on(async move { sonar_client.quality_gate_status(analysis_id).await })
}

fn get_env_var<'a>(env_var_name: &str, default_value: &str) -> String {
    var(env_var_name).unwrap_or(default_value.into())
}
