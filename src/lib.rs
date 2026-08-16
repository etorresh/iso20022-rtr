mod generated;
use generated::*;
mod messages;

use messages::pacs_002::Pacs002;
use serde::{Deserialize, Serialize};

use crate::messages::pacs_008::Pacs008;

mod datatypes;

#[derive(Deserialize, Serialize)]
pub struct Message {
    business_application_header: head_001::BusinessApplicationHeaderV02,
    #[serde(flatten)]
    pub payload: MessageType,
}

#[derive(Deserialize, Serialize)]
pub enum MessageType {
    #[serde(rename = "fi_to_fi_customer_credit_transfer")]
    Pacs008(Pacs008),
    #[serde(rename = "fi_to_fi_payment_status_report")]
    Pacs002(Pacs002),
}
