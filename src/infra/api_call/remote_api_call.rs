use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

pub async fn send_request<T: DeserializeOwned>(
    request_builder: RequestBuilder,
) -> anyhow::Result<T> {
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

pub fn send_request_blocking<T: DeserializeOwned>(
    request_builder: reqwest::blocking::RequestBuilder,
) -> anyhow::Result<T> {
    let response: reqwest::blocking::Response = request_builder.send()?;

    if !response.status().is_success() {
        return Err(anyhow!("{} : {:?}", response.status(), response.text()));
    }

    response.json::<T>().map_err(|e| anyhow!("{:?}", e))
}
