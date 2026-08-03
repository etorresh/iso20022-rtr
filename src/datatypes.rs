use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PartyIdentification135 {
    identification: Party38Choice,
}

#[derive(Deserialize, Serialize)]
pub struct Party38Choice {
    organisation_identification: OrganizationIdentification29,
}

#[derive(Deserialize, Serialize)]
pub struct OrganizationIdentification29 {
    other: GenericOrganisationIdentification1,
}

#[derive(Deserialize, Serialize)]
pub struct GenericOrganisationIdentification1 {
    identification: Max35Text,
}

#[derive(Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct Max35Text(String);
impl TryFrom<String> for Max35Text {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let len = value.chars().count();
        if len < 1 || len > 35 {
            Err("Text must be between 1 and 35 characters")
        } else {
            Ok(Max35Text(value))
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Party44Choice {
    OrganisationIdentification(PartyIdentification135),
    FinancialInstitutionIdentification(BranchAndFinancialInstitutionIdentification6),
}

#[derive(Deserialize, Serialize)]
pub struct BranchAndFinancialInstitutionIdentification6 {
    financial_institution_identification: FinancialInstitutionIdentification18,
}
#[derive(Deserialize, Serialize)]
pub struct FinancialInstitutionIdentification18 {
    clearing_system_member_identification: ClearingSystemMemberIdentification2,
}

#[derive(Deserialize, Serialize)]
pub struct ClearingSystemMemberIdentification2 {
    member_identification: Max35Text,
}

#[derive(Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct ISONormalisedDateTime(String);

impl TryFrom<String> for ISONormalisedDateTime {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.ends_with("Z") {
            return Err("Timestamp format must end with 'Z'".to_string());
        }

        DateTime::parse_from_rfc3339(&value)
            .map_err(|err| format!("Invalid calendar datetime: {err}"))?;

        Ok(ISONormalisedDateTime(value))
    }
}
