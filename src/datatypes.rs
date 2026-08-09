use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct PartyIdentification135 {
    identification: Option<Party38Choice>,
    name: Option<String>,
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

#[derive(Deserialize, Serialize)]
pub struct GroupHeader93__1 {
    message_identification: Max35Text,
    creation_date_time: ISONormalisedDateTime,
    number_of_transactions: Max15NumericText, // must be always 1 but that might be up to the business rules validator. I should get a better understanding of when to enforce at the parser level
    settlement_information: SettlementInstruction7,
}

#[derive(Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct Max15NumericText(String);
impl TryFrom<String> for Max15NumericText {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let len = value.chars().count();
        if len < 1 || len > 15 {
            Err("Text must be between 1 and 15 characters")
        } else if value.bytes().any(|b| !b.is_ascii_digit()) {
            Err("Text must only contain digits from 0 through 9")
        } else {
            Ok(Max15NumericText(value))
        }
    }
}

#[derive(Deserialize, Serialize)]
struct SettlementInstruction7 {
    settlement_method: SettlementMethod1Code,
    clearing_system: ClearingSystem,
}

#[derive(Deserialize, Serialize)]
pub enum SettlementMethod1Code {
    #[serde(rename = "CLRG")]
    ClearingSystem,
    #[serde(rename = "COVE")]
    CoverMethod,
    #[serde(rename = "INDA")]
    InstructedAgent,
    #[serde(rename = "INGA")]
    InstructingAgent,
}

#[derive(Deserialize, Serialize)]
pub struct ClearingSystem {
    code: ExternalCashClearingSystem1Code,
}

// External code sets can be downloaded from www.iso20022.org.
// Type: CodeSet
// https://www.iso20022.org/catalogue/additional-content-messages/external-code-sets
#[derive(Deserialize, Serialize)]
pub enum ExternalCashClearingSystem1Code {
    ABE,
    ACH,
    ACS,
    AIP,
    ART,
    AVP,
    AZM,
    BAP,
    BCC,
    BCE,
    BDS,
    BEL,
    BGN,
    BHS,
    BIS,
    BOF,
    BOJ,
    BRL,
    BSP,
    CAD,
    CAM,
    CBA,
    CBC,
    CBJ,
    CCE,
    CHI,
    CHP,
    CIP,
    CIS,
    COE,
    COI,
    COU,
    DDK,
    DKC,
    EBA,
    ELS,
    EMZ,
    EPM,
    EPN,
    ERP,
    FDA,
    FDN,
    FDW,
    FEY,
    FPS,
    GIS,
    HKL,
    HKS,
    HRK,
    HRM,
    HUF,
    IBP,
    INC,
    IMP,
    JOD,
    KPS,
    LGS,
    LKB,
    LVL,
    LVT,
    LYX,
    MEP,
    MOS,
    MQQ,
    MRS,
    MUP,
    NAM,
    NOC,
    NOR,
    NPP,
    NSS,
    NZE,
    PCH,
    PDS,
    PEG,
    PNS,
    PSA,
    PTR,
    PVE,
    ROL,
    ROS,
    RTG,
    RTP,
    RTR,
    SCL,
    SCP,
    SEC,
    SEU,
    SIC,
    SIP,
    SIT,
    SLB,
    SPG,
    SSK,
    ST2,
    STG,
    TBF,
    TCH,
    TGT,
    THB,
    THN,
    TIS,
    TOP,
    TTD,
    UBE,
    UIS,
    UPI,
    VCS,
    XCT,
    ZEN,
    ZET,
    ZIS,
    ISG,
    NBO,
    ISW,
    I27,
    B27,
    UKD,
    RIX,
    MOC,
    BOK,
    KTS,
    RON,
    TWP,
    SRB,
    RBM,
    ISR,
    NFT,
    RGS,
    LSW,
    REN,
    IBG,
    SGA,
    CIT,
    NAP,
    RSD,
    RSE,
    UGD,
    CBH,
    CBP,
    DZR,
}

#[derive(Deserialize, Serialize)]
pub struct CreditTransferTransaction39__1 {
    payment_identification: PaymentIdentification7,
    payment_type_information: PaymentTypeInformation28,
    interbank_settlement_amount: ActiveOrHistoryCurrencyAndAmount,
    interbank_settlement_date: String,
    charge_bearer: String,
    instructing_agent: Option<BranchAndFinancialInstitutionIdentification6>,
    instructed_agent: Option<BranchAndFinancialInstitutionIdentification6>,
    debtor: PartyIdentification135,
    debtor_account: Option<CashAccount38>,
    debtor_agent: BranchAndFinancialInstitutionIdentification6,
    creditor_agent: BranchAndFinancialInstitutionIdentification6,
    creditor: PartyIdentification135,
    creditor_account: Option<CashAccount38>,
}

#[derive(Deserialize, Serialize)]
pub struct PaymentIdentification7 {
    end_to_end_identification: Max35Text,
    uetr: UUIDv4Identifier,
}

#[derive(Deserialize, Serialize)]
pub struct UUIDv4Identifier(String);

#[derive(Deserialize, Serialize)]
pub struct PaymentTypeInformation28 {
    local_instrument: LocalInstrument2Choice,
}

#[derive(Deserialize, Serialize)]
pub struct LocalInstrument2Choice {
    proprietary: Max35Text,
}

#[derive(Deserialize, Serialize)]
pub struct ActiveOrHistoryCurrencyAndAmount {
    currency: String,
    amount: u64, // custom impl that to stay in u64 since t he constraints are perfectly fine for it vs rust_decimal
}

#[derive(Deserialize, Serialize)]
pub struct CashAccount38 {
    identification: AccountIdentification4Choice,
}
#[derive(Deserialize, Serialize)]
pub struct AccountIdentification4Choice {
    other: GenericAccountIdentification1,
}

#[derive(Deserialize, Serialize)]
pub struct GenericAccountIdentification1 {
    identification: Max34Text,
}

#[derive(Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct Max34Text(String);
impl TryFrom<String> for Max34Text {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let len = value.chars().count();
        if len < 1 || len > 34 {
            Err("Text must be between 1 and 34 characters")
        } else {
            Ok(Max34Text(value))
        }
    }
}
