use super::request::{ReboundRequest, ReboundRequestType};



pub struct ReboundClient {}

impl ReboundClient {

    pub fn new() -> Self {
        ReboundClient { }
    }


    pub async fn send(&self, req: ReboundRequest) -> () {
        // let mut redirect_builder = surf::Request::builder(get_redirect_method(&ingress_req.method), Url::parse(format!("{}{}", rule.redirect, ingress_req.path_uri).as_str()).unwrap());
        // if let Some(body) = &ingress_req.body {
        //     redirect_builder.body_string(body.to_string());
        // }

        // for (k, v) in &ingress_req.headers {
        //     redirect_builder.header(k.as_str(), v.as_str());
        // }
    }
}

fn get_redirect_method(method: &ReboundRequestType) -> surf::http::Method {
    match method {
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