use crate::messages::head_001::Head001;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Pacs008 {
    business_application_header: Head001,
}
