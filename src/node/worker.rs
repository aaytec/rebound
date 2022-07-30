use flume::Receiver;
use log::info;
use tiny_http::{Request, Response};

use crate::conf::ReboundConf;

///
/// 
pub struct WorkerNode {

    ///
    /// 
    pub id: String,

    ///
    /// 
    pub conf: ReboundConf,
    
    ///
    /// 
    request_queue_rx: Receiver<Request>
}

///
/// 
impl WorkerNode {

    pub fn from(wid: String, c: ReboundConf, receiver: Receiver<Request>) -> Self {
        WorkerNode { id: wid, conf: c, request_queue_rx: receiver }
    }

    pub fn run(&self) {

        for req in self.request_queue_rx.iter() {

            info!("{} handling request: {:?}", self.id, req);
            req.respond(Response::from_string("success")).unwrap();
        }
    }
}