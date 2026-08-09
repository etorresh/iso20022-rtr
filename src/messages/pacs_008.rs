use serde::{Deserialize, Serialize};

use crate::datatypes::GroupHeader93__1;

#[derive(Deserialize, Serialize)]
pub struct Pacs008 {
    group_header: GroupHeader93__1,
    credit_transfer_transaction_information: (),
}
