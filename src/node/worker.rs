
use flume::Receiver;
use log::info;
use tiny_http::Request;
use futures::executor;

use crate::conf::ReboundConf;
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
    engine: ReboundEngine
}

///
/// 
impl WorkerNode {

    pub fn from(wid: String, c: ReboundConf, receiver: Receiver<Request>) -> Self {
        WorkerNode { 
            id: wid,
            request_queue_rx: receiver,
            engine: ReboundEngine::new(c.rules.unwrap_or_else(|| Vec::new()))
        }
    }

    pub fn run(&mut self) {

        for mut req in self.request_queue_rx.iter() {

            info!("{} handling request: {:?}", self.id, req);
            executor::block_on(self.engine.rebound(req));
        }
    }
}