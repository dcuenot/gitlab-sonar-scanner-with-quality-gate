use std::time::Instant;

use anyhow::{Error, Result};
use retry::{OperationResult, retry_with_index};
use retry::delay::Fibonacci;

use crate::domain::sonar::QualityStatus;
use crate::domain::sonar::task::{SonarBackgroundTask, Task};
use crate::infra::api_call::remote_api_call::{send_request, send_request_blocking};

#[derive(Clone)]
pub(crate) struct SonarClient {
    url: String,
    token: String,
}

impl SonarClient {
    pub fn new(url: &str, token: &str) -> SonarClient {
        SonarClient {
            url: url.into(),
            token: token.into(),
        }
    }

    pub async fn quality_gate_status(self, analysis_id: &str) -> Result<QualityStatus, Error> {
        let request_builder = reqwest::Client::new()
            .get(&format!("{}/api/qualitygates/project_status", self.url))
            .query(&[("analysisId", analysis_id)])
            .basic_auth(&self.token, None::<String>);

        send_request::<QualityStatus>(request_builder).await
    }

    pub fn analysis_id(&self, task_id: &str) -> Result<SonarBackgroundTask> {
        let now = Instant::now();

        let result = retry_with_index(Fibonacci::from_millis(1000), |current_try| {
            if current_try > 9 {
                return OperationResult::Err("plop");
            }

            match self.get_task(&task_id) {
                Ok(t) if t.task.status == "SUCCESS" => OperationResult::Ok(t.task),
                Ok(t) => {
                    println!(
                        "Try: {} - {} sec - {}",
                        current_try,
                        now.elapsed().as_millis(),
                        t.task.status
                    );
                    OperationResult::Retry("Incorrect status")
                }
                Err(_) => OperationResult::Retry("Error during API call"),
            }
        });

        match result {
            Ok(task) => Ok(task),
            Err(_) => Err(anyhow!("Impossible to find analysis_id for {}", task_id)),
        }
    }

    fn get_task(&self, task_id: &&str) -> anyhow::Result<Task> {
        let request_builder = reqwest::blocking::Client::new()
            .get(&format!("{}/api/ce/task", self.url))
            .query(&[("id", task_id)])
            .basic_auth(&self.token, None::<String>);

        send_request_blocking::<Task>(request_builder)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use retry::{OperationResult, retry_with_index};
    use retry::delay::{Fibonacci};

    #[test]
    fn test_retry_mechanism() {
        let mut collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();
        let now = Instant::now();

        let result = retry_with_index(Fibonacci::from_millis(1000), |current_try| {
            println!("elapsed: {} sec", now.elapsed().as_millis());
            if current_try > 3 {
                return OperationResult::Err("did not succeed within 3 tries");
            }

            match collection.next() {
                Some(n) if n == 10 => OperationResult::Ok("n is 10!"),
                Some(_) => OperationResult::Retry("n must be 5!"),
                None => OperationResult::Retry("n was never 5!"),
            }
        });

        assert!(result.is_err());
    }
}
