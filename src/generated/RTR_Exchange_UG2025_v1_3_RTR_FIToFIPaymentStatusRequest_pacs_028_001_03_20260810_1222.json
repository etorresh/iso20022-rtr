/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///Unique and unambiguous identification of a financial institution or a branch of a financial institution.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique and unambiguous identification of a financial institution or a branch of a financial institution.",
///  "type": "object",
///  "required": [
///    "financial_institution_identification"
///  ],
///  "properties": {
///    "financial_institution_identification": {
///      "description": "Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.",
///      "$ref": "#/definitions/FinancialInstitutionIdentification18__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct BranchAndFinancialInstitutionIdentification61 {
    ///Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
    pub financial_institution_identification: FinancialInstitutionIdentification181,
}
impl BranchAndFinancialInstitutionIdentification61 {
    pub fn builder() -> builder::BranchAndFinancialInstitutionIdentification61 {
        Default::default()
    }
}
///Unique identification, as assigned by a clearing system, to unambiguously identify a member of the clearing system.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique identification, as assigned by a clearing system, to unambiguously identify a member of the clearing system.",
///  "type": "object",
///  "required": [
///    "member_identification"
///  ],
///  "properties": {
///    "member_identification": {
///      "description": "Identification of a member of a clearing system.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClearingSystemMemberIdentification21 {
    ///Identification of a member of a clearing system.
    pub member_identification: Max35Text,
}
impl ClearingSystemMemberIdentification21 {
    pub fn builder() -> builder::ClearingSystemMemberIdentification21 {
        Default::default()
    }
}
/**Scope
The FinancialInstitutionToFinancialInstitutionPaymentStatusRequest message is sent by the debtor agent to the creditor agent, directly or through other agents and/or a payment clearing and settlement system. It is used to request a FIToFIPaymentStatusReport message containing information on the status of a previously sent instruction. 
Usage
The FIToFIPaymentStatusRequest message is exchanged between agents to request status information about instructions previously sent. Its usage will always be governed by a bilateral agreement between the agents.
The FIToFIPaymentStatusRequest message can be used to request information about the status (e.g. rejection, acceptance) of a credit transfer instruction, a direct debit instruction, as well as other intra-agent instructions (for example FIToFIPaymentCancellationRequest).
The FIToFIPaymentStatusRequest message refers to the original instruction(s) by means of references only or by means of references and a set of elements from the original instruction.
The FIToFIPaymentStatusRequest message can be used in domestic and cross-border scenarios.

*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Scope\r\nThe FinancialInstitutionToFinancialInstitutionPaymentStatusRequest message is sent by the debtor agent to the creditor agent, directly or through other agents and/or a payment clearing and settlement system. It is used to request a FIToFIPaymentStatusReport message containing information on the status of a previously sent instruction. \r\nUsage\r\nThe FIToFIPaymentStatusRequest message is exchanged between agents to request status information about instructions previously sent. Its usage will always be governed by a bilateral agreement between the agents.\r\nThe FIToFIPaymentStatusRequest message can be used to request information about the status (e.g. rejection, acceptance) of a credit transfer instruction, a direct debit instruction, as well as other intra-agent instructions (for example FIToFIPaymentCancellationRequest).\r\nThe FIToFIPaymentStatusRequest message refers to the original instruction(s) by means of references only or by means of references and a set of elements from the original instruction.\r\nThe FIToFIPaymentStatusRequest message can be used in domestic and cross-border scenarios.\r\n\r\n",
///  "type": "object",
///  "required": [
///    "group_header",
///    "transaction_information"
///  ],
///  "properties": {
///    "group_header": {
///      "description": "Set of characteristics shared by all individual transactions included in the status request message.",
///      "$ref": "#/definitions/GroupHeader91__1"
///    },
///    "transaction_information": {
///      "description": "Information concerning the original transaction, to which the status request message refers.",
///      "$ref": "#/definitions/PaymentTransaction113__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FiToFiPaymentStatusRequestV03 {
    ///Set of characteristics shared by all individual transactions included in the status request message.
    pub group_header: GroupHeader911,
    ///Information concerning the original transaction, to which the status request message refers.
    pub transaction_information: PaymentTransaction1131,
}
impl FiToFiPaymentStatusRequestV03 {
    pub fn builder() -> builder::FiToFiPaymentStatusRequestV03 {
        Default::default()
    }
}
///Specifies the details to identify a financial institution.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the details to identify a financial institution.",
///  "type": "object",
///  "required": [
///    "clearing_system_member_identification"
///  ],
///  "properties": {
///    "clearing_system_member_identification": {
///      "description": "Information used to identify a member within a clearing system.",
///      "$ref": "#/definitions/ClearingSystemMemberIdentification2__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FinancialInstitutionIdentification181 {
    ///Information used to identify a member within a clearing system.
    pub clearing_system_member_identification: ClearingSystemMemberIdentification21,
}
impl FinancialInstitutionIdentification181 {
    pub fn builder() -> builder::FinancialInstitutionIdentification181 {
        Default::default()
    }
}
///Set of characteristics shared by all individual transactions included in the message.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Set of characteristics shared by all individual transactions included in the message.",
///  "type": "object",
///  "required": [
///    "creation_date_time",
///    "message_identification"
///  ],
///  "properties": {
///    "creation_date_time": {
///      "description": "Date and time at which the message was created.",
///      "$ref": "#/definitions/ISONormalisedDateTime"
///    },
///    "message_identification": {
///      "description": "Point to point reference, as assigned by the instructing party, and sent to the next party in the chain to unambiguously identify the message. Usage: The instructing party has to make sure that MessageIdentification is unique per instructed party for a pre-agreed period.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GroupHeader911 {
    ///Date and time at which the message was created.
    pub creation_date_time: IsoNormalisedDateTime,
    ///Point to point reference, as assigned by the instructing party, and sent to the next party in the chain to unambiguously identify the message. Usage: The instructing party has to make sure that MessageIdentification is unique per instructed party for a pre-agreed period.
    pub message_identification: Max35Text,
}
impl GroupHeader911 {
    pub fn builder() -> builder::GroupHeader911 {
        Default::default()
    }
}
///an ISODateTime whereby all timezoned dateTime values are UTC.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "an ISODateTime whereby all timezoned dateTime values are UTC.",
///  "type": "string",
///  "pattern": "^(?:[1-9]\\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\\d|2[0-3]):[0-5]\\d:[0-5]\\d(?:\\.[0-9]+)?(?:Z)$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct IsoNormalisedDateTime(::std::string::String);
impl ::std::ops::Deref for IsoNormalisedDateTime {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IsoNormalisedDateTime> for ::std::string::String {
    fn from(value: IsoNormalisedDateTime) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for IsoNormalisedDateTime {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^(?:[1-9]\\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\\d|2[0-3]):[0-5]\\d:[0-5]\\d(?:\\.[0-9]+)?(?:Z)$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(?:[1-9]\\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\\d|2[0-3]):[0-5]\\d:[0-5]\\d(?:\\.[0-9]+)?(?:Z)$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for IsoNormalisedDateTime {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IsoNormalisedDateTime {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IsoNormalisedDateTime {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for IsoNormalisedDateTime {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Specifies a character string with a maximum length of 35 characters.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a character string with a maximum length of 35 characters.",
///  "type": "string",
///  "maxLength": 35,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Max35Text(::std::string::String);
impl ::std::ops::Deref for Max35Text {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Max35Text> for ::std::string::String {
    fn from(value: Max35Text) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Max35Text {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 35usize {
            return Err("longer than 35 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Max35Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max35Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max35Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Max35Text {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Unique and unambiguous identifier of the group of transactions as assigned by the original instructing party.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique and unambiguous identifier of the group of transactions as assigned by the original instructing party.",
///  "type": "object",
///  "required": [
///    "original_message_identification",
///    "original_message_name_identification"
///  ],
///  "properties": {
///    "original_message_identification": {
///      "description": "Point to point reference assigned by the original instructing party to unambiguously identify the original message.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "original_message_name_identification": {
///      "description": "Specifies the original message name identifier to which the message refers, for example, pacs.003.001.01 or MT103.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OriginalGroupInformation291 {
    ///Point to point reference assigned by the original instructing party to unambiguously identify the original message.
    pub original_message_identification: Max35Text,
    ///Specifies the original message name identifier to which the message refers, for example, pacs.003.001.01 or MT103.
    pub original_message_name_identification: Max35Text,
}
impl OriginalGroupInformation291 {
    pub fn builder() -> builder::OriginalGroupInformation291 {
        Default::default()
    }
}
///Provides further details on the original transactions, to which the status report message refers.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides further details on the original transactions, to which the status report message refers.",
///  "type": "object",
///  "required": [
///    "original_end_to_end_identification",
///    "original_group_information",
///    "original_uetr"
///  ],
///  "properties": {
///    "acceptance_date_time": {
///      "description": "Point in time when the payment order from the initiating party meets the processing conditions of the account servicing agent. This means that the account servicing agent has received the payment order and has applied checks such as authorisation, availability of funds.",
///      "$ref": "#/definitions/ISONormalisedDateTime"
///    },
///    "clearing_system_reference": {
///      "description": "Unique reference, as assigned by a clearing system, to unambiguously identify the instruction.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "instructed_agent": {
///      "description": "Agent that is instructed by the previous party in the chain to carry out the (set of) instruction(s).  ",
///      "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__1"
///    },
///    "instructing_agent": {
///      "description": "Agent that instructs the next party in the chain to carry out the (set of) instruction(s).  ",
///      "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__1"
///    },
///    "original_end_to_end_identification": {
///      "description": "Unique identification, as assigned by the original initiating party, to unambiguously identify the original transaction.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "original_group_information": {
///      "description": "Point to point reference, as assigned by the original instructing party, to unambiguously identify the original message.",
///      "$ref": "#/definitions/OriginalGroupInformation29__1"
///    },
///    "original_instruction_identification": {
///      "description": "Unique identification, as assigned by the original instructing party for the original instructed party, to unambiguously identify the original instruction.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "original_uetr": {
///      "description": "Universally unique identifier to provide the original end-to-end reference of a payment transaction.",
///      "$ref": "#/definitions/UUIDv4Identifier"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PaymentTransaction1131 {
    ///Point in time when the payment order from the initiating party meets the processing conditions of the account servicing agent. This means that the account servicing agent has received the payment order and has applied checks such as authorisation, availability of funds.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub acceptance_date_time: ::std::option::Option<IsoNormalisedDateTime>,
    ///Unique reference, as assigned by a clearing system, to unambiguously identify the instruction.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub clearing_system_reference: ::std::option::Option<Max35Text>,
    ///Agent that is instructed by the previous party in the chain to carry out the (set of) instruction(s).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructed_agent: ::std::option::Option<
        BranchAndFinancialInstitutionIdentification61,
    >,
    ///Agent that instructs the next party in the chain to carry out the (set of) instruction(s).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructing_agent: ::std::option::Option<
        BranchAndFinancialInstitutionIdentification61,
    >,
    ///Unique identification, as assigned by the original initiating party, to unambiguously identify the original transaction.
    pub original_end_to_end_identification: Max35Text,
    ///Point to point reference, as assigned by the original instructing party, to unambiguously identify the original message.
    pub original_group_information: OriginalGroupInformation291,
    ///Unique identification, as assigned by the original instructing party for the original instructed party, to unambiguously identify the original instruction.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub original_instruction_identification: ::std::option::Option<Max35Text>,
    ///Universally unique identifier to provide the original end-to-end reference of a payment transaction.
    pub original_uetr: UuiDv4Identifier,
}
impl PaymentTransaction1131 {
    pub fn builder() -> builder::PaymentTransaction1131 {
        Default::default()
    }
}
///Universally Unique IDentifier (UUID) version 4, as described in IETC RFC 4122 "Universally Unique IDentifier (UUID) URN Namespace".
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Universally Unique IDentifier (UUID) version 4, as described in IETC RFC 4122 \"Universally Unique IDentifier (UUID) URN Namespace\".",
///  "type": "string",
///  "pattern": "^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UuiDv4Identifier(::std::string::String);
impl ::std::ops::Deref for UuiDv4Identifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UuiDv4Identifier> for ::std::string::String {
    fn from(value: UuiDv4Identifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for UuiDv4Identifier {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UuiDv4Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UuiDv4Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UuiDv4Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UuiDv4Identifier {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BranchAndFinancialInstitutionIdentification61 {
        financial_institution_identification: ::std::result::Result<
            super::FinancialInstitutionIdentification181,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BranchAndFinancialInstitutionIdentification61 {
        fn default() -> Self {
            Self {
                financial_institution_identification: Err(
                    "no value supplied for financial_institution_identification"
                        .to_string(),
                ),
            }
        }
    }
    impl BranchAndFinancialInstitutionIdentification61 {
        pub fn financial_institution_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FinancialInstitutionIdentification181>,
            T::Error: ::std::fmt::Display,
        {
            self.financial_institution_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for financial_institution_identification: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<BranchAndFinancialInstitutionIdentification61>
    for super::BranchAndFinancialInstitutionIdentification61 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BranchAndFinancialInstitutionIdentification61,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                financial_institution_identification: value
                    .financial_institution_identification?,
            })
        }
    }
    impl ::std::convert::From<super::BranchAndFinancialInstitutionIdentification61>
    for BranchAndFinancialInstitutionIdentification61 {
        fn from(value: super::BranchAndFinancialInstitutionIdentification61) -> Self {
            Self {
                financial_institution_identification: Ok(
                    value.financial_institution_identification,
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClearingSystemMemberIdentification21 {
        member_identification: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ClearingSystemMemberIdentification21 {
        fn default() -> Self {
            Self {
                member_identification: Err(
                    "no value supplied for member_identification".to_string(),
                ),
            }
        }
    }
    impl ClearingSystemMemberIdentification21 {
        pub fn member_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.member_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for member_identification: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ClearingSystemMemberIdentification21>
    for super::ClearingSystemMemberIdentification21 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClearingSystemMemberIdentification21,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                member_identification: value.member_identification?,
            })
        }
    }
    impl ::std::convert::From<super::ClearingSystemMemberIdentification21>
    for ClearingSystemMemberIdentification21 {
        fn from(value: super::ClearingSystemMemberIdentification21) -> Self {
            Self {
                member_identification: Ok(value.member_identification),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FiToFiPaymentStatusRequestV03 {
        group_header: ::std::result::Result<
            super::GroupHeader911,
            ::std::string::String,
        >,
        transaction_information: ::std::result::Result<
            super::PaymentTransaction1131,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FiToFiPaymentStatusRequestV03 {
        fn default() -> Self {
            Self {
                group_header: Err("no value supplied for group_header".to_string()),
                transaction_information: Err(
                    "no value supplied for transaction_information".to_string(),
                ),
            }
        }
    }
    impl FiToFiPaymentStatusRequestV03 {
        pub fn group_header<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupHeader911>,
            T::Error: ::std::fmt::Display,
        {
            self.group_header = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for group_header: {e}")
                });
            self
        }
        pub fn transaction_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaymentTransaction1131>,
            T::Error: ::std::fmt::Display,
        {
            self.transaction_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for transaction_information: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FiToFiPaymentStatusRequestV03>
    for super::FiToFiPaymentStatusRequestV03 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FiToFiPaymentStatusRequestV03,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_header: value.group_header?,
                transaction_information: value.transaction_information?,
            })
        }
    }
    impl ::std::convert::From<super::FiToFiPaymentStatusRequestV03>
    for FiToFiPaymentStatusRequestV03 {
        fn from(value: super::FiToFiPaymentStatusRequestV03) -> Self {
            Self {
                group_header: Ok(value.group_header),
                transaction_information: Ok(value.transaction_information),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FinancialInstitutionIdentification181 {
        clearing_system_member_identification: ::std::result::Result<
            super::ClearingSystemMemberIdentification21,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FinancialInstitutionIdentification181 {
        fn default() -> Self {
            Self {
                clearing_system_member_identification: Err(
                    "no value supplied for clearing_system_member_identification"
                        .to_string(),
                ),
            }
        }
    }
    impl FinancialInstitutionIdentification181 {
        pub fn clearing_system_member_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ClearingSystemMemberIdentification21>,
            T::Error: ::std::fmt::Display,
        {
            self.clearing_system_member_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for clearing_system_member_identification: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FinancialInstitutionIdentification181>
    for super::FinancialInstitutionIdentification181 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FinancialInstitutionIdentification181,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                clearing_system_member_identification: value
                    .clearing_system_member_identification?,
            })
        }
    }
    impl ::std::convert::From<super::FinancialInstitutionIdentification181>
    for FinancialInstitutionIdentification181 {
        fn from(value: super::FinancialInstitutionIdentification181) -> Self {
            Self {
                clearing_system_member_identification: Ok(
                    value.clearing_system_member_identification,
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupHeader911 {
        creation_date_time: ::std::result::Result<
            super::IsoNormalisedDateTime,
            ::std::string::String,
        >,
        message_identification: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GroupHeader911 {
        fn default() -> Self {
            Self {
                creation_date_time: Err(
                    "no value supplied for creation_date_time".to_string(),
                ),
                message_identification: Err(
                    "no value supplied for message_identification".to_string(),
                ),
            }
        }
    }
    impl GroupHeader911 {
        pub fn creation_date_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IsoNormalisedDateTime>,
            T::Error: ::std::fmt::Display,
        {
            self.creation_date_time = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for creation_date_time: {e}"
                    )
                });
            self
        }
        pub fn message_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.message_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for message_identification: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<GroupHeader911> for super::GroupHeader911 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupHeader911,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                creation_date_time: value.creation_date_time?,
                message_identification: value.message_identification?,
            })
        }
    }
    impl ::std::convert::From<super::GroupHeader911> for GroupHeader911 {
        fn from(value: super::GroupHeader911) -> Self {
            Self {
                creation_date_time: Ok(value.creation_date_time),
                message_identification: Ok(value.message_identification),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OriginalGroupInformation291 {
        original_message_identification: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
        original_message_name_identification: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OriginalGroupInformation291 {
        fn default() -> Self {
            Self {
                original_message_identification: Err(
                    "no value supplied for original_message_identification".to_string(),
                ),
                original_message_name_identification: Err(
                    "no value supplied for original_message_name_identification"
                        .to_string(),
                ),
            }
        }
    }
    impl OriginalGroupInformation291 {
        pub fn original_message_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.original_message_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for original_message_identification: {e}"
                    )
                });
            self
        }
        pub fn original_message_name_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.original_message_name_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for original_message_name_identification: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<OriginalGroupInformation291>
    for super::OriginalGroupInformation291 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OriginalGroupInformation291,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                original_message_identification: value.original_message_identification?,
                original_message_name_identification: value
                    .original_message_name_identification?,
            })
        }
    }
    impl ::std::convert::From<super::OriginalGroupInformation291>
    for OriginalGroupInformation291 {
        fn from(value: super::OriginalGroupInformation291) -> Self {
            Self {
                original_message_identification: Ok(
                    value.original_message_identification,
                ),
                original_message_name_identification: Ok(
                    value.original_message_name_identification,
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaymentTransaction1131 {
        acceptance_date_time: ::std::result::Result<
            ::std::option::Option<super::IsoNormalisedDateTime>,
            ::std::string::String,
        >,
        clearing_system_reference: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        instructed_agent: ::std::result::Result<
            ::std::option::Option<super::BranchAndFinancialInstitutionIdentification61>,
            ::std::string::String,
        >,
        instructing_agent: ::std::result::Result<
            ::std::option::Option<super::BranchAndFinancialInstitutionIdentification61>,
            ::std::string::String,
        >,
        original_end_to_end_identification: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
        original_group_information: ::std::result::Result<
            super::OriginalGroupInformation291,
            ::std::string::String,
        >,
        original_instruction_identification: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        original_uetr: ::std::result::Result<
            super::UuiDv4Identifier,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaymentTransaction1131 {
        fn default() -> Self {
            Self {
                acceptance_date_time: Ok(Default::default()),
                clearing_system_reference: Ok(Default::default()),
                instructed_agent: Ok(Default::default()),
                instructing_agent: Ok(Default::default()),
                original_end_to_end_identification: Err(
                    "no value supplied for original_end_to_end_identification"
                        .to_string(),
                ),
                original_group_information: Err(
                    "no value supplied for original_group_information".to_string(),
                ),
                original_instruction_identification: Ok(Default::default()),
                original_uetr: Err("no value supplied for original_uetr".to_string()),
            }
        }
    }
    impl PaymentTransaction1131 {
        pub fn acceptance_date_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::IsoNormalisedDateTime>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.acceptance_date_time = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for acceptance_date_time: {e}"
                    )
                });
            self
        }
        pub fn clearing_system_reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.clearing_system_reference = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for clearing_system_reference: {e}"
                    )
                });
            self
        }
        pub fn instructed_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::BranchAndFinancialInstitutionIdentification61,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.instructed_agent = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for instructed_agent: {e}")
                });
            self
        }
        pub fn instructing_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::BranchAndFinancialInstitutionIdentification61,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.instructing_agent = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for instructing_agent: {e}")
                });
            self
        }
        pub fn original_end_to_end_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.original_end_to_end_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for original_end_to_end_identification: {e}"
                    )
                });
            self
        }
        pub fn original_group_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OriginalGroupInformation291>,
            T::Error: ::std::fmt::Display,
        {
            self.original_group_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for original_group_information: {e}"
                    )
                });
            self
        }
        pub fn original_instruction_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.original_instruction_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for original_instruction_identification: {e}"
                    )
                });
            self
        }
        pub fn original_uetr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UuiDv4Identifier>,
            T::Error: ::std::fmt::Display,
        {
            self.original_uetr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for original_uetr: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PaymentTransaction1131>
    for super::PaymentTransaction1131 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaymentTransaction1131,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acceptance_date_time: value.acceptance_date_time?,
                clearing_system_reference: value.clearing_system_reference?,
                instructed_agent: value.instructed_agent?,
                instructing_agent: value.instructing_agent?,
                original_end_to_end_identification: value
                    .original_end_to_end_identification?,
                original_group_information: value.original_group_information?,
                original_instruction_identification: value
                    .original_instruction_identification?,
                original_uetr: value.original_uetr?,
            })
        }
    }
    impl ::std::convert::From<super::PaymentTransaction1131> for PaymentTransaction1131 {
        fn from(value: super::PaymentTransaction1131) -> Self {
            Self {
                acceptance_date_time: Ok(value.acceptance_date_time),
                clearing_system_reference: Ok(value.clearing_system_reference),
                instructed_agent: Ok(value.instructed_agent),
                instructing_agent: Ok(value.instructing_agent),
                original_end_to_end_identification: Ok(
                    value.original_end_to_end_identification,
                ),
                original_group_information: Ok(value.original_group_information),
                original_instruction_identification: Ok(
                    value.original_instruction_identification,
                ),
                original_uetr: Ok(value.original_uetr),
            }
        }
    }
}
