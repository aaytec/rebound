use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod parser;

/// Rebound Log File
/// 
pub const REBOUND_LOG_FILE: &'static str = "REBOUND_LOG_FILE";

/// Rebound Conf File
/// 
pub const REBOUND_CONF_FILE: &'static str = "REBOUND_CONF_FILE";

/// Default Number of workers for Rebound Server
/// 
pub const REBOUND_DEFAULT_WORKER_COUNT: usize = 3;

/// Configuration for Rebound Server
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReboundConf {

    /// Host Name of Rebound Server
    /// 
    pub host: String,

    /// Port for Rebound Server
    /// 
    pub port: u16,

    /// Rebound SSL configuration
    /// 
    pub ssl: Option<ReboundSSL>,

    /// Rebound worker count
    /// 
    pub workers: Option<usize>,

    /// Rebound Rules
    /// 
    pub rules: Option<Vec<ReboundRule>>

}


/// SSL configuration for Rebound
/// 
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReboundSSL {
    
    /// File Path for Public Certificate
    /// 
    pub pub_cert: String,

    /// File Path for Private Key
    /// 
    pub priv_key: String

}

/// Rebound Rule
/// 
/// Describe the Rebound rule with a pattern and which proxy location to send to
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReboundRule {

    ///
    /// 
    pub pattern: String,

    /// preserve URI path
    /// defaults = true
    pub preserve_path: bool,

    /// preserve Http Headers
    /// defaults = true
    pub preserve_hdrs: bool,

    /// Set Additional Http Headers
    /// defaults = true
    pub additional_hdrs: Option<HashMap<String, String>>,

    /// preserve Http Query Params
    /// defaults = true
    pub preserve_params: bool,

    ///
    /// 
    pub proxy: String

}
