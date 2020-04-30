use anyhow::{anyhow, Error};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

pub async fn send_request<T: DeserializeOwned>(
    request_builder: RequestBuilder,
) -> Result<T, Error> {
    let response = request_builder.send().await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "{} : {}",
            response.status(),
            response.text().await?
        ));
    }

    let resp = response.json::<T>();
    Ok(resp.await?)
}
