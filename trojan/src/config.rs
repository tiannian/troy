use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub remote_addr: String,
    pub remote_port: u16,
    pub password: String,
    pub tls: Tls,
    pub tcp: Tcp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tls {
    pub verify: Option<bool>,
    pub verify_hostname: Option<bool>,
    pub sni: Option<String>,
    pub min_version: Option<String>,
    pub max_version: Option<String>,
    pub alpns: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tcp {
    pub no_delay: Option<bool>,
    pub keep_alive: Option<bool>,
    pub reuse_port: Option<bool>,
    pub bind_device: Option<String>,
}
