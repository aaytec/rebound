use std::fs::File;

use flume::Receiver;
use log::{error, info};
use tiny_http::{Request, Response};

use crate::conf::ReboundConf;
use crate::engine::circuit::Circuit;
use crate::engine::client::ReboundClient;
use crate::engine::ReboundEngine;

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
    client: ReboundClient,
}

///
///
impl WorkerNode {
    pub fn from(wid: String, _conf: ReboundConf, circuit: Circuit, receiver: Receiver<Request>) -> Self {
        WorkerNode {
            id: wid,
            request_queue_rx: receiver,
            engine: ReboundEngine::new(circuit),
            client: ReboundClient::new(),
        }
    }

    // Response::from_file(File::open(format!("{}/default.html", std::env::var(REBOUND_SITE_DIR).unwrap())).unwrap()).with_status_code(502)
    pub fn run<F>(&mut self, mut error_provider: F)
    where
        F: FnMut() -> Response<File>,
    {
        for mut conn_req in self.request_queue_rx.iter() {
            info!("{} handling request: {:?}", self.id, conn_req);
            let r = self.engine.get(&mut conn_req);
            match r {
                Some(rebound_req) => {
                    info!("{} sending upstream request: {:?}", self.id, rebound_req);
                    match futures::executor::block_on(self.client.send(rebound_req)) {
                        Ok(rebound_res) => match conn_req.respond(rebound_res.into()) {
                            Ok(_) => info!("{} sent response from rule, finished request", self.id),
                            Err(_) => error!("{} failed to send response from rule", self.id),
                        },

                        Err(_) => match conn_req.respond(error_provider()) {
                            Ok(_) => info!("{} sent error response, finished request", self.id),
                            Err(_) => error!("{} failed to send error response", self.id),
                        },
                    }
                }
                None => match conn_req.respond(error_provider()) {
                    Ok(_) => info!("{} sent default response, finished request", self.id),
                    Err(_) => error!("{} failed to send default response", self.id),
                },
            }
        }
    }
}
