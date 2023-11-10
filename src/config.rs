use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct EnvConfig {
    listen_port: u16,

    listen_ip: Ipv4Addr,

    openai_api_key: String,
    openai_api_base: String,
    openai_api_text_to_speech_model_endpoint: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            listen_port: 3000,
            listen_ip: Ipv4Addr::new(127, 0, 0, 1),
            openai_api_base: "".to_string(),
            openai_api_key: "".to_string(),
            openai_api_text_to_speech_model_endpoint: "".to_string(),
        }
    }
}

impl EnvConfig {
    pub fn sock_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(self.listen_ip), self.listen_port)
    }

    pub fn openai_api_key(&self) -> &str {
        self.openai_api_key.as_str()
    }

    pub fn openai_api_base(&self) -> &str {
        self.openai_api_base.as_str()
    }

    pub fn openai_api_text_to_speech_model_endpoint(&self) -> &str {
        self.openai_api_text_to_speech_model_endpoint.as_str()
    }
}
