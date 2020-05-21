use crate::domain::gitlab::merge_requests::MergeRequests;
use crate::domain::gitlab::note::Note;
use crate::domain::sonar::QualityStatus;
use crate::infra::api_call::remote_api_call::send_request;
use reqwest::header;
use std::env;

#[derive(Clone)]
pub(crate) struct GitlabClient {
    url: String,
    token: String,
    ci_project_id: i64,
}

impl GitlabClient {
    pub fn new(url: &str, token: &str, ci_project_id: i64) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
            ci_project_id,
        }
    }

    pub async fn list_opened_merge_requests(
        self,
        ci_commit_ref_name: &str,
    ) -> anyhow::Result<MergeRequests> {
        let request_builder = reqwest::Client::new()
            .get(&format!(
                "{}/api/v4/projects/{}/merge_requests",
                self.url, self.ci_project_id
            ))
            .query(&[("source_branch", ci_commit_ref_name), ("state", "opened")])
            .header("PRIVATE-TOKEN", self.token);

        send_request::<MergeRequests>(request_builder).await
    }

    pub async fn write_quality_gate_report(
        self,
        ci_merge_request_iid: i64,
        qualtiy_status: QualityStatus,
    ) -> anyhow::Result<()> {
        let request_builder = reqwest::Client::new()
            .post(&format!(
                "{}/api/v4/projects/{}/merge_requests/{}/notes",
                self.url, self.ci_project_id, ci_merge_request_iid
            ))
            .json(&Note::from_quality_status(
                qualtiy_status,
                self.ci_project_id,
                ci_merge_request_iid,
            ))
            .header("PRIVATE-TOKEN", self.token);

        send_request(request_builder).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn plop() {
        let gitlab_client = GitlabClient::new("", "", "");

        let mut rt = Runtime::new().expect("tokio runtime can be initialized");
        let res =
            rt.block_on(async move { gitlab_client.list_opened_merge_requests("test").await });

        print!("{:?}", res);
    }
}
