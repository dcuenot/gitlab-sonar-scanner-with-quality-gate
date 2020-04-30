use crate::infra::api_call::remote_api_call::send_request;

use crate::domain::sonar::QualityStatus;
use anyhow::Error;

pub(crate) struct SonarClient {
    url: String,
    token: String,
}

impl SonarClient {
    pub fn new(url: &str, token: &str) -> SonarClient {
        SonarClient {
            url: String::from(url),
            token: String::from(token),
        }
    }

    pub async fn quality_gate_status(self, analysis_id: &str) -> Result<QualityStatus, Error> {
        let request_builder = reqwest::Client::new()
            .get(&format!("{}/api/qualitygates/project_status", self.url))
            .query(&[("analysisId", analysis_id)])
            .basic_auth(&self.token, None::<String>);

        send_request::<QualityStatus>(request_builder).await
    }
}
