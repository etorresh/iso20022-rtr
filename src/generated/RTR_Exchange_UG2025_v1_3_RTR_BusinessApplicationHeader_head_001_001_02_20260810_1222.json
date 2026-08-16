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
///      "$ref": "#/definitions/FinancialInstitutionIdentification18__2"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct BranchAndFinancialInstitutionIdentification62 {
    ///Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
    pub financial_institution_identification: FinancialInstitutionIdentification182,
}
impl BranchAndFinancialInstitutionIdentification62 {
    pub fn builder() -> builder::BranchAndFinancialInstitutionIdentification62 {
        Default::default()
    }
}
/**Specifies the Business Application Header of the Business Message.
Can be used when replying to a query; can also be used when canceling or amending.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the Business Application Header of the Business Message.\r\nCan be used when replying to a query; can also be used when canceling or amending.",
///  "type": "object",
///  "required": [
///    "business_message_identifier",
///    "creation_date",
///    "from",
///    "message_definition_identifier",
///    "to"
///  ],
///  "properties": {
///    "business_message_identifier": {
///      "description": "Unambiguously identifies the Business Message to the MessagingEndpoint that has created the Business Message.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "copy_duplicate": {
///      "description": "Indicates whether the message is a Copy, a Duplicate or a copy of a duplicate of a previously sent ISO 20022 Message.",
///      "$ref": "#/definitions/CopyDuplicate1Code__1"
///    },
///    "creation_date": {
///      "description": "Date and time when this Business Message (header) was created. Note Times must be normalized, using the \"Z\" annotation.",
///      "$ref": "#/definitions/ISONormalisedDateTime"
///    },
///    "from": {
///      "description": "The sending MessagingEndpoint that has created this Business Message for the receiving MessagingEndpoint that will process this Business Message.  Note\tthe sending MessagingEndpoint might be different from the sending address potentially contained in the transport header (as defined in the transport layer).",
///      "$ref": "#/definitions/Party44Choice__1"
///    },
///    "message_definition_identifier": {
///      "description": "Contains the MessageIdentifier that defines the BusinessMessage. It must contain a MessageIdentifier published on the ISO 20022 website.  example\tcamt.001.001.03.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "possible_duplicate": {
///      "description": "Flag indicating if the Business Message exchanged between the MessagingEndpoints is possibly a duplicate.  If the receiving MessagingEndpoint did not receive the original, then this Business Message should be processed as if it were the original.   If the receiving MessagingEndpoint did receive the original, then it should perform necessary actions to avoid processing this Business Message again.  This will guarantee business idempotent behaviour.  NOTE: this is named \"PossResend\" in FIX - this is an application level resend not a network level retransmission.",
///      "$ref": "#/definitions/YesNoIndicator"
///    },
///    "signature": {
///      "description": "Contains the digital signature of the Business Entity authorised to sign this Business Message.",
///      "$ref": "#/definitions/SignatureEnvelope"
///    },
///    "to": {
///      "description": "The MessagingEndpoint designated by the sending MessagingEndpoint to be the recipient who will ultimately process this Business Message.  Note the receiving MessagingEndpoint might be different from the receiving address potentially contained in the transport header (as defined in the transport layer).",
///      "$ref": "#/definitions/Party44Choice__2"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct BusinessApplicationHeader51 {
    ///Unambiguously identifies the Business Message to the MessagingEndpoint that has created the Business Message.
    pub business_message_identifier: Max35Text,
    ///Indicates whether the message is a Copy, a Duplicate or a copy of a duplicate of a previously sent ISO 20022 Message.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub copy_duplicate: ::std::option::Option<CopyDuplicate1Code1>,
    ///Date and time when this Business Message (header) was created. Note Times must be normalized, using the "Z" annotation.
    pub creation_date: IsoNormalisedDateTime,
    ///The sending MessagingEndpoint that has created this Business Message for the receiving MessagingEndpoint that will process this Business Message.  Note	the sending MessagingEndpoint might be different from the sending address potentially contained in the transport header (as defined in the transport layer).
    pub from: Party44Choice1,
    ///Contains the MessageIdentifier that defines the BusinessMessage. It must contain a MessageIdentifier published on the ISO 20022 website.  example	camt.001.001.03.
    pub message_definition_identifier: Max35Text,
    ///Flag indicating if the Business Message exchanged between the MessagingEndpoints is possibly a duplicate.  If the receiving MessagingEndpoint did not receive the original, then this Business Message should be processed as if it were the original.   If the receiving MessagingEndpoint did receive the original, then it should perform necessary actions to avoid processing this Business Message again.  This will guarantee business idempotent behaviour.  NOTE: this is named "PossResend" in FIX - this is an application level resend not a network level retransmission.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub possible_duplicate: ::std::option::Option<YesNoIndicator>,
    ///Contains the digital signature of the Business Entity authorised to sign this Business Message.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub signature: ::std::option::Option<SignatureEnvelope>,
    ///The MessagingEndpoint designated by the sending MessagingEndpoint to be the recipient who will ultimately process this Business Message.  Note the receiving MessagingEndpoint might be different from the receiving address potentially contained in the transport header (as defined in the transport layer).
    pub to: Party44Choice2,
}
impl BusinessApplicationHeader51 {
    pub fn builder() -> builder::BusinessApplicationHeader51 {
        Default::default()
    }
}
/**The Business Layer deals with Business Messages. The behaviour of the Business Messages is fully described by the Business Transaction and the structure of the Business Messages is fully described by the Message Definitions and related Message Rules, Rules and Market Practices. All of which are registered in the ISO 20022 Repository.
A single new Business Message (with its accompagnying business application header) is created - by the sending MessagingEndpoint - for each business event; that is each interaction in a Business Transaction. A Business Message adheres to the following principles:
" A Business Message (and its business application header) must not contain information about the Message Transport System or the mechanics or mechanism of message sending, transportation, or receipt. 
" A Business Message must be comprehensible outside of the context of the Transport Message. That is the Business Message must not require knowledge of the Transport Message to be understood.
" A Business Message may contain headers, footers, and envelopes that are meaningful for the business. When present, they are treated as any other message content, which means that they are considered part of the Message Definition of the Business Message and as such will be part of the ISO 20022 Repository.
" A Business Message refers to Business Actors by their Name. Each instance of a Business Actor has one Name. The Business Actor must not be referred to in the Transport Layer.
Specific usage of this BusinessMessageHeader may be defined by the relevant SEG.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The Business Layer deals with Business Messages. The behaviour of the Business Messages is fully described by the Business Transaction and the structure of the Business Messages is fully described by the Message Definitions and related Message Rules, Rules and Market Practices. All of which are registered in the ISO 20022 Repository.\r\nA single new Business Message (with its accompagnying business application header) is created - by the sending MessagingEndpoint - for each business event; that is each interaction in a Business Transaction. A Business Message adheres to the following principles:\r\n\" A Business Message (and its business application header) must not contain information about the Message Transport System or the mechanics or mechanism of message sending, transportation, or receipt. \r\n\" A Business Message must be comprehensible outside of the context of the Transport Message. That is the Business Message must not require knowledge of the Transport Message to be understood.\r\n\" A Business Message may contain headers, footers, and envelopes that are meaningful for the business. When present, they are treated as any other message content, which means that they are considered part of the Message Definition of the Business Message and as such will be part of the ISO 20022 Repository.\r\n\" A Business Message refers to Business Actors by their Name. Each instance of a Business Actor has one Name. The Business Actor must not be referred to in the Transport Layer.\r\nSpecific usage of this BusinessMessageHeader may be defined by the relevant SEG.",
///  "type": "object",
///  "required": [
///    "business_message_identifier",
///    "creation_date",
///    "from",
///    "message_definition_identifier",
///    "to"
///  ],
///  "properties": {
///    "business_message_identifier": {
///      "description": "Unambiguously identifies the Business Message to the MessagingEndpoint that has created the Business Message.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "copy_duplicate": {
///      "description": "Indicates whether the message is a Copy, a Duplicate or a copy of a duplicate of a previously sent ISO 20022 Message.",
///      "$ref": "#/definitions/CopyDuplicate1Code__1"
///    },
///    "creation_date": {
///      "description": "Date and time when this Business Message (header) was created.",
///      "$ref": "#/definitions/ISONormalisedDateTime"
///    },
///    "from": {
///      "description": "The sending MessagingEndpoint that has created this Business Message for the receiving MessagingEndpoint that will process this Business Message.  Note\tthe sending MessagingEndpoint might be different from the sending address potentially contained in the transport header (as defined in the transport layer).",
///      "$ref": "#/definitions/Party44Choice__1"
///    },
///    "message_definition_identifier": {
///      "description": "Contains the MessageIdentifier that defines the BusinessMessage. It must contain a MessageIdentifier published on the ISO 20022 website.  example\tcamt.001.001.03.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "possible_duplicate": {
///      "description": "Flag indicating if the Business Message exchanged between the MessagingEndpoints is possibly a duplicate.  If the receiving MessagingEndpoint did not receive the original, then this Business Message should be processed as if it were the original.   If the receiving MessagingEndpoint did receive the original, then it should perform necessary actions to avoid processing this Business Message again.  This will guarantee business idempotent behaviour.  NOTE: this is named \"PossResend\" in FIX - this is an application level resend not a network level retransmission.",
///      "$ref": "#/definitions/YesNoIndicator"
///    },
///    "related": {
///      "description": "Specifies the Business Application Header(s) of the Business Message(s) to which this Business Message relates. Can be used when replying to a query; can also be used when canceling or amending.",
///      "$ref": "#/definitions/BusinessApplicationHeader5__1"
///    },
///    "signature": {
///      "description": "Contains the digital signature of the Business Entity authorised to sign this Business Message.",
///      "$ref": "#/definitions/SignatureEnvelope"
///    },
///    "to": {
///      "description": "The MessagingEndpoint designated by the sending MessagingEndpoint to be the recipient who will ultimately process this Business Message.  Note the receiving MessagingEndpoint might be different from the receiving address potentially contained in the transport header (as defined in the transport layer).",
///      "$ref": "#/definitions/Party44Choice__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct BusinessApplicationHeaderV02 {
    ///Unambiguously identifies the Business Message to the MessagingEndpoint that has created the Business Message.
    pub business_message_identifier: Max35Text,
    ///Indicates whether the message is a Copy, a Duplicate or a copy of a duplicate of a previously sent ISO 20022 Message.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub copy_duplicate: ::std::option::Option<CopyDuplicate1Code1>,
    ///Date and time when this Business Message (header) was created.
    pub creation_date: IsoNormalisedDateTime,
    ///The sending MessagingEndpoint that has created this Business Message for the receiving MessagingEndpoint that will process this Business Message.  Note	the sending MessagingEndpoint might be different from the sending address potentially contained in the transport header (as defined in the transport layer).
    pub from: Party44Choice1,
    ///Contains the MessageIdentifier that defines the BusinessMessage. It must contain a MessageIdentifier published on the ISO 20022 website.  example	camt.001.001.03.
    pub message_definition_identifier: Max35Text,
    ///Flag indicating if the Business Message exchanged between the MessagingEndpoints is possibly a duplicate.  If the receiving MessagingEndpoint did not receive the original, then this Business Message should be processed as if it were the original.   If the receiving MessagingEndpoint did receive the original, then it should perform necessary actions to avoid processing this Business Message again.  This will guarantee business idempotent behaviour.  NOTE: this is named "PossResend" in FIX - this is an application level resend not a network level retransmission.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub possible_duplicate: ::std::option::Option<YesNoIndicator>,
    ///Specifies the Business Application Header(s) of the Business Message(s) to which this Business Message relates. Can be used when replying to a query; can also be used when canceling or amending.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub related: ::std::option::Option<BusinessApplicationHeader51>,
    ///Contains the digital signature of the Business Entity authorised to sign this Business Message.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub signature: ::std::option::Option<SignatureEnvelope>,
    ///The MessagingEndpoint designated by the sending MessagingEndpoint to be the recipient who will ultimately process this Business Message.  Note the receiving MessagingEndpoint might be different from the receiving address potentially contained in the transport header (as defined in the transport layer).
    pub to: Party44Choice1,
}
impl BusinessApplicationHeaderV02 {
    pub fn builder() -> builder::BusinessApplicationHeaderV02 {
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
/**Specifies if this document is a copy, a duplicate, or a duplicate of a copy.
*`COPY`-Message is being sent as a copy to a party other than the account owner, for information purposes.
*`DUPL`-Message is for information/confirmation purposes. It is a duplicate of a message previously sent.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies if this document is a copy, a duplicate, or a duplicate of a copy.\n*`COPY`-Message is being sent as a copy to a party other than the account owner, for information purposes.\n*`DUPL`-Message is for information/confirmation purposes. It is a duplicate of a message previously sent.",
///  "type": "string",
///  "enum": [
///    "COPY",
///    "DUPL"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CopyDuplicate1Code1 {
    #[serde(rename = "COPY")]
    Copy,
    #[serde(rename = "DUPL")]
    Dupl,
}
impl ::std::fmt::Display for CopyDuplicate1Code1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Copy => f.write_str("COPY"),
            Self::Dupl => f.write_str("DUPL"),
        }
    }
}
impl ::std::str::FromStr for CopyDuplicate1Code1 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "COPY" => Ok(Self::Copy),
            "DUPL" => Ok(Self::Dupl),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CopyDuplicate1Code1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CopyDuplicate1Code1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CopyDuplicate1Code1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
///Specifies the details to identify a financial institution.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the details to identify a financial institution.",
///  "type": "object",
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
pub struct FinancialInstitutionIdentification182 {
    ///Information used to identify a member within a clearing system.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub clearing_system_member_identification: ::std::option::Option<
        ClearingSystemMemberIdentification21,
    >,
}
impl ::std::default::Default for FinancialInstitutionIdentification182 {
    fn default() -> Self {
        Self {
            clearing_system_member_identification: Default::default(),
        }
    }
}
impl FinancialInstitutionIdentification182 {
    pub fn builder() -> builder::FinancialInstitutionIdentification182 {
        Default::default()
    }
}
///Information related to an identification of an organisation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information related to an identification of an organisation.",
///  "type": "object",
///  "required": [
///    "identification"
///  ],
///  "properties": {
///    "identification": {
///      "description": "Identification assigned by an institution.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GenericOrganisationIdentification11 {
    ///Identification assigned by an institution.
    pub identification: Max35Text,
}
impl GenericOrganisationIdentification11 {
    pub fn builder() -> builder::GenericOrganisationIdentification11 {
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
///Unique and unambiguous way to identify an organisation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique and unambiguous way to identify an organisation.",
///  "type": "object",
///  "required": [
///    "other"
///  ],
///  "properties": {
///    "other": {
///      "description": "Unique identification of an organisation, as assigned by an institution, using an identification scheme.",
///      "$ref": "#/definitions/GenericOrganisationIdentification1__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OrganisationIdentification291 {
    ///Unique identification of an organisation, as assigned by an institution, using an identification scheme.
    pub other: GenericOrganisationIdentification11,
}
impl OrganisationIdentification291 {
    pub fn builder() -> builder::OrganisationIdentification291 {
        Default::default()
    }
}
///Nature or use of the account.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Nature or use of the account.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "organisation_identification"
///      ],
///      "properties": {
///        "organisation_identification": {
///          "description": "Unique and unambiguous way to identify an organisation.",
///          "$ref": "#/definitions/OrganisationIdentification29__1"
///        }
///      },
///      "additionalProperties": false
///    }
///  ],
///  "additionalProperties": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Party38Choice1 {
    ///Unique and unambiguous way to identify an organisation.
    pub organisation_identification: OrganisationIdentification291,
}
impl Party38Choice1 {
    pub fn builder() -> builder::Party38Choice1 {
        Default::default()
    }
}
///Identification of a person, an organisation or a financial institution.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Identification of a person, an organisation or a financial institution.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "organisation_identification"
///      ],
///      "properties": {
///        "organisation_identification": {
///          "description": "Identification of a person or an organisation.",
///          "$ref": "#/definitions/PartyIdentification135__1"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "financial_institution_identification"
///      ],
///      "properties": {
///        "financial_institution_identification": {
///          "description": "Identification of a financial institution.",
///          "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__1"
///        }
///      },
///      "additionalProperties": false
///    }
///  ],
///  "additionalProperties": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum Party44Choice1 {
    #[serde(rename = "organisation_identification")]
    OrganisationIdentification(PartyIdentification1351),
    #[serde(rename = "financial_institution_identification")]
    FinancialInstitutionIdentification(BranchAndFinancialInstitutionIdentification61),
}
impl ::std::convert::From<PartyIdentification1351> for Party44Choice1 {
    fn from(value: PartyIdentification1351) -> Self {
        Self::OrganisationIdentification(value)
    }
}
impl ::std::convert::From<BranchAndFinancialInstitutionIdentification61>
for Party44Choice1 {
    fn from(value: BranchAndFinancialInstitutionIdentification61) -> Self {
        Self::FinancialInstitutionIdentification(value)
    }
}
///Identification of a person, an organisation or a financial institution.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Identification of a person, an organisation or a financial institution.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "organisation_identification"
///      ],
///      "properties": {
///        "organisation_identification": {
///          "description": "Identification of a person or an organisation.",
///          "$ref": "#/definitions/PartyIdentification135__1"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "financial_institution_identification"
///      ],
///      "properties": {
///        "financial_institution_identification": {
///          "description": "Identification of a financial institution.",
///          "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__2"
///        }
///      },
///      "additionalProperties": false
///    }
///  ],
///  "additionalProperties": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum Party44Choice2 {
    #[serde(rename = "organisation_identification")]
    OrganisationIdentification(PartyIdentification1351),
    #[serde(rename = "financial_institution_identification")]
    FinancialInstitutionIdentification(BranchAndFinancialInstitutionIdentification62),
}
impl ::std::convert::From<PartyIdentification1351> for Party44Choice2 {
    fn from(value: PartyIdentification1351) -> Self {
        Self::OrganisationIdentification(value)
    }
}
impl ::std::convert::From<BranchAndFinancialInstitutionIdentification62>
for Party44Choice2 {
    fn from(value: BranchAndFinancialInstitutionIdentification62) -> Self {
        Self::FinancialInstitutionIdentification(value)
    }
}
///Specifies the identification of a person or an organisation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the identification of a person or an organisation.",
///  "type": "object",
///  "required": [
///    "identification"
///  ],
///  "properties": {
///    "identification": {
///      "description": "Unique and unambiguous identification of a party.",
///      "$ref": "#/definitions/Party38Choice__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PartyIdentification1351 {
    ///Unique and unambiguous identification of a party.
    pub identification: Party38Choice1,
}
impl PartyIdentification1351 {
    pub fn builder() -> builder::PartyIdentification1351 {
        Default::default()
    }
}
/**The W3C XML Schema that specifies following standard signature:
 XML Signature Syntax and Processing (Second Edition) W3C Recommendation 10 June 2008
http://www.w3.org/TR/2008/REC-xmldsig-core-20080610/.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The W3C XML Schema that specifies following standard signature:\r\n XML Signature Syntax and Processing (Second Edition) W3C Recommendation 10 June 2008\r\nhttp://www.w3.org/TR/2008/REC-xmldsig-core-20080610/.",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SignatureEnvelope(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for SignatureEnvelope {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<SignatureEnvelope>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: SignatureEnvelope) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for SignatureEnvelope {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///Indicates a "Yes" or "No" type of answer for an element.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Indicates a \"Yes\" or \"No\" type of answer for an element.",
///  "type": "boolean"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct YesNoIndicator(pub bool);
impl ::std::ops::Deref for YesNoIndicator {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<YesNoIndicator> for bool {
    fn from(value: YesNoIndicator) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for YesNoIndicator {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for YesNoIndicator {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for YesNoIndicator {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for YesNoIndicator {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for YesNoIndicator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
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
    pub struct BranchAndFinancialInstitutionIdentification62 {
        financial_institution_identification: ::std::result::Result<
            super::FinancialInstitutionIdentification182,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BranchAndFinancialInstitutionIdentification62 {
        fn default() -> Self {
            Self {
                financial_institution_identification: Err(
                    "no value supplied for financial_institution_identification"
                        .to_string(),
                ),
            }
        }
    }
    impl BranchAndFinancialInstitutionIdentification62 {
        pub fn financial_institution_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FinancialInstitutionIdentification182>,
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
    impl ::std::convert::TryFrom<BranchAndFinancialInstitutionIdentification62>
    for super::BranchAndFinancialInstitutionIdentification62 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BranchAndFinancialInstitutionIdentification62,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                financial_institution_identification: value
                    .financial_institution_identification?,
            })
        }
    }
    impl ::std::convert::From<super::BranchAndFinancialInstitutionIdentification62>
    for BranchAndFinancialInstitutionIdentification62 {
        fn from(value: super::BranchAndFinancialInstitutionIdentification62) -> Self {
            Self {
                financial_institution_identification: Ok(
                    value.financial_institution_identification,
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BusinessApplicationHeader51 {
        business_message_identifier: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
        copy_duplicate: ::std::result::Result<
            ::std::option::Option<super::CopyDuplicate1Code1>,
            ::std::string::String,
        >,
        creation_date: ::std::result::Result<
            super::IsoNormalisedDateTime,
            ::std::string::String,
        >,
        from: ::std::result::Result<super::Party44Choice1, ::std::string::String>,
        message_definition_identifier: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
        possible_duplicate: ::std::result::Result<
            ::std::option::Option<super::YesNoIndicator>,
            ::std::string::String,
        >,
        signature: ::std::result::Result<
            ::std::option::Option<super::SignatureEnvelope>,
            ::std::string::String,
        >,
        to: ::std::result::Result<super::Party44Choice2, ::std::string::String>,
    }
    impl ::std::default::Default for BusinessApplicationHeader51 {
        fn default() -> Self {
            Self {
                business_message_identifier: Err(
                    "no value supplied for business_message_identifier".to_string(),
                ),
                copy_duplicate: Ok(Default::default()),
                creation_date: Err("no value supplied for creation_date".to_string()),
                from: Err("no value supplied for from".to_string()),
                message_definition_identifier: Err(
                    "no value supplied for message_definition_identifier".to_string(),
                ),
                possible_duplicate: Ok(Default::default()),
                signature: Ok(Default::default()),
                to: Err("no value supplied for to".to_string()),
            }
        }
    }
    impl BusinessApplicationHeader51 {
        pub fn business_message_identifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.business_message_identifier = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for business_message_identifier: {e}"
                    )
                });
            self
        }
        pub fn copy_duplicate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CopyDuplicate1Code1>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.copy_duplicate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for copy_duplicate: {e}")
                });
            self
        }
        pub fn creation_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IsoNormalisedDateTime>,
            T::Error: ::std::fmt::Display,
        {
            self.creation_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for creation_date: {e}")
                });
            self
        }
        pub fn from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Party44Choice1>,
            T::Error: ::std::fmt::Display,
        {
            self.from = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for from: {e}"));
            self
        }
        pub fn message_definition_identifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.message_definition_identifier = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for message_definition_identifier: {e}"
                    )
                });
            self
        }
        pub fn possible_duplicate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::YesNoIndicator>>,
            T::Error: ::std::fmt::Display,
        {
            self.possible_duplicate = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for possible_duplicate: {e}"
                    )
                });
            self
        }
        pub fn signature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SignatureEnvelope>>,
            T::Error: ::std::fmt::Display,
        {
            self.signature = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for signature: {e}")
                });
            self
        }
        pub fn to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Party44Choice2>,
            T::Error: ::std::fmt::Display,
        {
            self.to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BusinessApplicationHeader51>
    for super::BusinessApplicationHeader51 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BusinessApplicationHeader51,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                business_message_identifier: value.business_message_identifier?,
                copy_duplicate: value.copy_duplicate?,
                creation_date: value.creation_date?,
                from: value.from?,
                message_definition_identifier: value.message_definition_identifier?,
                possible_duplicate: value.possible_duplicate?,
                signature: value.signature?,
                to: value.to?,
            })
        }
    }
    impl ::std::convert::From<super::BusinessApplicationHeader51>
    for BusinessApplicationHeader51 {
        fn from(value: super::BusinessApplicationHeader51) -> Self {
            Self {
                business_message_identifier: Ok(value.business_message_identifier),
                copy_duplicate: Ok(value.copy_duplicate),
                creation_date: Ok(value.creation_date),
                from: Ok(value.from),
                message_definition_identifier: Ok(value.message_definition_identifier),
                possible_duplicate: Ok(value.possible_duplicate),
                signature: Ok(value.signature),
                to: Ok(value.to),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BusinessApplicationHeaderV02 {
        business_message_identifier: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
        copy_duplicate: ::std::result::Result<
            ::std::option::Option<super::CopyDuplicate1Code1>,
            ::std::string::String,
        >,
        creation_date: ::std::result::Result<
            super::IsoNormalisedDateTime,
            ::std::string::String,
        >,
        from: ::std::result::Result<super::Party44Choice1, ::std::string::String>,
        message_definition_identifier: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
        possible_duplicate: ::std::result::Result<
            ::std::option::Option<super::YesNoIndicator>,
            ::std::string::String,
        >,
        related: ::std::result::Result<
            ::std::option::Option<super::BusinessApplicationHeader51>,
            ::std::string::String,
        >,
        signature: ::std::result::Result<
            ::std::option::Option<super::SignatureEnvelope>,
            ::std::string::String,
        >,
        to: ::std::result::Result<super::Party44Choice1, ::std::string::String>,
    }
    impl ::std::default::Default for BusinessApplicationHeaderV02 {
        fn default() -> Self {
            Self {
                business_message_identifier: Err(
                    "no value supplied for business_message_identifier".to_string(),
                ),
                copy_duplicate: Ok(Default::default()),
                creation_date: Err("no value supplied for creation_date".to_string()),
                from: Err("no value supplied for from".to_string()),
                message_definition_identifier: Err(
                    "no value supplied for message_definition_identifier".to_string(),
                ),
                possible_duplicate: Ok(Default::default()),
                related: Ok(Default::default()),
                signature: Ok(Default::default()),
                to: Err("no value supplied for to".to_string()),
            }
        }
    }
    impl BusinessApplicationHeaderV02 {
        pub fn business_message_identifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.business_message_identifier = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for business_message_identifier: {e}"
                    )
                });
            self
        }
        pub fn copy_duplicate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CopyDuplicate1Code1>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.copy_duplicate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for copy_duplicate: {e}")
                });
            self
        }
        pub fn creation_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IsoNormalisedDateTime>,
            T::Error: ::std::fmt::Display,
        {
            self.creation_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for creation_date: {e}")
                });
            self
        }
        pub fn from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Party44Choice1>,
            T::Error: ::std::fmt::Display,
        {
            self.from = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for from: {e}"));
            self
        }
        pub fn message_definition_identifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.message_definition_identifier = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for message_definition_identifier: {e}"
                    )
                });
            self
        }
        pub fn possible_duplicate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::YesNoIndicator>>,
            T::Error: ::std::fmt::Display,
        {
            self.possible_duplicate = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for possible_duplicate: {e}"
                    )
                });
            self
        }
        pub fn related<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::BusinessApplicationHeader51>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.related = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for related: {e}")
                });
            self
        }
        pub fn signature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SignatureEnvelope>>,
            T::Error: ::std::fmt::Display,
        {
            self.signature = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for signature: {e}")
                });
            self
        }
        pub fn to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Party44Choice1>,
            T::Error: ::std::fmt::Display,
        {
            self.to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BusinessApplicationHeaderV02>
    for super::BusinessApplicationHeaderV02 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BusinessApplicationHeaderV02,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                business_message_identifier: value.business_message_identifier?,
                copy_duplicate: value.copy_duplicate?,
                creation_date: value.creation_date?,
                from: value.from?,
                message_definition_identifier: value.message_definition_identifier?,
                possible_duplicate: value.possible_duplicate?,
                related: value.related?,
                signature: value.signature?,
                to: value.to?,
            })
        }
    }
    impl ::std::convert::From<super::BusinessApplicationHeaderV02>
    for BusinessApplicationHeaderV02 {
        fn from(value: super::BusinessApplicationHeaderV02) -> Self {
            Self {
                business_message_identifier: Ok(value.business_message_identifier),
                copy_duplicate: Ok(value.copy_duplicate),
                creation_date: Ok(value.creation_date),
                from: Ok(value.from),
                message_definition_identifier: Ok(value.message_definition_identifier),
                possible_duplicate: Ok(value.possible_duplicate),
                related: Ok(value.related),
                signature: Ok(value.signature),
                to: Ok(value.to),
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
    pub struct FinancialInstitutionIdentification182 {
        clearing_system_member_identification: ::std::result::Result<
            ::std::option::Option<super::ClearingSystemMemberIdentification21>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FinancialInstitutionIdentification182 {
        fn default() -> Self {
            Self {
                clearing_system_member_identification: Ok(Default::default()),
            }
        }
    }
    impl FinancialInstitutionIdentification182 {
        pub fn clearing_system_member_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ClearingSystemMemberIdentification21>,
            >,
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
    impl ::std::convert::TryFrom<FinancialInstitutionIdentification182>
    for super::FinancialInstitutionIdentification182 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FinancialInstitutionIdentification182,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                clearing_system_member_identification: value
                    .clearing_system_member_identification?,
            })
        }
    }
    impl ::std::convert::From<super::FinancialInstitutionIdentification182>
    for FinancialInstitutionIdentification182 {
        fn from(value: super::FinancialInstitutionIdentification182) -> Self {
            Self {
                clearing_system_member_identification: Ok(
                    value.clearing_system_member_identification,
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GenericOrganisationIdentification11 {
        identification: ::std::result::Result<super::Max35Text, ::std::string::String>,
    }
    impl ::std::default::Default for GenericOrganisationIdentification11 {
        fn default() -> Self {
            Self {
                identification: Err("no value supplied for identification".to_string()),
            }
        }
    }
    impl GenericOrganisationIdentification11 {
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<GenericOrganisationIdentification11>
    for super::GenericOrganisationIdentification11 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GenericOrganisationIdentification11,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                identification: value.identification?,
            })
        }
    }
    impl ::std::convert::From<super::GenericOrganisationIdentification11>
    for GenericOrganisationIdentification11 {
        fn from(value: super::GenericOrganisationIdentification11) -> Self {
            Self {
                identification: Ok(value.identification),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganisationIdentification291 {
        other: ::std::result::Result<
            super::GenericOrganisationIdentification11,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganisationIdentification291 {
        fn default() -> Self {
            Self {
                other: Err("no value supplied for other".to_string()),
            }
        }
    }
    impl OrganisationIdentification291 {
        pub fn other<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GenericOrganisationIdentification11>,
            T::Error: ::std::fmt::Display,
        {
            self.other = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for other: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganisationIdentification291>
    for super::OrganisationIdentification291 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganisationIdentification291,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { other: value.other? })
        }
    }
    impl ::std::convert::From<super::OrganisationIdentification291>
    for OrganisationIdentification291 {
        fn from(value: super::OrganisationIdentification291) -> Self {
            Self { other: Ok(value.other) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Party38Choice1 {
        organisation_identification: ::std::result::Result<
            super::OrganisationIdentification291,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Party38Choice1 {
        fn default() -> Self {
            Self {
                organisation_identification: Err(
                    "no value supplied for organisation_identification".to_string(),
                ),
            }
        }
    }
    impl Party38Choice1 {
        pub fn organisation_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganisationIdentification291>,
            T::Error: ::std::fmt::Display,
        {
            self.organisation_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for organisation_identification: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Party38Choice1> for super::Party38Choice1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Party38Choice1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                organisation_identification: value.organisation_identification?,
            })
        }
    }
    impl ::std::convert::From<super::Party38Choice1> for Party38Choice1 {
        fn from(value: super::Party38Choice1) -> Self {
            Self {
                organisation_identification: Ok(value.organisation_identification),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PartyIdentification1351 {
        identification: ::std::result::Result<
            super::Party38Choice1,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PartyIdentification1351 {
        fn default() -> Self {
            Self {
                identification: Err("no value supplied for identification".to_string()),
            }
        }
    }
    impl PartyIdentification1351 {
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Party38Choice1>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PartyIdentification1351>
    for super::PartyIdentification1351 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PartyIdentification1351,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                identification: value.identification?,
            })
        }
    }
    impl ::std::convert::From<super::PartyIdentification1351>
    for PartyIdentification1351 {
        fn from(value: super::PartyIdentification1351) -> Self {
            Self {
                identification: Ok(value.identification),
            }
        }
    }
}
