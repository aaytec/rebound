use std::{io::Result, thread::{self, JoinHandle}, fs::File, env};
use flume::{Sender, Receiver};
use log::{info, error};
use tiny_http::{Server, SslConfig, Request, Response};

use crate::conf::{ReboundConf, parser::read_ssl_file, REBOUND_DEFAULT_ERROR_FILE};

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
        
        info!("starting master...");

        let (tx, rx) = flume::unbounded::<Request>();
        let wc = conf.workers;        
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
        for mut w in self.workers {

            info!("starting {}", w.id);
            let handle: JoinHandle<()> = thread::spawn(move || {
                w
                    .run(|| {
                        Response
                            ::from_file(File::open(env::var(REBOUND_DEFAULT_ERROR_FILE).unwrap()).unwrap())
                            .with_status_code(502)

                    });
                info!("shutting down {}", w.id);
            });

            worker_handles.push(handle)

        }
        
        info!("master ready!");

        // loop until shutdown, implemented as iterator
        for req in self.server.incoming_requests() {

             match self.request_queue_tx.send(req) {
                Ok(_) =>  (),
                Err(e) => error!("failed to queue request, error: {}", e),
            }
        }

        for handle in worker_handles {
            handle.join().unwrap()
        }
    }

}