use log::*;
use reqwest::Request;
use serde::de::DeserializeOwned;

pub async fn send<T: DeserializeOwned>(request: Request) -> anyhow::Result<T> {
    trace!("{:?}", request);
    let response = reqwest::Client::new().execute(request).await?;
    trace!("{:?}", &response);

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
