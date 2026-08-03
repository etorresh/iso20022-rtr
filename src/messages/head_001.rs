use crate::datatypes::{ISONormalisedDateTime, Max35Text, Party44Choice};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Head001 {
    character_set: Option<()>,
    from: Party44Choice,
    to: Party44Choice,
    business_message_identifier: Max35Text,
    message_definition_identifier: Max35Text,
    business_service: Option<()>,
    market_practice: Option<()>,
    creation_date: ISONormalisedDateTime,
    copy_duplicate: Option<()>,
    possible_duplicate: Option<()>,
    priority: Option<()>,
    signature: Option<()>,
    related: Option<()>,
}
