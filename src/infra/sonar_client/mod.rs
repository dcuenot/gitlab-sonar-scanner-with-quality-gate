use std::time::Instant;

use anyhow::{Error, Result};
use log::*;
use reqwest::Request;
use retry::delay::Fibonacci;
use retry::{retry_with_index, OperationResult};

use crate::domain::sonar::task::{SonarBackgroundTask, Task};
use crate::domain::sonar::QualityStatus;
use crate::infra::api_call::remote_api_call::{send, send_request_blocking};
use base64::write::EncoderWriter as Base64Encoder;
use core::fmt;
use reqwest::header::HeaderValue;
use std::io::Write;

#[derive(Clone)]
pub(crate) struct SonarClient {
    url: String,
    token: String,
}

impl fmt::Debug for SonarClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SonarClient")
            .field("url", &self.url)
            .field("token", &String::from("Sensitive"))
            .finish()
    }
}

impl SonarClient {
    pub fn new(url: &str, token: &str) -> Self {
        let sonar_client = Self {
            url: url.into(),
            token: token.into(),
        };
        trace!("{:?}", sonar_client);
        sonar_client
    }

    pub async fn quality_gate_status(self, analysis_id: &str) -> anyhow::Result<QualityStatus> {
        let request_builder: Request = reqwest::Client::new()
            .get(&format!("{}/api/qualitygates/project_status", self.url))
            .query(&[("analysisId", analysis_id)])
            .header(reqwest::header::AUTHORIZATION, self.header_authorization())
            .build()?;

        send::<QualityStatus>(request_builder).await
    }

    pub fn analysis_id(&self, task_id: &str) -> anyhow::Result<SonarBackgroundTask> {
        let result = retry_with_index(Fibonacci::from_millis(1000), |current_try| {
            if current_try > 9 {
                return OperationResult::Err("plop");
            }

            match self.get_task(&task_id) {
                Ok(t) if t.status == "SUCCESS" => OperationResult::Ok(t),
                Ok(t) => {
                    error!("{}. {}", current_try, t.status);
                    OperationResult::Retry("Incorrect status")
                }
                Err(e) => {
                    error!("{}. {}", current_try, e);
                    OperationResult::Retry("Error during API call")
                }
            }
        });

        match result {
            Ok(task) => Ok(task),
            Err(_) => Err(anyhow!("Impossible to find analysis_id for {}", task_id)),
        }
    }

    fn get_task(&self, task_id: &&str) -> anyhow::Result<SonarBackgroundTask> {
        let request_builder = reqwest::blocking::Client::new()
            .get(&format!("{}/api/ce/task", self.url))
            .query(&[("id", task_id)])
            .header(reqwest::header::AUTHORIZATION, self.header_authorization());

        let res = send_request_blocking::<Task>(request_builder)?;
        debug!("{:?}", res.task);
        Ok(res.task)
    }

    // Dirty workaround could be removed once PR will be validated
    // https://github.com/seanmonstar/reqwest/pull/916
    fn header_authorization(&self) -> HeaderValue {
        let mut header_value = b"Basic ".to_vec();
        {
            let mut encoder = Base64Encoder::new(&mut header_value, base64::STANDARD);
            // The unwraps here are fine because Vec::write* is infallible.
            write!(encoder, "{}:", self.token).unwrap();
        }

        let mut token = HeaderValue::from_str(std::str::from_utf8(&header_value).unwrap())
            .expect("Issue during HeaderValue creation");
        token.set_sensitive(true);
        token
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use retry::delay::Fibonacci;
    use retry::{retry_with_index, OperationResult};

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
