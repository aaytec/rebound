use std::{collections::HashMap, fmt::format};

use regex::Regex;
use tiny_http::{Header, Method, Request};

use crate::conf::ReboundRule;

pub fn match_rule(rules: &mut [ReboundRule], req_path: String) -> Option<ReboundRule> {

    for rule in rules.into_iter() {
        let re = Regex::new(rule.pattern.as_str()).unwrap();
            if re.is_match(req_path.as_str()) {
                return Some(rule.clone())
            }
    }

    None
}

#[derive(serde::Serialize, Debug)]
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

#[derive(serde::Serialize, Debug)]
pub struct ReboundRequest {



    pub uri: String,
    
    pub headers: HashMap<String, String>,

    pub query_params: HashMap<String, String>,

    pub method: ReboundRequestType,

    pub body: Option<String>

}


pub struct ReboundIngressRequestBuilder {

    rule: ReboundRule,

    url: Option<String>,

    headers: Option<Vec<Header>>,

    method: Option<Method>,

    body: Option<String>

}
impl ReboundIngressRequestBuilder {

    pub fn new(rule: &ReboundRule) -> Self {
        
        ReboundIngressRequestBuilder {
            rule: rule.clone(),
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
        
        let mut content = String::new();
        self.body = match req.as_reader().read_to_string(&mut content) {
            Ok(_) => Some(content),
            Err(_) => Some(String::new()),
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
        if self.rule.preserve_hdrs {
            if let Some(hdrs) = &self.headers {
                for hdr in hdrs.iter() {
                    headers.insert(hdr.field.to_string(), hdr.value.to_string());
                }
            }
        }

        for (k, v) in &self.rule.additional_hdrs {
            headers.insert(k.to_string(), v.to_string());
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
            .map(|x| {
                let path = if self.rule.preserve_path {
                    x
                } else {
                     String::default()
                };
                
                format!("{}{}", self.rule.redirect, path)
            })
            .unwrap_or_default()
    }

    fn build_query_params(&self) -> HashMap<String, String> {

        let mut params: HashMap<String, String> = HashMap::new();

        if self.rule.preserve_query {
            if let Some(url) = &self.url {
                if let Some(index) = url.find("?") {

                    if(index < url.len()) {
                        let all_params = &url[index+1..url.len()];
                        for query in all_params.split("&") {

                            let query_split: Vec<&str> = query.split("=").collect();
                            if query_split.len() == 2 {
                                params.insert(String::from(query_split[0]), String::from(query_split[1]));
                            }
                        }
                    }

                }
            }
        }
        
        for (k, v) in &self.rule.additional_query {
            params.insert(k.to_string(), v.to_string());
        }

        params
    }
}
