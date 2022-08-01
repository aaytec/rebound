use std::error::Error;
use super::response::ReboundResponse;


pub struct ReboundClient {
    client: surf::Client
}

impl ReboundClient {

    pub fn new() -> Self {
        ReboundClient { client: surf::client() }
    }


    pub async fn send(&self, req: impl Into<surf::Request>) -> Result<ReboundResponse, Box<dyn Error>> {
        let mut res = self.client.send(req).await?;
        let res_bytes = res.body_bytes().await?;
        Ok(ReboundResponse::from(res, res_bytes))
    }
}