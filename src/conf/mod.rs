use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod parser;

/// Rebound Log File
/// 
pub const REBOUND_LOG_DIR: &'static str = "REBOUND_LOG_DIR";

/// Rebound Conf File
/// 
pub const REBOUND_CONF_FILE: &'static str = "REBOUND_CONF_FILE";

/// Rebound Conf File
/// 
pub const REBOUND_DEFAULT_ERROR_FILE: &'static str = "REBOUND_DEFAULT_ERROR_FILE";

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
    pub workers: usize,

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

    /// preserve Http Headers
    /// defaults = true
    #[serde(default = "preserve_hdrs_default")]
    pub preserve_hdrs: bool,

    /// Set Additional Http Headers
    /// 
    #[serde(default)]
    pub additional_hdrs: HashMap<String, String>,

    /// preserve Http Query Params
    /// defaults = true
    #[serde(default = "preserve_query_default")]
    pub preserve_query: bool,

    /// Set Additional Http Query Params
    ///
    #[serde(default)]
    pub additional_query: HashMap<String, String>,

    ///
    /// 
    pub upstream: String

}

fn preserve_hdrs_default() -> bool {true}
fn preserve_query_default() -> bool {true}
