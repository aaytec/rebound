use std::collections::HashMap;
use tiny_http::{Header, Method, Request};

use super::circuit::{CircuitNode, CircuitPath, CircuitType, CircuitUpstream};

#[derive(serde::Serialize, Clone, Debug)]
pub enum ReboundRequestType {

    Get,
    Post,
    Patch,
    Put,
    Delete,
    Head,
    Connect,
    Trace,
    Options,

    // not standard method type
    Invalid

}

#[derive(serde::Serialize, Clone, Debug)]
pub struct ReboundRequest {

    pub uri: String,

    pub method: ReboundRequestType,
    
    pub headers: HashMap<String, String>,

    pub query_params: HashMap<String, String>,

    pub body: Option<Vec<u8>>

}

impl ReboundRequest {


    pub fn apply(&self, cnode: &CircuitNode) -> Option<ReboundRequest> {

        let ctype = &cnode.circuit_type;

        match ctype {
            CircuitType::Routable => {
                let mut new_req = self.clone();

                if !cnode.rule.as_ref().unwrap().preserve_hdrs {
                    new_req.headers.clear();
                }

                for (k, v) in &cnode.rule.as_ref().unwrap().additional_hdrs {
                    new_req.headers.insert(k.to_string(), v.to_string());
                }

                if !cnode.rule.as_ref().unwrap().preserve_query {
                    new_req.query_params.clear();
                }

                for (k, v) in &cnode.rule.as_ref().unwrap().additional_query {
                    new_req.query_params.insert(k.to_string(), v.to_string());
                }
                
                let upstream = cnode.rule.as_ref().unwrap().upstream.clone();
                let ends_with_dir = upstream.ends_with('/');
                let upstream_path = CircuitUpstream::from(upstream);
                let req_path = CircuitPath::from(new_req.uri.clone());

                if upstream_path.path.ordered_path.len() == 0 && !ends_with_dir {
                    new_req.uri = upstream_path.join(&req_path).into();
                }
                else {
                    let cpath = cnode.path.as_ref().unwrap();
                    let diff_path = req_path.get_diff(cpath);
                    new_req.uri = upstream_path.join(&diff_path).into();
                }

                Some(new_req)
            },
            
            CircuitType::Error => None,
        }

        
    }
}

impl Into<surf::Request> for ReboundRequest {

    fn into(self) -> surf::Request {

        let method = match self.method {
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
        };

        let full_url = 
        surf::Url
            ::parse_with_params(
                self.uri.as_str(),
                    self.query_params.iter().map(|(k, v)| -> (String, String) { (k.to_string(), v.to_string()) })
            )
            .unwrap();

        let mut upstream_req = surf::Request
            ::builder(method, full_url)
            .body(self.body.unwrap_or_default())
            .build();

        upstream_req.remove_header(surf::http::headers::CONTENT_TYPE);

        self.headers.iter().for_each(|(k, v)| {
            upstream_req.set_header(k.as_str(), v.as_str());
        });

        upstream_req
    }
}

impl From<&mut tiny_http::Request> for ReboundRequest {
    fn from(req: &mut tiny_http::Request) -> Self {
        ReboundIngressRequestBuilder
            ::new()
            .with_method(req.method())
            .with_headers(req.headers())
            .with_url(req.url().to_string())
            .with_body(req)
            .build()
    }
}


pub struct ReboundIngressRequestBuilder {

    url: Option<String>,

    headers: Option<Vec<Header>>,

    method: Option<Method>,

    body: Option<Vec<u8>>

}
impl ReboundIngressRequestBuilder {

    pub fn new() -> Self {
        
        ReboundIngressRequestBuilder {
            url: None,
            headers: None,
            method: None,
            body: None
        }
    }

    pub fn with_url(&mut self, url: String) -> &mut Self {
        
        self.url = Some(url);
        self
    }

    pub fn with_headers(&mut self, hdrs: &[Header]) -> &mut Self {
        
        self.headers = Some(hdrs.to_vec());
        self
    }

    pub fn with_method(&mut self, method: &Method) -> &mut Self {
        
        self.method = Some(method.clone());
        self
    }

    pub fn with_body(&mut self, req: &mut Request) -> &mut Self {
        
        let mut content = Vec::new();
        self.body = match req.as_reader().read_to_end(&mut content) {
            Ok(_) => Some(content),
            Err(_) => Some(Vec::new()),
        };

        self
    }

    pub fn build(&self) -> ReboundRequest {
        
        ReboundRequest { 
            uri: self.build_uri(),
            headers: self.build_hdrs(),
            query_params: self.build_query_params(), 
            method: self.build_method(),
            body: self.body.clone()
        }

    }

    fn build_hdrs(&self) -> HashMap<String, String> {

        let mut headers: HashMap<String, String> = HashMap::new();
        if let Some(hdrs) = &self.headers {
            for hdr in hdrs.iter() {
                headers.insert(hdr.field.to_string(), hdr.value.to_string());
            }
        }

        headers
    }

    fn build_method(&self) -> ReboundRequestType {

        if let Some(m) = &self.method {
            return match m {
                Method::Get => ReboundRequestType::Get,
                Method::Head => ReboundRequestType::Head,
                Method::Post => ReboundRequestType::Post,
                Method::Put => ReboundRequestType::Put,
                Method::Delete => ReboundRequestType::Delete,
                Method::Connect => ReboundRequestType::Connect,
                Method::Options => ReboundRequestType::Options,
                Method::Trace => ReboundRequestType::Trace,
                Method::Patch => ReboundRequestType::Patch,
                Method::NonStandard(_) => ReboundRequestType::Invalid,
            }
        }

        ReboundRequestType::Invalid
    }


    fn build_uri(&self) -> String {
        if self.url.is_none() {
            return String::default()
        }

        self.url
            .as_ref()
            .map(|x| {
                if let Some(index) = x.find("?") {
                    String::from(&x[0..index])
                }
                else {
                    x.clone()
                }
            })
            .unwrap_or_default()
    }

    fn build_query_params(&self) -> HashMap<String, String> {

        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(url) = &self.url {
            if let Some(index) = url.find("?") {

                if index < url.len() {
                    let all_params = &url[index+1..url.len()];
                    for query in all_params.split("&") {

                        if let Some((k, v)) = query.split_once("=") {
                            params.insert(String::from(k), String::from(v));
                        }
                    }
                }

            }
        }

        params
    }
}
