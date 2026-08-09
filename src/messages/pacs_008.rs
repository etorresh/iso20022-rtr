use serde::{Deserialize, Serialize};

use crate::datatypes::{CreditTransferTransaction39__1, GroupHeader93__1};

#[derive(Deserialize, Serialize)]
pub struct Pacs008 {
    group_header: GroupHeader93__1,
    credit_transfer_transaction_information: CreditTransferTransaction39__1,
}
