use std::{io::Result, thread::{self, JoinHandle}};
use flume::{Sender, Receiver};
use log::{info, error};
use tiny_http::{Server, SslConfig, Request};

use crate::conf::{ReboundConf, REBOUND_DEFAULT_WORKER_COUNT, parser::read_ssl_file};

use super::worker::WorkerNode;

/// Master Node for Rebound that controls the whole Server
/// 
pub struct MasterNode {

    ///
    /// 
    #[allow(unused)]
    config: ReboundConf,
    
    ///
    /// 
    server: Server,

    ///
    /// 
    workers: Vec<WorkerNode>,

    ///
    /// 
    request_queue_tx: Sender<Request>,

    ///
    /// 
    #[allow(unused)]
    request_queue_rx: Receiver<Request>,

}

impl MasterNode {
    
    pub fn from(conf: &ReboundConf) -> Result<Self> {
        
        info!("Starting master...");

        let (tx, rx) = flume::unbounded::<Request>();
        let wc = conf.workers.unwrap_or_else(|| REBOUND_DEFAULT_WORKER_COUNT);        
        let workers = (0..wc)
            .map(|n| WorkerNode::from( String::from(format!("worker-{}", n+1)), conf.clone(), rx.clone()))
            .collect();


        let s = match conf.clone().ssl {
            Some(rebound_ssl) => {
                Server::https(

                    format!("{}:{}", conf.host, conf.port),
                    SslConfig
                    {
                        certificate: read_ssl_file(rebound_ssl.pub_cert),
                        private_key: read_ssl_file(rebound_ssl.priv_key)
                    }

                ).unwrap()
            }
            None => Server::http(format!("{}:{}", conf.host, conf.port)).unwrap(),
        };
        info!("master listening on {}:{}", conf.host, conf.port);

        Ok(
            MasterNode {
               config: conf.clone(),
               server: s,
               workers: workers,
               request_queue_tx: tx,
               request_queue_rx: rx
            }
        )
    }

    pub fn run(self) {
        
        let mut worker_handles: Vec<JoinHandle<()>> = Vec::new();
        for w in self.workers {

            info!("Starting {}", w.id);
            let handle: JoinHandle<()> = thread::spawn(move || {
                w.run()
            });

            worker_handles.push(handle)

        }
        
        info!("master ready!");

        // loop until shutdown, implemented as iterator
        for req in self.server.incoming_requests() {

             match self.request_queue_tx.send(req) {
                Ok(_) =>  (),
                Err(e) => error!("Failed to Queue Request, Error: {}", e),
            }
        }

        for handle in worker_handles {
            handle.join().unwrap()
        }
    }

}