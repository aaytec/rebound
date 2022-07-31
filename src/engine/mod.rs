mod client;
mod request;

use std::fs::File;

use log::info;
use tiny_http::{Request, Response, StatusCode};
use crate::conf::{ReboundRule, REBOUND_SITE_DIR};

use self::{client::ReboundClient, request::{ReboundIngressRequestBuilder, match_rule}};

pub struct ReboundEngine {

    rules: Vec<ReboundRule>,

    pub client: ReboundClient

}

impl ReboundEngine {

    pub fn new(rules: Vec<ReboundRule>) -> Self {
        ReboundEngine { rules: rules, client: ReboundClient::new() }
    }

    pub async fn rebound(&mut self, mut req: Request) -> Result<(), Box<dyn std::error::Error>> {

        if let Some(rule) = match_rule(self.rules.as_mut_slice(), req.url().to_string())  {

            let rebound_req = ReboundIngressRequestBuilder::new(&rule)
                    .with_url(req.url().to_string())
                    .with_headers(req.headers())
                    .with_method(req.method())
                    .with_body(&mut req)
                    .build();

            info!("Redirect Request: {:?}", rebound_req);
            let res = self.client.send(rebound_req).await?;
            info!("Sending Response: {:?}", res);

            req.respond(Response::from_string("200 OK\r\n\r\n"))?
        }
        else {
            req.respond(Response::from_file(File::open(format!("{}/default.html", std::env::var(REBOUND_SITE_DIR).unwrap())).unwrap()).with_status_code(404))?
        }

        Ok(())
    }
}
