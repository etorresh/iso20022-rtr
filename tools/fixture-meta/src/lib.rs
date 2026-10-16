use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseKind {
    Iso(String),
    Http(u16),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub request: String,
    pub response: ResponseKind,
}
