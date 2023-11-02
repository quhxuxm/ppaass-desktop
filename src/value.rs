use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PpaassConfigurationForm {
    pub agent_port: u16,
    pub proxy_address: String,
    pub user_token: String,
}
