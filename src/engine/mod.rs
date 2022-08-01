pub mod client;
pub mod request;
pub mod response;
use regex::Regex;

use crate::conf::ReboundRule;
use self::request::ReboundRequest;

pub struct ReboundEngine {

    rules: Vec<ReboundRule>

}

impl ReboundEngine {

    pub fn new(rules: Vec<ReboundRule>) -> Self {
        ReboundEngine { rules: rules }
    }

    pub fn get(&mut self, req: impl Into<ReboundRequest>) -> Option<ReboundRequest> {

        let req: ReboundRequest = req.into();
           
        let rules_to_apply: Vec<ReboundRule> = self.rules
            .iter_mut()
            .filter(|r|  {
                let re = Regex::new(r.pattern.as_str()).unwrap();
                re.is_match(&req.uri)
            })
            .map(|r| r.clone())
            .collect();

        match rules_to_apply.first() {
            Some(rule) => Some(req.apply(rule)),
            None => None,
        }
    }
}