
use std::fs::File;
use std::io::Cursor;
use std::str::FromStr;

use flume::Receiver;
use log::{info, error};
use tiny_http::{Request, Response, Header};

use crate::conf::{ReboundConf, REBOUND_SITE_DIR};
use crate::engine::ReboundEngine;
use crate::engine::client::ReboundClient;

///
/// 
pub struct WorkerNode {

    ///
    /// 
    pub id: String,

    ///
    /// 
    request_queue_rx: Receiver<Request>,

    ///
    /// 
    engine: ReboundEngine,

    ///
    /// 
    client: ReboundClient
}

///
/// 
impl WorkerNode {

    pub fn from(wid: String, c: ReboundConf, receiver: Receiver<Request>) -> Self {
        WorkerNode { 
            id: wid,
            request_queue_rx: receiver,
            engine: ReboundEngine::new(c.rules.unwrap_or_else(|| Vec::new())),
            client: ReboundClient::new()
        }
    }

    pub fn run(&mut self) {

        for mut conn_req in self.request_queue_rx.iter() {

            info!("{} handling request: {:?}", self.id, conn_req);
            let r = self.engine.get(&mut conn_req);
            match r {
                Some(rebound_req) => {
                    info!("{} sending redirect request: {:?}", self.id, rebound_req);
                    match futures::executor::block_on(self.client.send(rebound_req)) {
                        Ok(rebound_res) => {

                            let rebound_res_body_size = rebound_res.body.len();
                            let res_status = conn_req.respond(
                                Response::new(
                                    rebound_res.status.into(),
                                    rebound_res.headers
                                        .iter()
                                        .map(|(k, v)| { 
                                            Header::from_str(format!("{}:{}", k.as_str(), v.as_str()).as_str()).unwrap()
                                        })
                                        .collect::<Vec<Header>>(),
                                    Cursor::new(rebound_res.body), 
                                    Some(rebound_res_body_size),
                                    None
                                )
                            );

                            match res_status {
                                Ok(_) => info!("{} sent response from rule, finished request", self.id),
                                Err(_) => error!("{} failed to send response from rule", self.id),
                            }

                        },
                        Err(_) => {
                            match conn_req.respond(Response::from_file(File::open(format!("{}/default.html", std::env::var(REBOUND_SITE_DIR).unwrap())).unwrap()).with_status_code(404)) {
                                Ok(_) => info!("{} sent error response, finished request", self.id),
                                Err(_) => error!("{} failed to send error response", self.id),
                            }
                        },
                    }

                },
                None => {
                    match conn_req.respond(Response::from_file(File::open(format!("{}/default.html", std::env::var(REBOUND_SITE_DIR).unwrap())).unwrap()).with_status_code(404)) {
                        Ok(_) => info!("{} sent default response, finished request", self.id),
                        Err(_) => error!("{} failed to send default response", self.id),
                    }
                }
            }
        }
    }
}