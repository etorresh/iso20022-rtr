mod messages;

use messages::head_001::Head001;
use messages::pacs_002::Pacs002;
use messages::pacs_008::Pacs008;
use serde::{Deserialize, Serialize};
mod datatypes;

#[derive(Deserialize, Serialize)]
pub struct Message {
    business_application_header: Head001,
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
