use std::error::Error;

use surf::{Url, http::{headers::CONTENT_TYPE}, Response};

use super::request::{ReboundRequest, ReboundRequestType};



pub struct ReboundClient {
    client: surf::Client
}

impl ReboundClient {

    pub fn new() -> Self {
        ReboundClient { client: surf::client() }
    }


    pub async fn send(&self, req: ReboundRequest) -> Result<Response, Box<dyn Error>> {
        let mut redirect_req = surf::Request
            ::builder(get_redirect_method(&req), get_url_with_params(&req))
            .body_string(req.body.unwrap_or_default())
            .build();

        redirect_req.remove_header(CONTENT_TYPE);

        for (k, v) in &req.headers {
            redirect_req.set_header(k.as_str(), v.as_str());
        }
        
        let res = self.client.send(redirect_req).await?;
        Ok(res)
    }
}

fn get_redirect_method(req: &ReboundRequest) -> surf::http::Method {
    match req.method {
        ReboundRequestType::Get => surf::http::Method::Get,
        ReboundRequestType::Post => surf::http::Method::Post,
        ReboundRequestType::Patch => surf::http::Method::Patch,
        ReboundRequestType::Put => surf::http::Method::Put,
        ReboundRequestType::Delete => surf::http::Method::Delete,
        ReboundRequestType::Head => surf::http::Method::Head,
        ReboundRequestType::Connect => surf::http::Method::Connect,
        ReboundRequestType::Trace => surf::http::Method::Trace,
        ReboundRequestType::Options => surf::http::Method::Options,
        ReboundRequestType::Invalid => panic!(),
    }
}

fn get_url_with_params(req: &ReboundRequest) -> Url {
    Url
        ::parse_with_params(
            req.uri.as_str(),
            req.query_params.iter().map(|(k, v)| -> (String, String) { (k.to_string(), v.to_string()) })
        )
        .unwrap()
}