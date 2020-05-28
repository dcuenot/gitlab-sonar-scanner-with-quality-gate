use reqwest::Request;
use serde::de::DeserializeOwned;

pub struct ApiCallRemoteAdapter;

impl ApiCallRemoteAdapter {
    pub async fn send<T: DeserializeOwned>(&self, request: Request) -> anyhow::Result<T> {
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
}
