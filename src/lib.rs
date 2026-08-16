mod datatypes;
mod generated;

use generated::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message {
    business_application_header: head_001::BusinessApplicationHeaderV02,
    #[serde(flatten)]
    pub payload: MessageType,
}

#[derive(Deserialize, Serialize)]
pub enum MessageType {
    #[serde(rename = "fi_to_fi_customer_credit_transfer")]
    Pacs008(pacs_008::FiToFiCustomerCreditTransferV08),
    #[serde(rename = "fi_to_fi_payment_status_report")]
    Pacs002(pacs_002::FiToFiPaymentStatusReportV10),
}
