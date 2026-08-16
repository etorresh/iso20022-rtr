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
///Specifies the unique identification of an account as assigned by the account servicer.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the unique identification of an account as assigned by the account servicer.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "iban"
///      ],
///      "properties": {
///        "iban": {
///          "description": "International Bank Account Number (IBAN) - identifier used internationally by financial institutions to uniquely identify the account of a customer. Further specifications of the format and content of the IBAN can be found in the standard ISO 13616 \"Banking and related financial services - International Bank Account Number (IBAN)\" version 1997-10-01, or later revisions.",
///          "$ref": "#/definitions/IBAN2007Identifier"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "other"
///      ],
///      "properties": {
///        "other": {
///          "description": "Unique identification of an account, as assigned by the account servicer, using an identification scheme.",
///          "$ref": "#/definitions/GenericAccountIdentification1"
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
pub enum AccountIdentification4Choice {
    #[serde(rename = "iban")]
    Iban(Iban2007Identifier),
    #[serde(rename = "other")]
    Other(GenericAccountIdentification1),
}
impl ::std::convert::From<Iban2007Identifier> for AccountIdentification4Choice {
    fn from(value: Iban2007Identifier) -> Self {
        Self::Iban(value)
    }
}
impl ::std::convert::From<GenericAccountIdentification1>
for AccountIdentification4Choice {
    fn from(value: GenericAccountIdentification1) -> Self {
        Self::Other(value)
    }
}
///Specifies the unique identification of an account as assigned by the account servicer.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the unique identification of an account as assigned by the account servicer.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "other"
///      ],
///      "properties": {
///        "other": {
///          "description": "Unique identification of an account, as assigned by the account servicer, using an identification scheme.",
///          "$ref": "#/definitions/GenericAccountIdentification1"
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
pub struct AccountIdentification4Choice1 {
    ///Unique identification of an account, as assigned by the account servicer, using an identification scheme.
    pub other: GenericAccountIdentification1,
}
impl AccountIdentification4Choice1 {
    pub fn builder() -> builder::AccountIdentification4Choice1 {
        Default::default()
    }
}
///Sets of elements to identify a name of the identification scheme.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Sets of elements to identify a name of the identification scheme.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Name of the identification scheme, in a coded form as published in an external list.",
///          "$ref": "#/definitions/ExternalAccountIdentification1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Name of the identification scheme, in a free text form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum AccountSchemeName1Choice {
    #[serde(rename = "code")]
    Code(ExternalAccountIdentification1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalAccountIdentification1Code>
for AccountSchemeName1Choice {
    fn from(value: ExternalAccountIdentification1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for AccountSchemeName1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
///`ActiveCurrencyAndAmount2decimalsCopy`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount",
///    "currency"
///  ],
///  "properties": {
///    "amount": {
///      "type": "string",
///      "maxLength": 15,
///      "pattern": "^0*(([0-9]{0,12}\\.[0-9]{1,2})|([0-9]{0,13}\\.[0-9]{1,1})|([0-9]{0,18}\\.)|0*|([0-9]{0,14}))$"
///    },
///    "currency": {
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyCode_fixed"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActiveCurrencyAndAmount2decimalsCopy {
    pub amount: ActiveCurrencyAndAmount2decimalsCopyAmount,
    pub currency: ActiveOrHistoricCurrencyCodeFixed,
}
impl ActiveCurrencyAndAmount2decimalsCopy {
    pub fn builder() -> builder::ActiveCurrencyAndAmount2decimalsCopy {
        Default::default()
    }
}
///`ActiveCurrencyAndAmount2decimalsCopyAmount`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 15,
///  "pattern": "^0*(([0-9]{0,12}\\.[0-9]{1,2})|([0-9]{0,13}\\.[0-9]{1,1})|([0-9]{0,18}\\.)|0*|([0-9]{0,14}))$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmount2decimalsCopyAmount(::std::string::String);
impl ::std::ops::Deref for ActiveCurrencyAndAmount2decimalsCopyAmount {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ActiveCurrencyAndAmount2decimalsCopyAmount>
for ::std::string::String {
    fn from(value: ActiveCurrencyAndAmount2decimalsCopyAmount) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ActiveCurrencyAndAmount2decimalsCopyAmount {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 15usize {
            return Err("longer than 15 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^0*(([0-9]{0,12}\\.[0-9]{1,2})|([0-9]{0,13}\\.[0-9]{1,1})|([0-9]{0,18}\\.)|0*|([0-9]{0,14}))$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^0*(([0-9]{0,12}\\.[0-9]{1,2})|([0-9]{0,13}\\.[0-9]{1,1})|([0-9]{0,18}\\.)|0*|([0-9]{0,14}))$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ActiveCurrencyAndAmount2decimalsCopyAmount {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ActiveCurrencyAndAmount2decimalsCopyAmount {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ActiveCurrencyAndAmount2decimalsCopyAmount {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ActiveCurrencyAndAmount2decimalsCopyAmount {
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
///A number of monetary units specified in an active or a historic currency where the unit of currency is explicit and compliant with ISO 4217.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A number of monetary units specified in an active or a historic currency where the unit of currency is explicit and compliant with ISO 4217.",
///  "type": "object",
///  "required": [
///    "amount",
///    "currency"
///  ],
///  "properties": {
///    "amount": {
///      "type": "string",
///      "maxLength": 19,
///      "pattern": "^0*(([0-9]{0,13}\\.[0-9]{1,5})|([0-9]{0,14}\\.[0-9]{1,4})|([0-9]{0,15}\\.[0-9]{1,3})|([0-9]{0,16}\\.[0-9]{1,2})|([0-9]{0,17}\\.[0-9]{1,1})|([0-9]{0,18}\\.)|0*|([0-9]{0,18}))$"
///    },
///    "currency": {
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyCode"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    pub amount: ActiveOrHistoricCurrencyAndAmountAmount,
    pub currency: ActiveOrHistoricCurrencyCode,
}
impl ActiveOrHistoricCurrencyAndAmount {
    pub fn builder() -> builder::ActiveOrHistoricCurrencyAndAmount {
        Default::default()
    }
}
///`ActiveOrHistoricCurrencyAndAmountAmount`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 19,
///  "pattern": "^0*(([0-9]{0,13}\\.[0-9]{1,5})|([0-9]{0,14}\\.[0-9]{1,4})|([0-9]{0,15}\\.[0-9]{1,3})|([0-9]{0,16}\\.[0-9]{1,2})|([0-9]{0,17}\\.[0-9]{1,1})|([0-9]{0,18}\\.)|0*|([0-9]{0,18}))$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountAmount(::std::string::String);
impl ::std::ops::Deref for ActiveOrHistoricCurrencyAndAmountAmount {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ActiveOrHistoricCurrencyAndAmountAmount>
for ::std::string::String {
    fn from(value: ActiveOrHistoricCurrencyAndAmountAmount) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ActiveOrHistoricCurrencyAndAmountAmount {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 19usize {
            return Err("longer than 19 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^0*(([0-9]{0,13}\\.[0-9]{1,5})|([0-9]{0,14}\\.[0-9]{1,4})|([0-9]{0,15}\\.[0-9]{1,3})|([0-9]{0,16}\\.[0-9]{1,2})|([0-9]{0,17}\\.[0-9]{1,1})|([0-9]{0,18}\\.)|0*|([0-9]{0,18}))$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^0*(([0-9]{0,13}\\.[0-9]{1,5})|([0-9]{0,14}\\.[0-9]{1,4})|([0-9]{0,15}\\.[0-9]{1,3})|([0-9]{0,16}\\.[0-9]{1,2})|([0-9]{0,17}\\.[0-9]{1,1})|([0-9]{0,18}\\.)|0*|([0-9]{0,18}))$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ActiveOrHistoricCurrencyAndAmountAmount {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ActiveOrHistoricCurrencyAndAmountAmount {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ActiveOrHistoricCurrencyAndAmountAmount {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ActiveOrHistoricCurrencyAndAmountAmount {
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
///A code allocated to a currency by a Maintenance Agency under an international identification scheme, as described in the latest edition of the international standard ISO 4217 "Codes for the representation of currencies and funds".
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A code allocated to a currency by a Maintenance Agency under an international identification scheme, as described in the latest edition of the international standard ISO 4217 \"Codes for the representation of currencies and funds\".",
///  "type": "string",
///  "pattern": "^[A-Z]{3,3}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode(::std::string::String);
impl ::std::ops::Deref for ActiveOrHistoricCurrencyCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ActiveOrHistoricCurrencyCode> for ::std::string::String {
    fn from(value: ActiveOrHistoricCurrencyCode) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ActiveOrHistoricCurrencyCode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[A-Z]{3,3}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Z]{3,3}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ActiveOrHistoricCurrencyCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ActiveOrHistoricCurrencyCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ActiveOrHistoricCurrencyCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ActiveOrHistoricCurrencyCode {
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
/**
*`CAD`-null*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "\n*`CAD`-null",
///  "type": "string",
///  "enum": [
///    "CAD"
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
pub enum ActiveOrHistoricCurrencyCodeFixed {
    #[serde(rename = "CAD")]
    Cad,
}
impl ::std::fmt::Display for ActiveOrHistoricCurrencyCodeFixed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cad => f.write_str("CAD"),
        }
    }
}
impl ::std::str::FromStr for ActiveOrHistoricCurrencyCodeFixed {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CAD" => Ok(Self::Cad),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ActiveOrHistoricCurrencyCodeFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ActiveOrHistoricCurrencyCodeFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ActiveOrHistoricCurrencyCodeFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Code allocated to a financial or non-financial institution by the ISO 9362 Registration Authority, as described in ISO 9362: 2014 - "Banking - Banking telecommunication messages - Business identifier code (BIC)".
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Code allocated to a financial or non-financial institution by the ISO 9362 Registration Authority, as described in ISO 9362: 2014 - \"Banking - Banking telecommunication messages - Business identifier code (BIC)\".",
///  "type": "string",
///  "pattern": "^[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AnyBicDec2014Identifier(::std::string::String);
impl ::std::ops::Deref for AnyBicDec2014Identifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AnyBicDec2014Identifier> for ::std::string::String {
    fn from(value: AnyBicDec2014Identifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AnyBicDec2014Identifier {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AnyBicDec2014Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AnyBicDec2014Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnyBicDec2014Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AnyBicDec2014Identifier {
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
///Rate expressed as a decimal, for example, 0.7 is 7/10 and 70%.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Rate expressed as a decimal, for example, 0.7 is 7/10 and 70%.",
///  "type": "string",
///  "maxLength": 12
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BaseOneRate(::std::string::String);
impl ::std::ops::Deref for BaseOneRate {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BaseOneRate> for ::std::string::String {
    fn from(value: BaseOneRate) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BaseOneRate {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 12usize {
            return Err("longer than 12 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BaseOneRate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BaseOneRate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BaseOneRate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BaseOneRate {
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
///Code allocated to a financial institution by the ISO 9362 Registration Authority as described in ISO 9362: 2014 - "Banking - Banking telecommunication messages - Business identifier code (BIC)".
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Code allocated to a financial institution by the ISO 9362 Registration Authority as described in ISO 9362: 2014 - \"Banking - Banking telecommunication messages - Business identifier code (BIC)\".",
///  "type": "string",
///  "pattern": "^[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BicfiDec2014Identifier(::std::string::String);
impl ::std::ops::Deref for BicfiDec2014Identifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BicfiDec2014Identifier> for ::std::string::String {
    fn from(value: BicfiDec2014Identifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BicfiDec2014Identifier {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BicfiDec2014Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BicfiDec2014Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BicfiDec2014Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BicfiDec2014Identifier {
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
///      "$ref": "#/definitions/FinancialInstitutionIdentification18__3"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct BranchAndFinancialInstitutionIdentification63 {
    ///Unique and unambiguous identification of a financial institution, as assigned under an internationally recognised or proprietary identification scheme.
    pub financial_institution_identification: FinancialInstitutionIdentification183,
}
impl BranchAndFinancialInstitutionIdentification63 {
    pub fn builder() -> builder::BranchAndFinancialInstitutionIdentification63 {
        Default::default()
    }
}
///Provides the details to identify an account.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides the details to identify an account.",
///  "type": "object",
///  "required": [
///    "identification"
///  ],
///  "properties": {
///    "currency": {
///      "description": "Identification of the currency in which the account is held.   Usage: Currency should only be used in case one and the same account number covers several currencies and the initiating party needs to identify which currency needs to be used for settlement on the account.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyCode"
///    },
///    "identification": {
///      "description": "Unique and unambiguous identification for the account between the account owner and the account servicer.",
///      "$ref": "#/definitions/AccountIdentification4Choice"
///    },
///    "name": {
///      "description": "Name of the account, as assigned by the account servicing institution, in agreement with the account owner in order to provide an additional means of identification of the account.  Usage: The account name is different from the account owner name. The account name is used in certain user communities to provide a means of identifying the account, in addition to the account owner's identity and the account number.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "proxy": {
///      "description": "Specifies an alternate assumed name for the identification of the account. ",
///      "$ref": "#/definitions/ProxyAccountIdentification1"
///    },
///    "type": {
///      "description": "Specifies the nature, or use of the account.",
///      "$ref": "#/definitions/CashAccountType2Choice"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CashAccount38 {
    ///Identification of the currency in which the account is held.   Usage: Currency should only be used in case one and the same account number covers several currencies and the initiating party needs to identify which currency needs to be used for settlement on the account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
    ///Unique and unambiguous identification for the account between the account owner and the account servicer.
    pub identification: AccountIdentification4Choice,
    ///Name of the account, as assigned by the account servicing institution, in agreement with the account owner in order to provide an additional means of identification of the account.  Usage: The account name is different from the account owner name. The account name is used in certain user communities to provide a means of identifying the account, in addition to the account owner's identity and the account number.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Max70Text>,
    ///Specifies an alternate assumed name for the identification of the account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub proxy: ::std::option::Option<ProxyAccountIdentification1>,
    ///Specifies the nature, or use of the account.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<CashAccountType2Choice>,
}
impl CashAccount38 {
    pub fn builder() -> builder::CashAccount38 {
        Default::default()
    }
}
///Provides the details to identify an account.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides the details to identify an account.",
///  "type": "object",
///  "required": [
///    "identification"
///  ],
///  "properties": {
///    "identification": {
///      "description": "Unique and unambiguous identification for the account between the account owner and the account servicer.",
///      "$ref": "#/definitions/AccountIdentification4Choice__1"
///    },
///    "proxy": {
///      "description": "Specifies an alternate assumed name for the identification of the account. ",
///      "$ref": "#/definitions/ProxyAccountIdentification1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CashAccount381 {
    ///Unique and unambiguous identification for the account between the account owner and the account servicer.
    pub identification: AccountIdentification4Choice1,
    ///Specifies an alternate assumed name for the identification of the account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub proxy: ::std::option::Option<ProxyAccountIdentification1>,
}
impl CashAccount381 {
    pub fn builder() -> builder::CashAccount381 {
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
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Account type, in a coded form.",
///          "$ref": "#/definitions/ExternalCashAccountType1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Nature or use of the account in a proprietary form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum CashAccountType2Choice {
    #[serde(rename = "code")]
    Code(ExternalCashAccountType1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalCashAccountType1Code> for CashAccountType2Choice {
    fn from(value: ExternalCashAccountType1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for CashAccountType2Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
/**Specifies the high level purpose of the instruction based on a set of pre-defined categories.
Usage: This is used by the initiating party to provide information concerning the processing of the payment. It is likely to trigger special processing by any of the agents involved in the payment chain.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the high level purpose of the instruction based on a set of pre-defined categories.\nUsage: This is used by the initiating party to provide information concerning the processing of the payment. It is likely to trigger special processing by any of the agents involved in the payment chain.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Category purpose, as published in an external category purpose code list.",
///          "$ref": "#/definitions/ExternalCategoryPurpose1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Category purpose, in a proprietary form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum CategoryPurpose1Choice {
    #[serde(rename = "code")]
    Code(ExternalCategoryPurpose1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalCategoryPurpose1Code> for CategoryPurpose1Choice {
    fn from(value: ExternalCategoryPurpose1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for CategoryPurpose1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
/**Specifies which party(ies) will pay charges due for processing of the instruction.
*`SLEV`-Charges are to be applied following the rules agreed in the service level and/or scheme.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies which party(ies) will pay charges due for processing of the instruction.\n*`SLEV`-Charges are to be applied following the rules agreed in the service level and/or scheme.",
///  "type": "string",
///  "enum": [
///    "SLEV"
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
pub enum ChargeBearerType1Code1 {
    #[serde(rename = "SLEV")]
    Slev,
}
impl ::std::fmt::Display for ChargeBearerType1Code1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Slev => f.write_str("SLEV"),
        }
    }
}
impl ::std::str::FromStr for ChargeBearerType1Code1 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SLEV" => Ok(Self::Slev),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChargeBearerType1Code1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ChargeBearerType1Code1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChargeBearerType1Code1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Provides information on the charges related to the payment transaction.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides information on the charges related to the payment transaction.",
///  "type": "object",
///  "required": [
///    "agent",
///    "amount"
///  ],
///  "properties": {
///    "agent": {
///      "description": "Agent that takes the transaction charges or to which the transaction charges are due.",
///      "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__1"
///    },
///    "amount": {
///      "description": "Transaction charges to be paid by the charge bearer.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Charges71 {
    ///Agent that takes the transaction charges or to which the transaction charges are due.
    pub agent: BranchAndFinancialInstitutionIdentification61,
    ///Transaction charges to be paid by the charge bearer.
    pub amount: ActiveOrHistoricCurrencyAndAmount,
}
impl Charges71 {
    pub fn builder() -> builder::Charges71 {
        Default::default()
    }
}
///Choice of a clearing system identifier.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Choice of a clearing system identifier.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Identification of a clearing system, in a coded form as published in an external list.",
///          "$ref": "#/definitions/ExternalClearingSystemIdentification1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Identification code for a clearing system, that has not yet been identified in the list of clearing systems.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum ClearingSystemIdentification2Choice {
    #[serde(rename = "code")]
    Code(ExternalClearingSystemIdentification1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalClearingSystemIdentification1Code>
for ClearingSystemIdentification2Choice {
    fn from(value: ExternalClearingSystemIdentification1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for ClearingSystemIdentification2Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
///Choice of a clearing system identifier.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Choice of a clearing system identifier.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Identification of a clearing system, in a coded form as published in an external list.",
///          "$ref": "#/definitions/ExternalClearingSystemIdentification1Code"
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
pub struct ClearingSystemIdentification2Choice1 {
    ///Identification of a clearing system, in a coded form as published in an external list.
    pub code: ExternalClearingSystemIdentification1Code,
}
impl ClearingSystemIdentification2Choice1 {
    pub fn builder() -> builder::ClearingSystemIdentification2Choice1 {
        Default::default()
    }
}
///Specifies the clearing system identification.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the clearing system identification.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Infrastructure through which the payment instruction is processed, as published in an external clearing system identification code list.",
///          "$ref": "#/definitions/ExternalCashClearingSystem1Code_fixed"
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
pub struct ClearingSystemIdentification3Choice1 {
    ///Infrastructure through which the payment instruction is processed, as published in an external clearing system identification code list.
    pub code: ExternalCashClearingSystem1CodeFixed,
}
impl ClearingSystemIdentification3Choice1 {
    pub fn builder() -> builder::ClearingSystemIdentification3Choice1 {
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
///    "clearing_system_identification": {
///      "description": "Specification of a pre-agreed offering between clearing agents or the channel through which the payment instruction is processed.",
///      "$ref": "#/definitions/ClearingSystemIdentification2Choice"
///    },
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
pub struct ClearingSystemMemberIdentification2 {
    ///Specification of a pre-agreed offering between clearing agents or the channel through which the payment instruction is processed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub clearing_system_identification: ::std::option::Option<
        ClearingSystemIdentification2Choice,
    >,
    ///Identification of a member of a clearing system.
    pub member_identification: Max35Text,
}
impl ClearingSystemMemberIdentification2 {
    pub fn builder() -> builder::ClearingSystemMemberIdentification2 {
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
///    "clearing_system_identification": {
///      "description": "Specification of a pre-agreed offering between clearing agents or the channel through which the payment instruction is processed.",
///      "$ref": "#/definitions/ClearingSystemIdentification2Choice__1"
///    },
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
    ///Specification of a pre-agreed offering between clearing agents or the channel through which the payment instruction is processed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub clearing_system_identification: ::std::option::Option<
        ClearingSystemIdentification2Choice1,
    >,
    ///Identification of a member of a clearing system.
    pub member_identification: Max35Text,
}
impl ClearingSystemMemberIdentification21 {
    pub fn builder() -> builder::ClearingSystemMemberIdentification21 {
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
pub struct ClearingSystemMemberIdentification22 {
    ///Identification of a member of a clearing system.
    pub member_identification: Max35Text,
}
impl ClearingSystemMemberIdentification22 {
    pub fn builder() -> builder::ClearingSystemMemberIdentification22 {
        Default::default()
    }
}
///Specifies the details of the contact person.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the details of the contact person.",
///  "type": "object",
///  "properties": {
///    "email_address": {
///      "description": "Address for electronic mail (e-mail).",
///      "$ref": "#/definitions/Max2048Text"
///    },
///    "fax_number": {
///      "description": "Collection of information that identifies a FAX number, as defined by telecom services.",
///      "$ref": "#/definitions/PhoneNumber"
///    },
///    "mobile_number": {
///      "description": "Collection of information that identifies a mobile phone number, as defined by telecom services.",
///      "$ref": "#/definitions/PhoneNumber"
///    },
///    "name": {
///      "description": "Name by which a party is known and which is usually used to identify that party.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "phone_number": {
///      "description": "Collection of information that identifies a phone number, as defined by telecom services.",
///      "$ref": "#/definitions/PhoneNumber"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Contact41 {
    ///Address for electronic mail (e-mail).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email_address: ::std::option::Option<Max2048Text>,
    ///Collection of information that identifies a FAX number, as defined by telecom services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fax_number: ::std::option::Option<PhoneNumber>,
    ///Collection of information that identifies a mobile phone number, as defined by telecom services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mobile_number: ::std::option::Option<PhoneNumber>,
    ///Name by which a party is known and which is usually used to identify that party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Max140Text>,
    ///Collection of information that identifies a phone number, as defined by telecom services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phone_number: ::std::option::Option<PhoneNumber>,
}
impl ::std::default::Default for Contact41 {
    fn default() -> Self {
        Self {
            email_address: Default::default(),
            fax_number: Default::default(),
            mobile_number: Default::default(),
            name: Default::default(),
            phone_number: Default::default(),
        }
    }
}
impl Contact41 {
    pub fn builder() -> builder::Contact41 {
        Default::default()
    }
}
///Specifies the details of the contact person.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the details of the contact person.",
///  "type": "object",
///  "properties": {
///    "email_address": {
///      "description": "Address for electronic mail (e-mail).",
///      "$ref": "#/definitions/Max2048Text"
///    },
///    "mobile_number": {
///      "description": "Collection of information that identifies a mobile phone number, as defined by telecom services.",
///      "$ref": "#/definitions/PhoneNumber"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Contact42 {
    ///Address for electronic mail (e-mail).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email_address: ::std::option::Option<Max2048Text>,
    ///Collection of information that identifies a mobile phone number, as defined by telecom services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mobile_number: ::std::option::Option<PhoneNumber>,
}
impl ::std::default::Default for Contact42 {
    fn default() -> Self {
        Self {
            email_address: Default::default(),
            mobile_number: Default::default(),
        }
    }
}
impl Contact42 {
    pub fn builder() -> builder::Contact42 {
        Default::default()
    }
}
///Specifies the details of the contact person.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the details of the contact person.",
///  "type": "object",
///  "properties": {
///    "department": {
///      "description": "Identification of a division of a large organisation or building.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "email_address": {
///      "description": "Address for electronic mail (e-mail).",
///      "$ref": "#/definitions/Max2048Text"
///    },
///    "fax_number": {
///      "description": "Collection of information that identifies a FAX number, as defined by telecom services.",
///      "$ref": "#/definitions/PhoneNumber"
///    },
///    "mobile_number": {
///      "description": "Collection of information that identifies a mobile phone number, as defined by telecom services.",
///      "$ref": "#/definitions/PhoneNumber"
///    },
///    "name": {
///      "description": "Name by which a party is known and which is usually used to identify that party.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "phone_number": {
///      "description": "Collection of information that identifies a phone number, as defined by telecom services.",
///      "$ref": "#/definitions/PhoneNumber"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Contact43 {
    ///Identification of a division of a large organisation or building.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub department: ::std::option::Option<Max70Text>,
    ///Address for electronic mail (e-mail).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email_address: ::std::option::Option<Max2048Text>,
    ///Collection of information that identifies a FAX number, as defined by telecom services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fax_number: ::std::option::Option<PhoneNumber>,
    ///Collection of information that identifies a mobile phone number, as defined by telecom services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mobile_number: ::std::option::Option<PhoneNumber>,
    ///Name by which a party is known and which is usually used to identify that party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Max140Text>,
    ///Collection of information that identifies a phone number, as defined by telecom services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phone_number: ::std::option::Option<PhoneNumber>,
}
impl ::std::default::Default for Contact43 {
    fn default() -> Self {
        Self {
            department: Default::default(),
            email_address: Default::default(),
            fax_number: Default::default(),
            mobile_number: Default::default(),
            name: Default::default(),
            phone_number: Default::default(),
        }
    }
}
impl Contact43 {
    pub fn builder() -> builder::Contact43 {
        Default::default()
    }
}
///Code to identify a country, a dependency, or another area of particular geopolitical interest, on the basis of country names obtained from the United Nations (ISO 3166, Alpha-2 code).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Code to identify a country, a dependency, or another area of particular geopolitical interest, on the basis of country names obtained from the United Nations (ISO 3166, Alpha-2 code).",
///  "type": "string",
///  "pattern": "^[A-Z]{2,2}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CountryCode(::std::string::String);
impl ::std::ops::Deref for CountryCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CountryCode> for ::std::string::String {
    fn from(value: CountryCode) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CountryCode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[A-Z]{2,2}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Z]{2,2}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CountryCode {
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
/**Specifies if an operation is an increase or a decrease.
*`CRDT`-Operation is an increase.
*`DBIT`-Operation is a decrease.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies if an operation is an increase or a decrease.\n*`CRDT`-Operation is an increase.\n*`DBIT`-Operation is a decrease.",
///  "type": "string",
///  "enum": [
///    "CRDT",
///    "DBIT"
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
pub enum CreditDebitCode {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
}
impl ::std::fmt::Display for CreditDebitCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Crdt => f.write_str("CRDT"),
            Self::Dbit => f.write_str("DBIT"),
        }
    }
}
impl ::std::str::FromStr for CreditDebitCode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CRDT" => Ok(Self::Crdt),
            "DBIT" => Ok(Self::Dbit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreditDebitCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CreditDebitCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreditDebitCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Provides further details specific to the individual transaction(s) included in the message.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides further details specific to the individual transaction(s) included in the message.",
///  "type": "object",
///  "required": [
///    "charge_bearer",
///    "creditor",
///    "creditor_account",
///    "creditor_agent",
///    "debtor",
///    "debtor_account",
///    "debtor_agent",
///    "instructed_agent",
///    "instructing_agent",
///    "interbank_settlement_amount",
///    "interbank_settlement_date",
///    "payment_identification",
///    "payment_type_information"
///  ],
///  "properties": {
///    "acceptance_date_time": {
///      "description": "Point in time when the payment order from the initiating party meets the processing conditions of the account servicing agent. This means that the account servicing agent has received the payment order and has applied checks such as authorisation, availability of funds.",
///      "$ref": "#/definitions/ISONormalisedDateTime"
///    },
///    "charge_bearer": {
///      "description": "Specifies which party/parties will bear the charges associated with the processing of the payment transaction.",
///      "$ref": "#/definitions/ChargeBearerType1Code__1"
///    },
///    "charges_information": {
///      "description": "Provides information on the charges to be paid by the charge bearer(s) related to the payment transaction.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Charges7__1"
///      },
///      "maxItems": 10
///    },
///    "creditor": {
///      "description": "Party to which an amount of money is due.",
///      "$ref": "#/definitions/PartyIdentification135__3"
///    },
///    "creditor_account": {
///      "description": "Unambiguous identification of the account of the creditor to which a credit entry will be posted as a result of the payment transaction.",
///      "$ref": "#/definitions/CashAccount38__1"
///    },
///    "creditor_agent": {
///      "description": "Financial institution servicing an account for the creditor.",
///      "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__1"
///    },
///    "creditor_agent_account": {
///      "description": "Unambiguous identification of the account of the creditor agent at its servicing agent to which a credit entry will be made as a result of the payment transaction.",
///      "$ref": "#/definitions/CashAccount38"
///    },
///    "debtor": {
///      "description": "Party that owes an amount of money to the (ultimate) creditor.",
///      "$ref": "#/definitions/PartyIdentification135__3"
///    },
///    "debtor_account": {
///      "description": "Unambiguous identification of the account of the debtor to which a debit entry will be made as a result of the transaction.",
///      "$ref": "#/definitions/CashAccount38"
///    },
///    "debtor_agent": {
///      "description": "Financial institution servicing an account for the debtor.",
///      "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__1"
///    },
///    "debtor_agent_account": {
///      "description": "Unambiguous identification of the account of the debtor agent at its servicing agent in the payment chain.",
///      "$ref": "#/definitions/CashAccount38"
///    },
///    "exchange_rate": {
///      "description": "Factor used to convert an amount from one currency into another. This reflects the price at which one currency was bought with another currency.",
///      "$ref": "#/definitions/BaseOneRate"
///    },
///    "initiating_party": {
///      "description": "Party that initiates the payment. Usage: This can be either the debtor or a party that initiates the credit transfer on behalf of the debtor.",
///      "$ref": "#/definitions/PartyIdentification135__2"
///    },
///    "instructed_agent": {
///      "description": "Agent that is instructed by the previous party in the chain to carry out the (set of) instruction(s).",
///      "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__3"
///    },
///    "instructed_amount": {
///      "description": "Amount of money to be moved between the debtor and creditor, before deduction of charges, expressed in the currency as ordered by the initiating party. Usage: This amount has to be transported unchanged through the transaction chain.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "instructing_agent": {
///      "description": "Agent that instructs the next party in the chain to carry out the (set of) instruction(s).",
///      "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__3"
///    },
///    "instruction_for_creditor_agent": {
///      "description": "Further information related to the processing of the payment instruction, provided by the initiating party, and intended for the creditor agent.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/InstructionForCreditorAgent1"
///      }
///    },
///    "interbank_settlement_amount": {
///      "description": "Amount of money moved between the instructing agent and the instructed agent.",
///      "$ref": "#/definitions/ActiveCurrencyAndAmount_2decimals---copy"
///    },
///    "interbank_settlement_date": {
///      "description": "Date on which the amount of money ceases to be available to the agent that owes it and when the amount of money becomes available to the agent to which it is due.",
///      "$ref": "#/definitions/ISODate"
///    },
///    "payment_identification": {
///      "description": "Set of elements used to reference a payment instruction.",
///      "$ref": "#/definitions/PaymentIdentification7__1"
///    },
///    "payment_type_information": {
///      "description": "Set of elements used to further specify the type of transaction.",
///      "$ref": "#/definitions/PaymentTypeInformation28__1"
///    },
///    "previous_instructing_agent1": {
///      "description": "Agent immediately prior to the instructing agent.",
///      "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__2"
///    },
///    "previous_instructing_agent1_account": {
///      "description": "Unambiguous identification of the account of the previous instructing agent at its servicing agent in the payment chain.",
///      "$ref": "#/definitions/CashAccount38"
///    },
///    "previous_instructing_agent2": {
///      "description": "Agent immediately prior to the instructing agent.",
///      "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__2"
///    },
///    "previous_instructing_agent2_account": {
///      "description": "Unambiguous identification of the account of the previous instructing agent at its servicing agent in the payment chain.",
///      "$ref": "#/definitions/CashAccount38"
///    },
///    "previous_instructing_agent3": {
///      "description": "Agent immediately prior to the instructing agent.",
///      "$ref": "#/definitions/BranchAndFinancialInstitutionIdentification6__2"
///    },
///    "previous_instructing_agent3_account": {
///      "description": "Unambiguous identification of the account of the previous instructing agent at its servicing agent in the payment chain.",
///      "$ref": "#/definitions/CashAccount38"
///    },
///    "purpose": {
///      "description": "Underlying reason for the payment transaction. Usage: Purpose is used by the end-customers, that is initiating party, (ultimate) debtor, (ultimate) creditor to provide information concerning the nature of the payment. Purpose is a content element, which is not used for processing by any of the agents involved in the payment chain.",
///      "$ref": "#/definitions/Purpose2Choice"
///    },
///    "regulatory_reporting": {
///      "description": "Information needed due to regulatory and statutory requirements.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/RegulatoryReporting3"
///      },
///      "maxItems": 10
///    },
///    "related_remittance_information": {
///      "description": "Provides information related to the handling of the remittance information by any of the agents in the transaction processing chain.",
///      "$ref": "#/definitions/RemittanceLocation7__1"
///    },
///    "remittance_information": {
///      "description": "Information supplied to enable the matching of an entry with the items that the transfer is intended to settle, such as commercial invoices in an accounts' receivable system.",
///      "$ref": "#/definitions/RemittanceInformation16__1"
///    },
///    "ultimate_creditor": {
///      "description": "Ultimate party to which an amount of money is due.",
///      "$ref": "#/definitions/PartyIdentification135__1"
///    },
///    "ultimate_debtor": {
///      "description": "Ultimate party that owes an amount of money to the (ultimate) creditor.",
///      "$ref": "#/definitions/PartyIdentification135__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreditTransferTransaction391 {
    ///Point in time when the payment order from the initiating party meets the processing conditions of the account servicing agent. This means that the account servicing agent has received the payment order and has applied checks such as authorisation, availability of funds.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub acceptance_date_time: ::std::option::Option<IsoNormalisedDateTime>,
    ///Specifies which party/parties will bear the charges associated with the processing of the payment transaction.
    pub charge_bearer: ChargeBearerType1Code1,
    ///Provides information on the charges to be paid by the charge bearer(s) related to the payment transaction.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub charges_information: ::std::vec::Vec<Charges71>,
    ///Party to which an amount of money is due.
    pub creditor: PartyIdentification1353,
    ///Unambiguous identification of the account of the creditor to which a credit entry will be posted as a result of the payment transaction.
    pub creditor_account: CashAccount381,
    ///Financial institution servicing an account for the creditor.
    pub creditor_agent: BranchAndFinancialInstitutionIdentification61,
    ///Unambiguous identification of the account of the creditor agent at its servicing agent to which a credit entry will be made as a result of the payment transaction.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub creditor_agent_account: ::std::option::Option<CashAccount38>,
    ///Party that owes an amount of money to the (ultimate) creditor.
    pub debtor: PartyIdentification1353,
    ///Unambiguous identification of the account of the debtor to which a debit entry will be made as a result of the transaction.
    pub debtor_account: CashAccount38,
    ///Financial institution servicing an account for the debtor.
    pub debtor_agent: BranchAndFinancialInstitutionIdentification61,
    ///Unambiguous identification of the account of the debtor agent at its servicing agent in the payment chain.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub debtor_agent_account: ::std::option::Option<CashAccount38>,
    ///Factor used to convert an amount from one currency into another. This reflects the price at which one currency was bought with another currency.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exchange_rate: ::std::option::Option<BaseOneRate>,
    ///Party that initiates the payment. Usage: This can be either the debtor or a party that initiates the credit transfer on behalf of the debtor.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub initiating_party: ::std::option::Option<PartyIdentification1352>,
    ///Agent that is instructed by the previous party in the chain to carry out the (set of) instruction(s).
    pub instructed_agent: BranchAndFinancialInstitutionIdentification63,
    ///Amount of money to be moved between the debtor and creditor, before deduction of charges, expressed in the currency as ordered by the initiating party. Usage: This amount has to be transported unchanged through the transaction chain.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructed_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Agent that instructs the next party in the chain to carry out the (set of) instruction(s).
    pub instructing_agent: BranchAndFinancialInstitutionIdentification63,
    ///Further information related to the processing of the payment instruction, provided by the initiating party, and intended for the creditor agent.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub instruction_for_creditor_agent: ::std::vec::Vec<InstructionForCreditorAgent1>,
    ///Amount of money moved between the instructing agent and the instructed agent.
    pub interbank_settlement_amount: ActiveCurrencyAndAmount2decimalsCopy,
    ///Date on which the amount of money ceases to be available to the agent that owes it and when the amount of money becomes available to the agent to which it is due.
    pub interbank_settlement_date: IsoDate,
    ///Set of elements used to reference a payment instruction.
    pub payment_identification: PaymentIdentification71,
    ///Set of elements used to further specify the type of transaction.
    pub payment_type_information: PaymentTypeInformation281,
    ///Agent immediately prior to the instructing agent.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_instructing_agent1: ::std::option::Option<
        BranchAndFinancialInstitutionIdentification62,
    >,
    ///Unambiguous identification of the account of the previous instructing agent at its servicing agent in the payment chain.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_instructing_agent1_account: ::std::option::Option<CashAccount38>,
    ///Agent immediately prior to the instructing agent.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_instructing_agent2: ::std::option::Option<
        BranchAndFinancialInstitutionIdentification62,
    >,
    ///Unambiguous identification of the account of the previous instructing agent at its servicing agent in the payment chain.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_instructing_agent2_account: ::std::option::Option<CashAccount38>,
    ///Agent immediately prior to the instructing agent.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_instructing_agent3: ::std::option::Option<
        BranchAndFinancialInstitutionIdentification62,
    >,
    ///Unambiguous identification of the account of the previous instructing agent at its servicing agent in the payment chain.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_instructing_agent3_account: ::std::option::Option<CashAccount38>,
    ///Underlying reason for the payment transaction. Usage: Purpose is used by the end-customers, that is initiating party, (ultimate) debtor, (ultimate) creditor to provide information concerning the nature of the payment. Purpose is a content element, which is not used for processing by any of the agents involved in the payment chain.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub purpose: ::std::option::Option<Purpose2Choice>,
    ///Information needed due to regulatory and statutory requirements.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub regulatory_reporting: ::std::vec::Vec<RegulatoryReporting3>,
    ///Provides information related to the handling of the remittance information by any of the agents in the transaction processing chain.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub related_remittance_information: ::std::option::Option<RemittanceLocation71>,
    ///Information supplied to enable the matching of an entry with the items that the transfer is intended to settle, such as commercial invoices in an accounts' receivable system.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remittance_information: ::std::option::Option<RemittanceInformation161>,
    ///Ultimate party to which an amount of money is due.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ultimate_creditor: ::std::option::Option<PartyIdentification1351>,
    ///Ultimate party that owes an amount of money to the (ultimate) creditor.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ultimate_debtor: ::std::option::Option<PartyIdentification1351>,
}
impl CreditTransferTransaction391 {
    pub fn builder() -> builder::CreditTransferTransaction391 {
        Default::default()
    }
}
///Reference information provided by the creditor to allow the identification of the underlying documents.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Reference information provided by the creditor to allow the identification of the underlying documents.",
///  "type": "object",
///  "properties": {
///    "reference": {
///      "description": "Unique reference, as assigned by the creditor, to unambiguously refer to the payment transaction.  Usage: If available, the initiating party should provide this reference in the structured remittance information, to enable reconciliation by the creditor upon receipt of the amount of money.  If the business context requires the use of a creditor reference or a payment remit identification, and only one identifier can be passed through the end-to-end chain, the creditor's reference or payment remittance identification should be quoted in the end-to-end transaction identification.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "type": {
///      "description": "Specifies the type of creditor reference.",
///      "$ref": "#/definitions/CreditorReferenceType2"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreditorReferenceInformation2 {
    ///Unique reference, as assigned by the creditor, to unambiguously refer to the payment transaction.  Usage: If available, the initiating party should provide this reference in the structured remittance information, to enable reconciliation by the creditor upon receipt of the amount of money.  If the business context requires the use of a creditor reference or a payment remit identification, and only one identifier can be passed through the end-to-end chain, the creditor's reference or payment remittance identification should be quoted in the end-to-end transaction identification.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reference: ::std::option::Option<Max35Text>,
    ///Specifies the type of creditor reference.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<CreditorReferenceType2>,
}
impl ::std::default::Default for CreditorReferenceInformation2 {
    fn default() -> Self {
        Self {
            reference: Default::default(),
            type_: Default::default(),
        }
    }
}
impl CreditorReferenceInformation2 {
    pub fn builder() -> builder::CreditorReferenceInformation2 {
        Default::default()
    }
}
///Specifies the type of document referred by the creditor.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the type of document referred by the creditor.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Type of creditor reference, in a coded form.",
///          "$ref": "#/definitions/DocumentType3Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Creditor reference type, in a proprietary form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum CreditorReferenceType1Choice {
    #[serde(rename = "code")]
    Code(DocumentType3Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<DocumentType3Code> for CreditorReferenceType1Choice {
    fn from(value: DocumentType3Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for CreditorReferenceType1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
///Specifies the type of creditor reference.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the type of creditor reference.",
///  "type": "object",
///  "required": [
///    "code_or_proprietary"
///  ],
///  "properties": {
///    "code_or_proprietary": {
///      "description": "Coded or proprietary format creditor reference type.",
///      "$ref": "#/definitions/CreditorReferenceType1Choice"
///    },
///    "issuer": {
///      "description": "Entity that assigns the credit reference type.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreditorReferenceType2 {
    ///Coded or proprietary format creditor reference type.
    pub code_or_proprietary: CreditorReferenceType1Choice,
    ///Entity that assigns the credit reference type.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer: ::std::option::Option<Max35Text>,
}
impl CreditorReferenceType2 {
    pub fn builder() -> builder::CreditorReferenceType2 {
        Default::default()
    }
}
///Date and place of birth of a person.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Date and place of birth of a person.",
///  "type": "object",
///  "required": [
///    "birth_date",
///    "city_of_birth",
///    "country_of_birth"
///  ],
///  "properties": {
///    "birth_date": {
///      "description": "Date on which a person is born.",
///      "$ref": "#/definitions/ISODate"
///    },
///    "city_of_birth": {
///      "description": "City where a person was born.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "country_of_birth": {
///      "description": "Country where a person was born.",
///      "$ref": "#/definitions/CountryCode"
///    },
///    "province_of_birth": {
///      "description": "Province where a person was born.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DateAndPlaceOfBirth1 {
    ///Date on which a person is born.
    pub birth_date: IsoDate,
    ///City where a person was born.
    pub city_of_birth: Max35Text,
    ///Country where a person was born.
    pub country_of_birth: CountryCode,
    ///Province where a person was born.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub province_of_birth: ::std::option::Option<Max35Text>,
}
impl DateAndPlaceOfBirth1 {
    pub fn builder() -> builder::DateAndPlaceOfBirth1 {
        Default::default()
    }
}
///Range of time defined by a start date and an end date.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Range of time defined by a start date and an end date.",
///  "type": "object",
///  "required": [
///    "from_date",
///    "to_date"
///  ],
///  "properties": {
///    "from_date": {
///      "description": "Start date of the range.",
///      "$ref": "#/definitions/ISODate"
///    },
///    "to_date": {
///      "description": "End date of the range.",
///      "$ref": "#/definitions/ISODate"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DatePeriod2 {
    ///Start date of the range.
    pub from_date: IsoDate,
    ///End date of the range.
    pub to_date: IsoDate,
}
impl DatePeriod2 {
    pub fn builder() -> builder::DatePeriod2 {
        Default::default()
    }
}
///Specifies the amount with a specific type.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the amount with a specific type.",
///  "type": "object",
///  "required": [
///    "amount"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Amount of money, which has been typed.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "type": {
///      "description": "Specifies the type of the amount.",
///      "$ref": "#/definitions/DiscountAmountType1Choice"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DiscountAmountAndType1 {
    ///Amount of money, which has been typed.
    pub amount: ActiveOrHistoricCurrencyAndAmount,
    ///Specifies the type of the amount.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<DiscountAmountType1Choice>,
}
impl DiscountAmountAndType1 {
    pub fn builder() -> builder::DiscountAmountAndType1 {
        Default::default()
    }
}
///Specifies the amount type.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the amount type.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Specifies the amount type, in a coded form.",
///          "$ref": "#/definitions/ExternalDiscountAmountType1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Specifies the amount type, in a free-text form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum DiscountAmountType1Choice {
    #[serde(rename = "code")]
    Code(ExternalDiscountAmountType1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalDiscountAmountType1Code>
for DiscountAmountType1Choice {
    fn from(value: ExternalDiscountAmountType1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for DiscountAmountType1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
///Set of elements used to provide information on the amount and reason of the document adjustment.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Set of elements used to provide information on the amount and reason of the document adjustment.",
///  "type": "object",
///  "required": [
///    "amount"
///  ],
///  "properties": {
///    "additional_information": {
///      "description": "Provides further details on the document adjustment.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "amount": {
///      "description": "Amount of money of the document adjustment.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "credit_debit_indicator": {
///      "description": "Specifies whether the adjustment must be subtracted or added to the total amount.",
///      "$ref": "#/definitions/CreditDebitCode"
///    },
///    "reason": {
///      "description": "Specifies the reason for the adjustment.",
///      "$ref": "#/definitions/Max4Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DocumentAdjustment1 {
    ///Provides further details on the document adjustment.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub additional_information: ::std::option::Option<Max140Text>,
    ///Amount of money of the document adjustment.
    pub amount: ActiveOrHistoricCurrencyAndAmount,
    ///Specifies whether the adjustment must be subtracted or added to the total amount.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credit_debit_indicator: ::std::option::Option<CreditDebitCode>,
    ///Specifies the reason for the adjustment.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<Max4Text>,
}
impl DocumentAdjustment1 {
    pub fn builder() -> builder::DocumentAdjustment1 {
        Default::default()
    }
}
///Identifies the documents referred to in the remittance information.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Identifies the documents referred to in the remittance information.",
///  "type": "object",
///  "properties": {
///    "number": {
///      "description": "Identification of the type specified for the referred document line.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "related_date": {
///      "description": "Date associated with the referred document line.",
///      "$ref": "#/definitions/ISODate"
///    },
///    "type": {
///      "description": "Specifies the type of referred document line identification.",
///      "$ref": "#/definitions/DocumentLineType1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DocumentLineIdentification1 {
    ///Identification of the type specified for the referred document line.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub number: ::std::option::Option<Max35Text>,
    ///Date associated with the referred document line.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub related_date: ::std::option::Option<IsoDate>,
    ///Specifies the type of referred document line identification.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<DocumentLineType1>,
}
impl ::std::default::Default for DocumentLineIdentification1 {
    fn default() -> Self {
        Self {
            number: Default::default(),
            related_date: Default::default(),
            type_: Default::default(),
        }
    }
}
impl DocumentLineIdentification1 {
    pub fn builder() -> builder::DocumentLineIdentification1 {
        Default::default()
    }
}
/**Provides document line information.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides document line information.\r\n",
///  "type": "object",
///  "required": [
///    "identification"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Provides details on the amounts of the document line.",
///      "$ref": "#/definitions/RemittanceAmount3"
///    },
///    "description": {
///      "description": "Description associated with the document line.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "identification": {
///      "description": "Provides identification of the document line.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/DocumentLineIdentification1"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DocumentLineInformation11 {
    ///Provides details on the amounts of the document line.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<RemittanceAmount3>,
    ///Description associated with the document line.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<Max35Text>,
    ///Provides identification of the document line.
    pub identification: ::std::vec::Vec<DocumentLineIdentification1>,
}
impl DocumentLineInformation11 {
    pub fn builder() -> builder::DocumentLineInformation11 {
        Default::default()
    }
}
///Specifies the type of the document line identification.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the type of the document line identification.",
///  "type": "object",
///  "required": [
///    "code_or_proprietary"
///  ],
///  "properties": {
///    "code_or_proprietary": {
///      "description": "Provides the type details of the referred document line identification.",
///      "$ref": "#/definitions/DocumentLineType1Choice"
///    },
///    "issuer": {
///      "description": "Identification of the issuer of the reference document line identificationtype.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DocumentLineType1 {
    ///Provides the type details of the referred document line identification.
    pub code_or_proprietary: DocumentLineType1Choice,
    ///Identification of the issuer of the reference document line identificationtype.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer: ::std::option::Option<Max35Text>,
}
impl DocumentLineType1 {
    pub fn builder() -> builder::DocumentLineType1 {
        Default::default()
    }
}
///Specifies the type of the document line identification.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the type of the document line identification.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Line identification type in a coded form.",
///          "$ref": "#/definitions/ExternalDocumentLineType1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Proprietary identification of the type of the remittance document.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum DocumentLineType1Choice {
    #[serde(rename = "code")]
    Code(ExternalDocumentLineType1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalDocumentLineType1Code> for DocumentLineType1Choice {
    fn from(value: ExternalDocumentLineType1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for DocumentLineType1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
/**Specifies a type of financial or commercial document.
*`RADM`-Document is a remittance advice sent separately from the current transaction.
*`RPIN`-Document is a linked payment instruction to which the current payment instruction is related, for example, in a cover scenario.
*`FXDR`-Document is a pre-agreed or pre-arranged foreign exchange transaction to which the payment transaction refers.
*`DISP`-Document is a dispatch advice.
*`PUOR`-Document is a purchase order.
*`SCOR`-Document is a structured communication reference provided by the creditor to identify the referred transaction.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a type of financial or commercial document.\n*`RADM`-Document is a remittance advice sent separately from the current transaction.\n*`RPIN`-Document is a linked payment instruction to which the current payment instruction is related, for example, in a cover scenario.\n*`FXDR`-Document is a pre-agreed or pre-arranged foreign exchange transaction to which the payment transaction refers.\n*`DISP`-Document is a dispatch advice.\n*`PUOR`-Document is a purchase order.\n*`SCOR`-Document is a structured communication reference provided by the creditor to identify the referred transaction.",
///  "type": "string",
///  "enum": [
///    "RADM",
///    "RPIN",
///    "FXDR",
///    "DISP",
///    "PUOR",
///    "SCOR"
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
pub enum DocumentType3Code {
    #[serde(rename = "RADM")]
    Radm,
    #[serde(rename = "RPIN")]
    Rpin,
    #[serde(rename = "FXDR")]
    Fxdr,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "PUOR")]
    Puor,
    #[serde(rename = "SCOR")]
    Scor,
}
impl ::std::fmt::Display for DocumentType3Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Radm => f.write_str("RADM"),
            Self::Rpin => f.write_str("RPIN"),
            Self::Fxdr => f.write_str("FXDR"),
            Self::Disp => f.write_str("DISP"),
            Self::Puor => f.write_str("PUOR"),
            Self::Scor => f.write_str("SCOR"),
        }
    }
}
impl ::std::str::FromStr for DocumentType3Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "RADM" => Ok(Self::Radm),
            "RPIN" => Ok(Self::Rpin),
            "FXDR" => Ok(Self::Fxdr),
            "DISP" => Ok(Self::Disp),
            "PUOR" => Ok(Self::Puor),
            "SCOR" => Ok(Self::Scor),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DocumentType3Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DocumentType3Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DocumentType3Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**Specifies a type of financial or commercial document.
*`MSIN`-Document is an invoice claiming payment for the supply of metered services, for example gas or electricity supplied to a fixed meter.
*`CNFA`-Document is a credit note for the final amount settled for a commercial transaction.
*`DNFA`-Document is a debit note for the final amount settled for a commercial transaction.
*`CINV`-Document is an invoice.
*`CREN`-Document is a credit note.
*`DEBN`-Document is a debit note.
*`HIRI`-Document is an invoice for the hiring of human resources or renting goods or equipment.
*`SBIN`-Document is an invoice issued by the debtor.
*`CMCN`-Document is an agreement between the parties, stipulating the terms and conditions of the delivery of goods or services.
*`SOAC`-Document is a statement of the transactions posted to the debtor's account at the supplier.
*`DISP`-Document is a dispatch advice.
*`BOLD`-Document is a shipping notice.
*`VCHR`-Document is an electronic payment document.
*`AROI`-Document is a payment that applies to a specific source document.
*`TSUT`-Document is a transaction identifier as assigned by the Trade Services Utility.
*`PUOR`-Document is a purchase order.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a type of financial or commercial document.\n*`MSIN`-Document is an invoice claiming payment for the supply of metered services, for example gas or electricity supplied to a fixed meter.\n*`CNFA`-Document is a credit note for the final amount settled for a commercial transaction.\n*`DNFA`-Document is a debit note for the final amount settled for a commercial transaction.\n*`CINV`-Document is an invoice.\n*`CREN`-Document is a credit note.\n*`DEBN`-Document is a debit note.\n*`HIRI`-Document is an invoice for the hiring of human resources or renting goods or equipment.\n*`SBIN`-Document is an invoice issued by the debtor.\n*`CMCN`-Document is an agreement between the parties, stipulating the terms and conditions of the delivery of goods or services.\n*`SOAC`-Document is a statement of the transactions posted to the debtor's account at the supplier.\n*`DISP`-Document is a dispatch advice.\n*`BOLD`-Document is a shipping notice.\n*`VCHR`-Document is an electronic payment document.\n*`AROI`-Document is a payment that applies to a specific source document.\n*`TSUT`-Document is a transaction identifier as assigned by the Trade Services Utility.\n*`PUOR`-Document is a purchase order.",
///  "type": "string",
///  "enum": [
///    "MSIN",
///    "CNFA",
///    "DNFA",
///    "CINV",
///    "CREN",
///    "DEBN",
///    "HIRI",
///    "SBIN",
///    "CMCN",
///    "SOAC",
///    "DISP",
///    "BOLD",
///    "VCHR",
///    "AROI",
///    "TSUT",
///    "PUOR"
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
pub enum DocumentType6Code {
    #[serde(rename = "MSIN")]
    Msin,
    #[serde(rename = "CNFA")]
    Cnfa,
    #[serde(rename = "DNFA")]
    Dnfa,
    #[serde(rename = "CINV")]
    Cinv,
    #[serde(rename = "CREN")]
    Cren,
    #[serde(rename = "DEBN")]
    Debn,
    #[serde(rename = "HIRI")]
    Hiri,
    #[serde(rename = "SBIN")]
    Sbin,
    #[serde(rename = "CMCN")]
    Cmcn,
    #[serde(rename = "SOAC")]
    Soac,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "BOLD")]
    Bold,
    #[serde(rename = "VCHR")]
    Vchr,
    #[serde(rename = "AROI")]
    Aroi,
    #[serde(rename = "TSUT")]
    Tsut,
    #[serde(rename = "PUOR")]
    Puor,
}
impl ::std::fmt::Display for DocumentType6Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Msin => f.write_str("MSIN"),
            Self::Cnfa => f.write_str("CNFA"),
            Self::Dnfa => f.write_str("DNFA"),
            Self::Cinv => f.write_str("CINV"),
            Self::Cren => f.write_str("CREN"),
            Self::Debn => f.write_str("DEBN"),
            Self::Hiri => f.write_str("HIRI"),
            Self::Sbin => f.write_str("SBIN"),
            Self::Cmcn => f.write_str("CMCN"),
            Self::Soac => f.write_str("SOAC"),
            Self::Disp => f.write_str("DISP"),
            Self::Bold => f.write_str("BOLD"),
            Self::Vchr => f.write_str("VCHR"),
            Self::Aroi => f.write_str("AROI"),
            Self::Tsut => f.write_str("TSUT"),
            Self::Puor => f.write_str("PUOR"),
        }
    }
}
impl ::std::str::FromStr for DocumentType6Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MSIN" => Ok(Self::Msin),
            "CNFA" => Ok(Self::Cnfa),
            "DNFA" => Ok(Self::Dnfa),
            "CINV" => Ok(Self::Cinv),
            "CREN" => Ok(Self::Cren),
            "DEBN" => Ok(Self::Debn),
            "HIRI" => Ok(Self::Hiri),
            "SBIN" => Ok(Self::Sbin),
            "CMCN" => Ok(Self::Cmcn),
            "SOAC" => Ok(Self::Soac),
            "DISP" => Ok(Self::Disp),
            "BOLD" => Ok(Self::Bold),
            "VCHR" => Ok(Self::Vchr),
            "AROI" => Ok(Self::Aroi),
            "TSUT" => Ok(Self::Tsut),
            "PUOR" => Ok(Self::Puor),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DocumentType6Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DocumentType6Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DocumentType6Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**Specifies the external account identification scheme name code in the format of character string with a maximum length of 4 characters.
The list of valid codes is an external code list published separately.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the external account identification scheme name code in the format of character string with a maximum length of 4 characters.\r\nThe list of valid codes is an external code list published separately.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalAccountIdentification1Code(::std::string::String);
impl ::std::ops::Deref for ExternalAccountIdentification1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalAccountIdentification1Code> for ::std::string::String {
    fn from(value: ExternalAccountIdentification1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalAccountIdentification1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalAccountIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ExternalAccountIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ExternalAccountIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalAccountIdentification1Code {
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
/**Specifies the nature, or use, of the cash account in the format of character string with a maximum length of 4 characters.
The list of valid codes is an external code list published separately.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the nature, or use, of the cash account in the format of character string with a maximum length of 4 characters.\r\nThe list of valid codes is an external code list published separately.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalCashAccountType1Code(::std::string::String);
impl ::std::ops::Deref for ExternalCashAccountType1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalCashAccountType1Code> for ::std::string::String {
    fn from(value: ExternalCashAccountType1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalCashAccountType1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalCashAccountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExternalCashAccountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalCashAccountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalCashAccountType1Code {
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
/**
*`RTR`-null*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "\n*`RTR`-null",
///  "type": "string",
///  "enum": [
///    "RTR"
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
pub enum ExternalCashClearingSystem1CodeFixed {
    #[serde(rename = "RTR")]
    Rtr,
}
impl ::std::fmt::Display for ExternalCashClearingSystem1CodeFixed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Rtr => f.write_str("RTR"),
        }
    }
}
impl ::std::str::FromStr for ExternalCashClearingSystem1CodeFixed {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "RTR" => Ok(Self::Rtr),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExternalCashClearingSystem1CodeFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ExternalCashClearingSystem1CodeFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ExternalCashClearingSystem1CodeFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**Specifies the category purpose, as published in an external category purpose code list.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the category purpose, as published in an external category purpose code list.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalCategoryPurpose1Code(::std::string::String);
impl ::std::ops::Deref for ExternalCategoryPurpose1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalCategoryPurpose1Code> for ::std::string::String {
    fn from(value: ExternalCategoryPurpose1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalCategoryPurpose1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalCategoryPurpose1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExternalCategoryPurpose1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalCategoryPurpose1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalCategoryPurpose1Code {
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
/**Specifies the clearing system identification code, as published in an external clearing system identification code list.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the clearing system identification code, as published in an external clearing system identification code list.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 5,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalClearingSystemIdentification1Code(::std::string::String);
impl ::std::ops::Deref for ExternalClearingSystemIdentification1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalClearingSystemIdentification1Code>
for ::std::string::String {
    fn from(value: ExternalClearingSystemIdentification1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalClearingSystemIdentification1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 5usize {
            return Err("longer than 5 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalClearingSystemIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ExternalClearingSystemIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ExternalClearingSystemIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalClearingSystemIdentification1Code {
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
/**Specifies the nature, or use, of the amount in the format of character string with a maximum length of 4 characters.
The list of valid codes is an external code list published separately.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the nature, or use, of the amount in the format of character string with a maximum length of 4 characters.\r\nThe list of valid codes is an external code list published separately.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalDiscountAmountType1Code(::std::string::String);
impl ::std::ops::Deref for ExternalDiscountAmountType1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalDiscountAmountType1Code> for ::std::string::String {
    fn from(value: ExternalDiscountAmountType1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalDiscountAmountType1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalDiscountAmountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ExternalDiscountAmountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalDiscountAmountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalDiscountAmountType1Code {
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
///Specifies the document line type as published in an external document type code list.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the document line type as published in an external document type code list.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalDocumentLineType1Code(::std::string::String);
impl ::std::ops::Deref for ExternalDocumentLineType1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalDocumentLineType1Code> for ::std::string::String {
    fn from(value: ExternalDocumentLineType1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalDocumentLineType1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalDocumentLineType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExternalDocumentLineType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalDocumentLineType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalDocumentLineType1Code {
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
/**Specifies the external financial institution identification scheme name code in the format of character string with a maximum length of 4 characters.
The list of valid codes is an external code list published separately.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the external financial institution identification scheme name code in the format of character string with a maximum length of 4 characters.\r\nThe list of valid codes is an external code list published separately.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalFinancialInstitutionIdentification1Code(::std::string::String);
impl ::std::ops::Deref for ExternalFinancialInstitutionIdentification1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalFinancialInstitutionIdentification1Code>
for ::std::string::String {
    fn from(value: ExternalFinancialInstitutionIdentification1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalFinancialInstitutionIdentification1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalFinancialInstitutionIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ExternalFinancialInstitutionIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ExternalFinancialInstitutionIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalFinancialInstitutionIdentification1Code {
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
///Specifies the garnishment type as published in an external document type code list.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the garnishment type as published in an external document type code list.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalGarnishmentType1Code(::std::string::String);
impl ::std::ops::Deref for ExternalGarnishmentType1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalGarnishmentType1Code> for ::std::string::String {
    fn from(value: ExternalGarnishmentType1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalGarnishmentType1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalGarnishmentType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExternalGarnishmentType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalGarnishmentType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalGarnishmentType1Code {
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
/**Specifies the external organisation identification scheme name code in the format of character string with a maximum length of 4 characters.
The list of valid codes is an external code list published separately.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the external organisation identification scheme name code in the format of character string with a maximum length of 4 characters.\r\nThe list of valid codes is an external code list published separately.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalOrganisationIdentification1Code(::std::string::String);
impl ::std::ops::Deref for ExternalOrganisationIdentification1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalOrganisationIdentification1Code>
for ::std::string::String {
    fn from(value: ExternalOrganisationIdentification1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalOrganisationIdentification1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalOrganisationIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ExternalOrganisationIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ExternalOrganisationIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalOrganisationIdentification1Code {
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
/**Specifies the external person identification scheme name code in the format of character string with a maximum length of 4 characters.
The list of valid codes is an external code list published separately.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the external person identification scheme name code in the format of character string with a maximum length of 4 characters.\r\nThe list of valid codes is an external code list published separately.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalPersonIdentification1Code(::std::string::String);
impl ::std::ops::Deref for ExternalPersonIdentification1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalPersonIdentification1Code> for ::std::string::String {
    fn from(value: ExternalPersonIdentification1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalPersonIdentification1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalPersonIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ExternalPersonIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ExternalPersonIdentification1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalPersonIdentification1Code {
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
/**Specifies the external proxy account type code, as published in the proxy account type external code set.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the external proxy account type code, as published in the proxy account type external code set.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalProxyAccountType1Code(::std::string::String);
impl ::std::ops::Deref for ExternalProxyAccountType1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalProxyAccountType1Code> for ::std::string::String {
    fn from(value: ExternalProxyAccountType1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalProxyAccountType1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalProxyAccountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExternalProxyAccountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalProxyAccountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalProxyAccountType1Code {
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
/**Specifies the external purpose code in the format of character string with a maximum length of 4 characters.
The list of valid codes is an external code list published separately.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the external purpose code in the format of character string with a maximum length of 4 characters.\r\nThe list of valid codes is an external code list published separately.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalPurpose1Code(::std::string::String);
impl ::std::ops::Deref for ExternalPurpose1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalPurpose1Code> for ::std::string::String {
    fn from(value: ExternalPurpose1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalPurpose1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalPurpose1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExternalPurpose1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalPurpose1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalPurpose1Code {
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
/**Specifies the external service level code in the format of character string with a maximum length of 4 characters.
The list of valid codes is an external code list published separately.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the external service level code in the format of character string with a maximum length of 4 characters.\r\nThe list of valid codes is an external code list published separately.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalServiceLevel1Code(::std::string::String);
impl ::std::ops::Deref for ExternalServiceLevel1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalServiceLevel1Code> for ::std::string::String {
    fn from(value: ExternalServiceLevel1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalServiceLevel1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalServiceLevel1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExternalServiceLevel1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalServiceLevel1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalServiceLevel1Code {
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
/**Specifies the nature, or use, of the amount in the format of character string with a maximum length of 4 characters.
The list of valid codes is an external code list published separately.
External code sets can be downloaded from www.iso20022.org.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the nature, or use, of the amount in the format of character string with a maximum length of 4 characters.\r\nThe list of valid codes is an external code list published separately.\r\nExternal code sets can be downloaded from www.iso20022.org.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExternalTaxAmountType1Code(::std::string::String);
impl ::std::ops::Deref for ExternalTaxAmountType1Code {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExternalTaxAmountType1Code> for ::std::string::String {
    fn from(value: ExternalTaxAmountType1Code) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExternalTaxAmountType1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ExternalTaxAmountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExternalTaxAmountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalTaxAmountType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ExternalTaxAmountType1Code {
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
/**Scope
The FinancialInstitutionToFinancialInstitutionCustomerCreditTransfer message is sent by the debtor agent to the creditor agent, directly or through other agents and/or a payment clearing and settlement system. It is used to move funds from a debtor account to a creditor.
Usage
The FIToFICustomerCreditTransfer message is exchanged between agents and can contain one or more customer credit transfer instructions.
The FIToFICustomerCreditTransfer message does not allow for grouping: a CreditTransferTransactionInformation block must be present for each credit transfer transaction.
The FIToFICustomerCreditTransfer message can be used in different ways:
- If the instructing agent and the instructed agent wish to use their direct account relationship in the currency of the transfer then the message contains both the funds for the customer transfer(s) as well as the payment details;
- If the instructing agent and the instructed agent have no direct account relationship in the currency of the transfer, or do not wish to use their account relationship, then other (reimbursement) agents will be involved to cover for the customer transfer(s). The FIToFICustomerCreditTransfer contains only the payment details and the instructing agent must cover the customer transfer by sending a FinancialInstitutionCreditTransfer to a reimbursement agent. This payment method is called the Cover method;
- If more than two financial institutions are involved in the payment chain and if the FIToFICustomerCreditTransfer is sent from one financial institution to the next financial institution in the payment chain, then the payment method is called the Serial method.
The FIToFICustomerCreditTransfer message can be used in domestic and cross-border scenarios.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Scope\r\nThe FinancialInstitutionToFinancialInstitutionCustomerCreditTransfer message is sent by the debtor agent to the creditor agent, directly or through other agents and/or a payment clearing and settlement system. It is used to move funds from a debtor account to a creditor.\r\nUsage\r\nThe FIToFICustomerCreditTransfer message is exchanged between agents and can contain one or more customer credit transfer instructions.\r\nThe FIToFICustomerCreditTransfer message does not allow for grouping: a CreditTransferTransactionInformation block must be present for each credit transfer transaction.\r\nThe FIToFICustomerCreditTransfer message can be used in different ways:\r\n- If the instructing agent and the instructed agent wish to use their direct account relationship in the currency of the transfer then the message contains both the funds for the customer transfer(s) as well as the payment details;\r\n- If the instructing agent and the instructed agent have no direct account relationship in the currency of the transfer, or do not wish to use their account relationship, then other (reimbursement) agents will be involved to cover for the customer transfer(s). The FIToFICustomerCreditTransfer contains only the payment details and the instructing agent must cover the customer transfer by sending a FinancialInstitutionCreditTransfer to a reimbursement agent. This payment method is called the Cover method;\r\n- If more than two financial institutions are involved in the payment chain and if the FIToFICustomerCreditTransfer is sent from one financial institution to the next financial institution in the payment chain, then the payment method is called the Serial method.\r\nThe FIToFICustomerCreditTransfer message can be used in domestic and cross-border scenarios.",
///  "type": "object",
///  "required": [
///    "credit_transfer_transaction_information",
///    "group_header"
///  ],
///  "properties": {
///    "credit_transfer_transaction_information": {
///      "description": "Set of elements providing information specific to the individual credit transfer(s).",
///      "$ref": "#/definitions/CreditTransferTransaction39__1"
///    },
///    "group_header": {
///      "description": "Set of characteristics shared by all individual transactions included in the message.",
///      "$ref": "#/definitions/GroupHeader93__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FiToFiCustomerCreditTransferV08 {
    ///Set of elements providing information specific to the individual credit transfer(s).
    pub credit_transfer_transaction_information: CreditTransferTransaction391,
    ///Set of characteristics shared by all individual transactions included in the message.
    pub group_header: GroupHeader931,
}
impl FiToFiCustomerCreditTransferV08 {
    pub fn builder() -> builder::FiToFiCustomerCreditTransferV08 {
        Default::default()
    }
}
///Sets of elements to identify a name of the organisation identification scheme.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Sets of elements to identify a name of the organisation identification scheme.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Name of the identification scheme, in a coded form as published in an external list.",
///          "$ref": "#/definitions/ExternalFinancialInstitutionIdentification1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Name of the identification scheme, in a free text form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum FinancialIdentificationSchemeName1Choice {
    #[serde(rename = "code")]
    Code(ExternalFinancialInstitutionIdentification1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalFinancialInstitutionIdentification1Code>
for FinancialIdentificationSchemeName1Choice {
    fn from(value: ExternalFinancialInstitutionIdentification1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for FinancialIdentificationSchemeName1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
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
///    "bicfi": {
///      "description": "Code allocated to a financial institution by the ISO 9362 Registration Authority as described in ISO 9362 \"Banking - Banking telecommunication messages - Business identifier code (BIC)\".",
///      "$ref": "#/definitions/BICFIDec2014Identifier"
///    },
///    "clearing_system_member_identification": {
///      "description": "Information used to identify a member within a clearing system.",
///      "$ref": "#/definitions/ClearingSystemMemberIdentification2__1"
///    },
///    "lei": {
///      "description": "Legal entity identifier of the financial institution.",
///      "$ref": "#/definitions/LEIIdentifier"
///    },
///    "name": {
///      "description": "Name by which an agent is known and which is usually used to identify that agent.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "postal_address": {
///      "description": "Information that locates and identifies a specific address, as defined by postal services.",
///      "$ref": "#/definitions/PostalAddress24__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FinancialInstitutionIdentification181 {
    ///Code allocated to a financial institution by the ISO 9362 Registration Authority as described in ISO 9362 "Banking - Banking telecommunication messages - Business identifier code (BIC)".
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bicfi: ::std::option::Option<BicfiDec2014Identifier>,
    ///Information used to identify a member within a clearing system.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub clearing_system_member_identification: ::std::option::Option<
        ClearingSystemMemberIdentification21,
    >,
    ///Legal entity identifier of the financial institution.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lei: ::std::option::Option<LeiIdentifier>,
    ///Name by which an agent is known and which is usually used to identify that agent.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Max140Text>,
    ///Information that locates and identifies a specific address, as defined by postal services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub postal_address: ::std::option::Option<PostalAddress241>,
}
impl ::std::default::Default for FinancialInstitutionIdentification181 {
    fn default() -> Self {
        Self {
            bicfi: Default::default(),
            clearing_system_member_identification: Default::default(),
            lei: Default::default(),
            name: Default::default(),
            postal_address: Default::default(),
        }
    }
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
///    "bicfi": {
///      "description": "Code allocated to a financial institution by the ISO 9362 Registration Authority as described in ISO 9362 \"Banking - Banking telecommunication messages - Business identifier code (BIC)\".",
///      "$ref": "#/definitions/BICFIDec2014Identifier"
///    },
///    "clearing_system_member_identification": {
///      "description": "Information used to identify a member within a clearing system.",
///      "$ref": "#/definitions/ClearingSystemMemberIdentification2"
///    },
///    "lei": {
///      "description": "Legal entity identifier of the financial institution.",
///      "$ref": "#/definitions/LEIIdentifier"
///    },
///    "name": {
///      "description": "Name by which an agent is known and which is usually used to identify that agent.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "other": {
///      "description": "Unique identification of an agent, as assigned by an institution, using an identification scheme.",
///      "$ref": "#/definitions/GenericFinancialIdentification1"
///    },
///    "postal_address": {
///      "description": "Information that locates and identifies a specific address, as defined by postal services.",
///      "$ref": "#/definitions/PostalAddress24__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FinancialInstitutionIdentification182 {
    ///Code allocated to a financial institution by the ISO 9362 Registration Authority as described in ISO 9362 "Banking - Banking telecommunication messages - Business identifier code (BIC)".
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bicfi: ::std::option::Option<BicfiDec2014Identifier>,
    ///Information used to identify a member within a clearing system.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub clearing_system_member_identification: ::std::option::Option<
        ClearingSystemMemberIdentification2,
    >,
    ///Legal entity identifier of the financial institution.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lei: ::std::option::Option<LeiIdentifier>,
    ///Name by which an agent is known and which is usually used to identify that agent.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Max140Text>,
    ///Unique identification of an agent, as assigned by an institution, using an identification scheme.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub other: ::std::option::Option<GenericFinancialIdentification1>,
    ///Information that locates and identifies a specific address, as defined by postal services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub postal_address: ::std::option::Option<PostalAddress241>,
}
impl ::std::default::Default for FinancialInstitutionIdentification182 {
    fn default() -> Self {
        Self {
            bicfi: Default::default(),
            clearing_system_member_identification: Default::default(),
            lei: Default::default(),
            name: Default::default(),
            other: Default::default(),
            postal_address: Default::default(),
        }
    }
}
impl FinancialInstitutionIdentification182 {
    pub fn builder() -> builder::FinancialInstitutionIdentification182 {
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
///      "$ref": "#/definitions/ClearingSystemMemberIdentification2__2"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FinancialInstitutionIdentification183 {
    ///Information used to identify a member within a clearing system.
    pub clearing_system_member_identification: ClearingSystemMemberIdentification22,
}
impl FinancialInstitutionIdentification183 {
    pub fn builder() -> builder::FinancialInstitutionIdentification183 {
        Default::default()
    }
}
///Provides remittance information about a payment for garnishment-related purposes.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides remittance information about a payment for garnishment-related purposes.",
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "date": {
///      "description": "Date of payment which garnishment was taken from.",
///      "$ref": "#/definitions/ISODate"
///    },
///    "employee_termination_indicator": {
///      "description": "Indicates if the employment of the person to whom the garnishment applies (that is, the ultimate debtor) has been terminated.",
///      "$ref": "#/definitions/TrueFalseIndicator"
///    },
///    "family_medical_insurance_indicator": {
///      "description": "Indicates if the person to whom the garnishment applies (that is, the ultimate debtor) has family medical insurance coverage available.",
///      "$ref": "#/definitions/TrueFalseIndicator"
///    },
///    "garnishee": {
///      "description": "Ultimate party that owes an amount of money to the (ultimate) creditor, in this case, to the garnisher.",
///      "$ref": "#/definitions/PartyIdentification135__5"
///    },
///    "garnishment_administrator": {
///      "description": "Party on the credit side of the transaction who administers the garnishment on behalf of the ultimate beneficiary.",
///      "$ref": "#/definitions/PartyIdentification135__5"
///    },
///    "reference_number": {
///      "description": "Reference information that is specific to the agency receiving the garnishment.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "remitted_amount": {
///      "description": "Amount of money remitted for the referred document.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "type": {
///      "description": "Specifies the type of garnishment.",
///      "$ref": "#/definitions/GarnishmentType1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Garnishment31 {
    ///Date of payment which garnishment was taken from.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<IsoDate>,
    ///Indicates if the employment of the person to whom the garnishment applies (that is, the ultimate debtor) has been terminated.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub employee_termination_indicator: ::std::option::Option<TrueFalseIndicator>,
    ///Indicates if the person to whom the garnishment applies (that is, the ultimate debtor) has family medical insurance coverage available.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub family_medical_insurance_indicator: ::std::option::Option<TrueFalseIndicator>,
    ///Ultimate party that owes an amount of money to the (ultimate) creditor, in this case, to the garnisher.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub garnishee: ::std::option::Option<PartyIdentification1355>,
    ///Party on the credit side of the transaction who administers the garnishment on behalf of the ultimate beneficiary.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub garnishment_administrator: ::std::option::Option<PartyIdentification1355>,
    ///Reference information that is specific to the agency receiving the garnishment.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reference_number: ::std::option::Option<Max140Text>,
    ///Amount of money remitted for the referred document.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remitted_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Specifies the type of garnishment.
    #[serde(rename = "type")]
    pub type_: GarnishmentType1,
}
impl Garnishment31 {
    pub fn builder() -> builder::Garnishment31 {
        Default::default()
    }
}
///Specifies the type of garnishment.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the type of garnishment.",
///  "type": "object",
///  "required": [
///    "code_or_proprietary"
///  ],
///  "properties": {
///    "code_or_proprietary": {
///      "description": "Provides the type details of the garnishment.",
///      "$ref": "#/definitions/GarnishmentType1Choice"
///    },
///    "issuer": {
///      "description": "Identification of the issuer of the garnishment type.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GarnishmentType1 {
    ///Provides the type details of the garnishment.
    pub code_or_proprietary: GarnishmentType1Choice,
    ///Identification of the issuer of the garnishment type.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer: ::std::option::Option<Max35Text>,
}
impl GarnishmentType1 {
    pub fn builder() -> builder::GarnishmentType1 {
        Default::default()
    }
}
///Specifies the type of garnishment.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the type of garnishment.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Garnishment type in a coded form. Would suggest this to be an External Code List to contain: GNCS    Garnishment from a third party payer for Child Support GNDP    Garnishment from a Direct Payer for Child Support GTPP     Garnishment from a third party payer to taxing agency.",
///          "$ref": "#/definitions/ExternalGarnishmentType1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Proprietary identification of the type of garnishment.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum GarnishmentType1Choice {
    #[serde(rename = "code")]
    Code(ExternalGarnishmentType1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalGarnishmentType1Code> for GarnishmentType1Choice {
    fn from(value: ExternalGarnishmentType1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for GarnishmentType1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
///Information related to a generic account identification.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information related to a generic account identification.",
///  "type": "object",
///  "required": [
///    "identification"
///  ],
///  "properties": {
///    "identification": {
///      "description": "Identification assigned by an institution.",
///      "$ref": "#/definitions/Max34Text"
///    },
///    "issuer": {
///      "description": "Entity that assigns the identification.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "scheme_name": {
///      "description": "Name of the identification scheme.",
///      "$ref": "#/definitions/AccountSchemeName1Choice"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GenericAccountIdentification1 {
    ///Identification assigned by an institution.
    pub identification: Max34Text,
    ///Entity that assigns the identification.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer: ::std::option::Option<Max35Text>,
    ///Name of the identification scheme.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scheme_name: ::std::option::Option<AccountSchemeName1Choice>,
}
impl GenericAccountIdentification1 {
    pub fn builder() -> builder::GenericAccountIdentification1 {
        Default::default()
    }
}
///Information related to an identification of a financial institution.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information related to an identification of a financial institution.",
///  "type": "object",
///  "required": [
///    "identification"
///  ],
///  "properties": {
///    "identification": {
///      "description": "Unique and unambiguous identification of a person.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "issuer": {
///      "description": "Entity that assigns the identification.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "scheme_name": {
///      "description": "Name of the identification scheme.",
///      "$ref": "#/definitions/FinancialIdentificationSchemeName1Choice"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GenericFinancialIdentification1 {
    ///Unique and unambiguous identification of a person.
    pub identification: Max35Text,
    ///Entity that assigns the identification.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer: ::std::option::Option<Max35Text>,
    ///Name of the identification scheme.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scheme_name: ::std::option::Option<FinancialIdentificationSchemeName1Choice>,
}
impl GenericFinancialIdentification1 {
    pub fn builder() -> builder::GenericFinancialIdentification1 {
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
///    },
///    "issuer": {
///      "description": "Entity that assigns the identification.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "scheme_name": {
///      "description": "Name of the identification scheme.",
///      "$ref": "#/definitions/OrganisationIdentificationSchemeName1Choice"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GenericOrganisationIdentification1 {
    ///Identification assigned by an institution.
    pub identification: Max35Text,
    ///Entity that assigns the identification.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer: ::std::option::Option<Max35Text>,
    ///Name of the identification scheme.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scheme_name: ::std::option::Option<OrganisationIdentificationSchemeName1Choice>,
}
impl GenericOrganisationIdentification1 {
    pub fn builder() -> builder::GenericOrganisationIdentification1 {
        Default::default()
    }
}
///Information related to an identification of a person.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information related to an identification of a person.",
///  "type": "object",
///  "required": [
///    "identification"
///  ],
///  "properties": {
///    "identification": {
///      "description": "Unique and unambiguous identification of a person.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "issuer": {
///      "description": "Entity that assigns the identification.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "scheme_name": {
///      "description": "Name of the identification scheme.",
///      "$ref": "#/definitions/PersonIdentificationSchemeName1Choice"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GenericPersonIdentification1 {
    ///Unique and unambiguous identification of a person.
    pub identification: Max35Text,
    ///Entity that assigns the identification.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer: ::std::option::Option<Max35Text>,
    ///Name of the identification scheme.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scheme_name: ::std::option::Option<PersonIdentificationSchemeName1Choice>,
}
impl GenericPersonIdentification1 {
    pub fn builder() -> builder::GenericPersonIdentification1 {
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
///    "message_identification",
///    "number_of_transactions",
///    "settlement_information"
///  ],
///  "properties": {
///    "creation_date_time": {
///      "description": "Date and time at which the message was created.",
///      "$ref": "#/definitions/ISONormalisedDateTime"
///    },
///    "message_identification": {
///      "description": "Point to point reference, as assigned by the instructing party, and sent to the next party in the chain to unambiguously identify the message. Usage: The instructing party has to make sure that MessageIdentification is unique per instructed party for a pre-agreed period.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "number_of_transactions": {
///      "description": "Number of individual transactions contained in the message.",
///      "$ref": "#/definitions/Max15NumericText_fixed"
///    },
///    "settlement_information": {
///      "description": "Specifies the details on how the settlement of the transaction(s) between the instructing agent and the instructed agent is completed.",
///      "$ref": "#/definitions/SettlementInstruction7__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GroupHeader931 {
    ///Date and time at which the message was created.
    pub creation_date_time: IsoNormalisedDateTime,
    ///Point to point reference, as assigned by the instructing party, and sent to the next party in the chain to unambiguously identify the message. Usage: The instructing party has to make sure that MessageIdentification is unique per instructed party for a pre-agreed period.
    pub message_identification: Max35Text,
    ///Number of individual transactions contained in the message.
    pub number_of_transactions: Max15NumericTextFixed,
    ///Specifies the details on how the settlement of the transaction(s) between the instructing agent and the instructed agent is completed.
    pub settlement_information: SettlementInstruction71,
}
impl GroupHeader931 {
    pub fn builder() -> builder::GroupHeader931 {
        Default::default()
    }
}
///The International Bank Account Number is a code used internationally by financial institutions to uniquely identify the account of a customer at a financial institution as described in the 2007 edition of the ISO 13616 standard "Banking and related financial services - International Bank Account Number (IBAN)" and replaced by the more recent edition of the standard.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The International Bank Account Number is a code used internationally by financial institutions to uniquely identify the account of a customer at a financial institution as described in the 2007 edition of the ISO 13616 standard \"Banking and related financial services - International Bank Account Number (IBAN)\" and replaced by the more recent edition of the standard.",
///  "type": "string",
///  "pattern": "^[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Iban2007Identifier(::std::string::String);
impl ::std::ops::Deref for Iban2007Identifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Iban2007Identifier> for ::std::string::String {
    fn from(value: Iban2007Identifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Iban2007Identifier {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Iban2007Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Iban2007Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Iban2007Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Iban2007Identifier {
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
/**Specifies further instructions concerning the processing of a payment instruction, provided by the sending clearing agent to the next agent(s).
*`CHQB`-(Ultimate) creditor must be paid by cheque.
*`HOLD`-Amount of money must be held for the (ultimate) creditor, who will call. Pay on identification.
*`PHOB`-Please advise/contact (ultimate) creditor/claimant by phone.
*`TELB`-Please advise/contact (ultimate) creditor/claimant by the most efficient means of telecommunication.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies further instructions concerning the processing of a payment instruction, provided by the sending clearing agent to the next agent(s).\n*`CHQB`-(Ultimate) creditor must be paid by cheque.\n*`HOLD`-Amount of money must be held for the (ultimate) creditor, who will call. Pay on identification.\n*`PHOB`-Please advise/contact (ultimate) creditor/claimant by phone.\n*`TELB`-Please advise/contact (ultimate) creditor/claimant by the most efficient means of telecommunication.",
///  "type": "string",
///  "enum": [
///    "CHQB",
///    "HOLD",
///    "PHOB",
///    "TELB"
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
pub enum Instruction3Code {
    #[serde(rename = "CHQB")]
    Chqb,
    #[serde(rename = "HOLD")]
    Hold,
    #[serde(rename = "PHOB")]
    Phob,
    #[serde(rename = "TELB")]
    Telb,
}
impl ::std::fmt::Display for Instruction3Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Chqb => f.write_str("CHQB"),
            Self::Hold => f.write_str("HOLD"),
            Self::Phob => f.write_str("PHOB"),
            Self::Telb => f.write_str("TELB"),
        }
    }
}
impl ::std::str::FromStr for Instruction3Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CHQB" => Ok(Self::Chqb),
            "HOLD" => Ok(Self::Hold),
            "PHOB" => Ok(Self::Phob),
            "TELB" => Ok(Self::Telb),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Instruction3Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Instruction3Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Instruction3Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Further information related to the processing of the payment instruction that may need to be acted upon by the creditor's agent. The instruction may relate to a level of service, or may be an instruction that has to be executed by the creditor's agent, or may be information required by the creditor's agent.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Further information related to the processing of the payment instruction that may need to be acted upon by the creditor's agent. The instruction may relate to a level of service, or may be an instruction that has to be executed by the creditor's agent, or may be information required by the creditor's agent.",
///  "type": "object",
///  "properties": {
///    "code": {
///      "description": "Coded information related to the processing of the payment instruction, provided by the initiating party, and intended for the creditor's agent.",
///      "$ref": "#/definitions/Instruction3Code"
///    },
///    "instruction_information": {
///      "description": "Further information complementing the coded instruction or instruction to the creditor's agent that is bilaterally agreed or specific to a user community.",
///      "$ref": "#/definitions/Max140Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct InstructionForCreditorAgent1 {
    ///Coded information related to the processing of the payment instruction, provided by the initiating party, and intended for the creditor's agent.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<Instruction3Code>,
    ///Further information complementing the coded instruction or instruction to the creditor's agent that is bilaterally agreed or specific to a user community.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instruction_information: ::std::option::Option<Max140Text>,
}
impl ::std::default::Default for InstructionForCreditorAgent1 {
    fn default() -> Self {
        Self {
            code: Default::default(),
            instruction_information: Default::default(),
        }
    }
}
impl InstructionForCreditorAgent1 {
    pub fn builder() -> builder::InstructionForCreditorAgent1 {
        Default::default()
    }
}
///A particular point in the progression of time in a calendar year expressed in the YYYY-MM-DD format. This representation is defined in "XML Schema Part 2: Datatypes Second Edition - W3C Recommendation 28 October 2004" which is aligned with ISO 8601.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A particular point in the progression of time in a calendar year expressed in the YYYY-MM-DD format. This representation is defined in \"XML Schema Part 2: Datatypes Second Edition - W3C Recommendation 28 October 2004\" which is aligned with ISO 8601.",
///  "type": "string",
///  "pattern": "^(?:[1-9]\\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)(?:Z|[+-][01]\\d:[0-5]\\d)?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct IsoDate(::std::string::String);
impl ::std::ops::Deref for IsoDate {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IsoDate> for ::std::string::String {
    fn from(value: IsoDate) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for IsoDate {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^(?:[1-9]\\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)(?:Z|[+-][01]\\d:[0-5]\\d)?$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(?:[1-9]\\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)(?:Z|[+-][01]\\d:[0-5]\\d)?$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for IsoDate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IsoDate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IsoDate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for IsoDate {
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
///Legal Entity Identifier is a code allocated to a party as described in ISO 17442 "Financial Services - Legal Entity Identifier (LEI)".
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Legal Entity Identifier is a code allocated to a party as described in ISO 17442 \"Financial Services - Legal Entity Identifier (LEI)\".",
///  "type": "string",
///  "pattern": "^[A-Z0-9]{18,18}[0-9]{2,2}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LeiIdentifier(::std::string::String);
impl ::std::ops::Deref for LeiIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LeiIdentifier> for ::std::string::String {
    fn from(value: LeiIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LeiIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[A-Z0-9]{18,18}[0-9]{2,2}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Z0-9]{18,18}[0-9]{2,2}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LeiIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LeiIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LeiIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LeiIdentifier {
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
///Set of elements that further identifies the type of local instruments being requested by the initiating party.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Set of elements that further identifies the type of local instruments being requested by the initiating party.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Specifies the local instrument, as a proprietary code.",
///          "$ref": "#/definitions/Max35Text"
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
pub struct LocalInstrument2Choice1 {
    ///Specifies the local instrument, as a proprietary code.
    pub proprietary: Max35Text,
}
impl LocalInstrument2Choice1 {
    pub fn builder() -> builder::LocalInstrument2Choice1 {
        Default::default()
    }
}
///Specifies a character string with a maximum length of 10 characters.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a character string with a maximum length of 10 characters.",
///  "type": "string",
///  "maxLength": 10,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Max10Text(::std::string::String);
impl ::std::ops::Deref for Max10Text {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Max10Text> for ::std::string::String {
    fn from(value: Max10Text) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Max10Text {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10usize {
            return Err("longer than 10 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Max10Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max10Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max10Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Max10Text {
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
///Specifies a character string with a maximum length of 140 characters.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a character string with a maximum length of 140 characters.",
///  "type": "string",
///  "maxLength": 140,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Max140Text(::std::string::String);
impl ::std::ops::Deref for Max140Text {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Max140Text> for ::std::string::String {
    fn from(value: Max140Text) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Max140Text {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 140usize {
            return Err("longer than 140 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Max140Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max140Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max140Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Max140Text {
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
/**
*`1`-null*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "\n*`1`-null",
///  "type": "string",
///  "enum": [
///    "1"
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
pub enum Max15NumericTextFixed {
    #[serde(rename = "1")]
    X1,
}
impl ::std::fmt::Display for Max15NumericTextFixed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1 => f.write_str("1"),
        }
    }
}
impl ::std::str::FromStr for Max15NumericTextFixed {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1" => Ok(Self::X1),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Max15NumericTextFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max15NumericTextFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max15NumericTextFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Specifies a character string with a maximum length of 16 characters.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a character string with a maximum length of 16 characters.",
///  "type": "string",
///  "maxLength": 16,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Max16Text(::std::string::String);
impl ::std::ops::Deref for Max16Text {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Max16Text> for ::std::string::String {
    fn from(value: Max16Text) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Max16Text {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 16usize {
            return Err("longer than 16 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Max16Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max16Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max16Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Max16Text {
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
///Specifies a character string with a maximum length of 2048 characters.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a character string with a maximum length of 2048 characters.",
///  "type": "string",
///  "maxLength": 2048,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Max2048Text(::std::string::String);
impl ::std::ops::Deref for Max2048Text {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Max2048Text> for ::std::string::String {
    fn from(value: Max2048Text) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Max2048Text {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2048usize {
            return Err("longer than 2048 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Max2048Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max2048Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max2048Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Max2048Text {
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
///Specifies a character string with a maximum length of 34 characters.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a character string with a maximum length of 34 characters.",
///  "type": "string",
///  "maxLength": 34,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Max34Text(::std::string::String);
impl ::std::ops::Deref for Max34Text {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Max34Text> for ::std::string::String {
    fn from(value: Max34Text) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Max34Text {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 34usize {
            return Err("longer than 34 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Max34Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max34Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max34Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Max34Text {
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
///Specifies a character string with a maximum length of 4 characters.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a character string with a maximum length of 4 characters.",
///  "type": "string",
///  "maxLength": 4,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Max4Text(::std::string::String);
impl ::std::ops::Deref for Max4Text {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Max4Text> for ::std::string::String {
    fn from(value: Max4Text) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Max4Text {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4usize {
            return Err("longer than 4 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Max4Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max4Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max4Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Max4Text {
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
///Specifies a character string with a maximum length of 70characters.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a character string with a maximum length of 70characters.",
///  "type": "string",
///  "maxLength": 70,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Max70Text(::std::string::String);
impl ::std::ops::Deref for Max70Text {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Max70Text> for ::std::string::String {
    fn from(value: Max70Text) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Max70Text {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 70usize {
            return Err("longer than 70 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Max70Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max70Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max70Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Max70Text {
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
///Information that locates and identifies a party.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information that locates and identifies a party.",
///  "type": "object",
///  "required": [
///    "address",
///    "name"
///  ],
///  "properties": {
///    "address": {
///      "description": "Postal address of a party.",
///      "$ref": "#/definitions/PostalAddress24__1"
///    },
///    "name": {
///      "description": "Name by which a party is known and is usually used to identify that party.",
///      "$ref": "#/definitions/Max140Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NameAndAddress161 {
    ///Postal address of a party.
    pub address: PostalAddress241,
    ///Name by which a party is known and is usually used to identify that party.
    pub name: Max140Text,
}
impl NameAndAddress161 {
    pub fn builder() -> builder::NameAndAddress161 {
        Default::default()
    }
}
///Number of objects represented as an integer.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Number of objects represented as an integer.",
///  "type": "string",
///  "maxLength": 19
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Number(::std::string::String);
impl ::std::ops::Deref for Number {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Number> for ::std::string::String {
    fn from(value: Number) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Number {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 19usize {
            return Err("longer than 19 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Number {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Number {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Number {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Number {
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
///  "properties": {
///    "any_bic": {
///      "description": "Business identification code of the organisation.",
///      "$ref": "#/definitions/AnyBICDec2014Identifier"
///    },
///    "lei": {
///      "description": "Legal entity identification as an alternate identification for a party.",
///      "$ref": "#/definitions/LEIIdentifier"
///    },
///    "other": {
///      "description": "Unique identification of an organisation, as assigned by an institution, using an identification scheme.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/GenericOrganisationIdentification1"
///      },
///      "maxItems": 2
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OrganisationIdentification291 {
    ///Business identification code of the organisation.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub any_bic: ::std::option::Option<AnyBicDec2014Identifier>,
    ///Legal entity identification as an alternate identification for a party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lei: ::std::option::Option<LeiIdentifier>,
    ///Unique identification of an organisation, as assigned by an institution, using an identification scheme.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub other: ::std::vec::Vec<GenericOrganisationIdentification1>,
}
impl ::std::default::Default for OrganisationIdentification291 {
    fn default() -> Self {
        Self {
            any_bic: Default::default(),
            lei: Default::default(),
            other: Default::default(),
        }
    }
}
impl OrganisationIdentification291 {
    pub fn builder() -> builder::OrganisationIdentification291 {
        Default::default()
    }
}
///Sets of elements to identify a name of the organisation identification scheme.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Sets of elements to identify a name of the organisation identification scheme.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Name of the identification scheme, in a coded form as published in an external list.",
///          "$ref": "#/definitions/ExternalOrganisationIdentification1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Name of the identification scheme, in a free text form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum OrganisationIdentificationSchemeName1Choice {
    #[serde(rename = "code")]
    Code(ExternalOrganisationIdentification1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalOrganisationIdentification1Code>
for OrganisationIdentificationSchemeName1Choice {
    fn from(value: ExternalOrganisationIdentification1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for OrganisationIdentificationSchemeName1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
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
///    },
///    {
///      "type": "object",
///      "required": [
///        "private_identification"
///      ],
///      "properties": {
///        "private_identification": {
///          "description": "Unique and unambiguous identification of a person, for example a passport.",
///          "$ref": "#/definitions/PersonIdentification13__1"
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
pub enum Party38Choice1 {
    #[serde(rename = "organisation_identification")]
    OrganisationIdentification(OrganisationIdentification291),
    #[serde(rename = "private_identification")]
    PrivateIdentification(PersonIdentification131),
}
impl ::std::convert::From<OrganisationIdentification291> for Party38Choice1 {
    fn from(value: OrganisationIdentification291) -> Self {
        Self::OrganisationIdentification(value)
    }
}
impl ::std::convert::From<PersonIdentification131> for Party38Choice1 {
    fn from(value: PersonIdentification131) -> Self {
        Self::PrivateIdentification(value)
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
///    "name"
///  ],
///  "properties": {
///    "country_of_residence": {
///      "description": "Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.",
///      "$ref": "#/definitions/CountryCode"
///    },
///    "identification": {
///      "description": "Unique and unambiguous identification of a party.",
///      "$ref": "#/definitions/Party38Choice__1"
///    },
///    "name": {
///      "description": "Name by which a party is known and which is usually used to identify that party.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "postal_address": {
///      "description": "Information that locates and identifies a specific address, as defined by postal services.",
///      "$ref": "#/definitions/PostalAddress24__2"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PartyIdentification1351 {
    ///Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country_of_residence: ::std::option::Option<CountryCode>,
    ///Unique and unambiguous identification of a party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub identification: ::std::option::Option<Party38Choice1>,
    ///Name by which a party is known and which is usually used to identify that party.
    pub name: Max140Text,
    ///Information that locates and identifies a specific address, as defined by postal services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub postal_address: ::std::option::Option<PostalAddress242>,
}
impl PartyIdentification1351 {
    pub fn builder() -> builder::PartyIdentification1351 {
        Default::default()
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
///    "name"
///  ],
///  "properties": {
///    "contact_details": {
///      "description": "Set of elements used to indicate how to contact the party.",
///      "$ref": "#/definitions/Contact4__1"
///    },
///    "country_of_residence": {
///      "description": "Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.",
///      "$ref": "#/definitions/CountryCode"
///    },
///    "identification": {
///      "description": "Unique and unambiguous identification of a party.",
///      "$ref": "#/definitions/Party38Choice__1"
///    },
///    "name": {
///      "description": "Name by which a party is known and which is usually used to identify that party.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "postal_address": {
///      "description": "Information that locates and identifies a specific address, as defined by postal services.",
///      "$ref": "#/definitions/PostalAddress24__2"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PartyIdentification1352 {
    ///Set of elements used to indicate how to contact the party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contact_details: ::std::option::Option<Contact41>,
    ///Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country_of_residence: ::std::option::Option<CountryCode>,
    ///Unique and unambiguous identification of a party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub identification: ::std::option::Option<Party38Choice1>,
    ///Name by which a party is known and which is usually used to identify that party.
    pub name: Max140Text,
    ///Information that locates and identifies a specific address, as defined by postal services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub postal_address: ::std::option::Option<PostalAddress242>,
}
impl PartyIdentification1352 {
    pub fn builder() -> builder::PartyIdentification1352 {
        Default::default()
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
///    "name"
///  ],
///  "properties": {
///    "contact_details": {
///      "description": "Set of elements used to indicate how to contact the party.",
///      "$ref": "#/definitions/Contact4__2"
///    },
///    "country_of_residence": {
///      "description": "Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.",
///      "$ref": "#/definitions/CountryCode"
///    },
///    "identification": {
///      "description": "Unique and unambiguous identification of a party.",
///      "$ref": "#/definitions/Party38Choice__1"
///    },
///    "name": {
///      "description": "Name by which a party is known and which is usually used to identify that party.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "postal_address": {
///      "description": "Information that locates and identifies a specific address, as defined by postal services.",
///      "$ref": "#/definitions/PostalAddress24__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PartyIdentification1353 {
    ///Set of elements used to indicate how to contact the party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contact_details: ::std::option::Option<Contact42>,
    ///Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country_of_residence: ::std::option::Option<CountryCode>,
    ///Unique and unambiguous identification of a party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub identification: ::std::option::Option<Party38Choice1>,
    ///Name by which a party is known and which is usually used to identify that party.
    pub name: Max140Text,
    ///Information that locates and identifies a specific address, as defined by postal services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub postal_address: ::std::option::Option<PostalAddress241>,
}
impl PartyIdentification1353 {
    pub fn builder() -> builder::PartyIdentification1353 {
        Default::default()
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
///  "properties": {
///    "contact_details": {
///      "description": "Set of elements used to indicate how to contact the party.",
///      "$ref": "#/definitions/Contact4__3"
///    },
///    "country_of_residence": {
///      "description": "Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.",
///      "$ref": "#/definitions/CountryCode"
///    },
///    "identification": {
///      "description": "Unique and unambiguous identification of a party.",
///      "$ref": "#/definitions/Party38Choice__1"
///    },
///    "name": {
///      "description": "Name by which a party is known and which is usually used to identify that party.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "postal_address": {
///      "description": "Information that locates and identifies a specific address, as defined by postal services.",
///      "$ref": "#/definitions/PostalAddress24__2"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PartyIdentification1354 {
    ///Set of elements used to indicate how to contact the party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contact_details: ::std::option::Option<Contact43>,
    ///Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country_of_residence: ::std::option::Option<CountryCode>,
    ///Unique and unambiguous identification of a party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub identification: ::std::option::Option<Party38Choice1>,
    ///Name by which a party is known and which is usually used to identify that party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Max140Text>,
    ///Information that locates and identifies a specific address, as defined by postal services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub postal_address: ::std::option::Option<PostalAddress242>,
}
impl ::std::default::Default for PartyIdentification1354 {
    fn default() -> Self {
        Self {
            contact_details: Default::default(),
            country_of_residence: Default::default(),
            identification: Default::default(),
            name: Default::default(),
            postal_address: Default::default(),
        }
    }
}
impl PartyIdentification1354 {
    pub fn builder() -> builder::PartyIdentification1354 {
        Default::default()
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
///  "properties": {
///    "country_of_residence": {
///      "description": "Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.",
///      "$ref": "#/definitions/CountryCode"
///    },
///    "identification": {
///      "description": "Unique and unambiguous identification of a party.",
///      "$ref": "#/definitions/Party38Choice__1"
///    },
///    "name": {
///      "description": "Name by which a party is known and which is usually used to identify that party.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "postal_address": {
///      "description": "Information that locates and identifies a specific address, as defined by postal services.",
///      "$ref": "#/definitions/PostalAddress24__2"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PartyIdentification1355 {
    ///Country in which a person resides (the place of a person's home). In the case of a company, it is the country from which the affairs of that company are directed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country_of_residence: ::std::option::Option<CountryCode>,
    ///Unique and unambiguous identification of a party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub identification: ::std::option::Option<Party38Choice1>,
    ///Name by which a party is known and which is usually used to identify that party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Max140Text>,
    ///Information that locates and identifies a specific address, as defined by postal services.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub postal_address: ::std::option::Option<PostalAddress242>,
}
impl ::std::default::Default for PartyIdentification1355 {
    fn default() -> Self {
        Self {
            country_of_residence: Default::default(),
            identification: Default::default(),
            name: Default::default(),
            postal_address: Default::default(),
        }
    }
}
impl PartyIdentification1355 {
    pub fn builder() -> builder::PartyIdentification1355 {
        Default::default()
    }
}
///Provides further means of referencing a payment transaction.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides further means of referencing a payment transaction.",
///  "type": "object",
///  "required": [
///    "end_to_end_identification",
///    "uetr"
///  ],
///  "properties": {
///    "clearing_system_reference": {
///      "description": "Unique reference, as assigned by a clearing system, to unambiguously identify the instruction.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "end_to_end_identification": {
///      "description": "Unique identification, as assigned by the initiating party, to unambiguously identify the transaction. This identification is passed on, unchanged, throughout the entire end-to-end chain.  Usage: The end-to-end identification can be used for reconciliation or to link tasks relating to the transaction. It can be included in several messages related to the transaction.  Usage: In case there are technical limitations to pass on multiple references, the end-to-end identification must be passed on throughout the entire end-to-end chain.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "instruction_identification": {
///      "description": "Unique identification, as assigned by an instructing party for an instructed party, to unambiguously identify the instruction.  Usage: The instruction identification is a point to point reference that can be used between the instructing party and the instructed party to refer to the individual instruction. It can be included in several messages related to the instruction.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "transaction_identification": {
///      "description": "Unique identification, as assigned by the first instructing agent, to unambiguously identify the transaction that is passed on, unchanged, throughout the entire interbank chain.  Usage: The transaction identification can be used for reconciliation, tracking or to link tasks relating to the transaction on the interbank level.  Usage: The instructing agent has to make sure that the transaction identification is unique for a pre-agreed period.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "uetr": {
///      "description": "Universally unique identifier to provide an end-to-end reference of a payment transaction.",
///      "$ref": "#/definitions/UUIDv4Identifier"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PaymentIdentification71 {
    ///Unique reference, as assigned by a clearing system, to unambiguously identify the instruction.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub clearing_system_reference: ::std::option::Option<Max35Text>,
    ///Unique identification, as assigned by the initiating party, to unambiguously identify the transaction. This identification is passed on, unchanged, throughout the entire end-to-end chain.  Usage: The end-to-end identification can be used for reconciliation or to link tasks relating to the transaction. It can be included in several messages related to the transaction.  Usage: In case there are technical limitations to pass on multiple references, the end-to-end identification must be passed on throughout the entire end-to-end chain.
    pub end_to_end_identification: Max35Text,
    ///Unique identification, as assigned by an instructing party for an instructed party, to unambiguously identify the instruction.  Usage: The instruction identification is a point to point reference that can be used between the instructing party and the instructed party to refer to the individual instruction. It can be included in several messages related to the instruction.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instruction_identification: ::std::option::Option<Max35Text>,
    ///Unique identification, as assigned by the first instructing agent, to unambiguously identify the transaction that is passed on, unchanged, throughout the entire interbank chain.  Usage: The transaction identification can be used for reconciliation, tracking or to link tasks relating to the transaction on the interbank level.  Usage: The instructing agent has to make sure that the transaction identification is unique for a pre-agreed period.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transaction_identification: ::std::option::Option<Max35Text>,
    ///Universally unique identifier to provide an end-to-end reference of a payment transaction.
    pub uetr: UuiDv4Identifier,
}
impl PaymentIdentification71 {
    pub fn builder() -> builder::PaymentIdentification71 {
        Default::default()
    }
}
///Provides further details of the type of payment.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides further details of the type of payment.",
///  "type": "object",
///  "required": [
///    "local_instrument"
///  ],
///  "properties": {
///    "category_purpose": {
///      "description": "Specifies the high level purpose of the instruction based on a set of pre-defined categories. Usage: This is used by the initiating party to provide information concerning the processing of the payment. It is likely to trigger special processing by any of the agents involved in the payment chain.",
///      "$ref": "#/definitions/CategoryPurpose1Choice"
///    },
///    "local_instrument": {
///      "description": "User community specific instrument.  Usage: This element is used to specify a local instrument, local clearing option and/or further qualify the service or service level.",
///      "$ref": "#/definitions/LocalInstrument2Choice__1"
///    },
///    "service_level": {
///      "description": "Agreement under which or rules under which the transaction should be processed.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ServiceLevel8Choice"
///      },
///      "maxItems": 3
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PaymentTypeInformation281 {
    ///Specifies the high level purpose of the instruction based on a set of pre-defined categories. Usage: This is used by the initiating party to provide information concerning the processing of the payment. It is likely to trigger special processing by any of the agents involved in the payment chain.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub category_purpose: ::std::option::Option<CategoryPurpose1Choice>,
    ///User community specific instrument.  Usage: This element is used to specify a local instrument, local clearing option and/or further qualify the service or service level.
    pub local_instrument: LocalInstrument2Choice1,
    ///Agreement under which or rules under which the transaction should be processed.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub service_level: ::std::vec::Vec<ServiceLevel8Choice>,
}
impl PaymentTypeInformation281 {
    pub fn builder() -> builder::PaymentTypeInformation281 {
        Default::default()
    }
}
///Rate expressed as a percentage, that is, in hundredths, for example, 0.7 is 7/10 of a percent, and 7.0 is 7%.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Rate expressed as a percentage, that is, in hundredths, for example, 0.7 is 7/10 of a percent, and 7.0 is 7%.",
///  "type": "string",
///  "maxLength": 12
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PercentageRate(::std::string::String);
impl ::std::ops::Deref for PercentageRate {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PercentageRate> for ::std::string::String {
    fn from(value: PercentageRate) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PercentageRate {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 12usize {
            return Err("longer than 12 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PercentageRate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PercentageRate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PercentageRate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PercentageRate {
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
///Unique and unambiguous way to identify a person.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique and unambiguous way to identify a person.",
///  "type": "object",
///  "properties": {
///    "date_and_place_of_birth": {
///      "description": "Date and place of birth of a person.",
///      "$ref": "#/definitions/DateAndPlaceOfBirth1"
///    },
///    "other": {
///      "description": "Unique identification of a person, as assigned by an institution, using an identification scheme.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/GenericPersonIdentification1"
///      },
///      "maxItems": 2
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PersonIdentification131 {
    ///Date and place of birth of a person.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date_and_place_of_birth: ::std::option::Option<DateAndPlaceOfBirth1>,
    ///Unique identification of a person, as assigned by an institution, using an identification scheme.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub other: ::std::vec::Vec<GenericPersonIdentification1>,
}
impl ::std::default::Default for PersonIdentification131 {
    fn default() -> Self {
        Self {
            date_and_place_of_birth: Default::default(),
            other: Default::default(),
        }
    }
}
impl PersonIdentification131 {
    pub fn builder() -> builder::PersonIdentification131 {
        Default::default()
    }
}
///Sets of elements to identify a name of the identification scheme.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Sets of elements to identify a name of the identification scheme.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Name of the identification scheme, in a coded form as published in an external list.",
///          "$ref": "#/definitions/ExternalPersonIdentification1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Name of the identification scheme, in a free text form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum PersonIdentificationSchemeName1Choice {
    #[serde(rename = "code")]
    Code(ExternalPersonIdentification1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalPersonIdentification1Code>
for PersonIdentificationSchemeName1Choice {
    fn from(value: ExternalPersonIdentification1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for PersonIdentificationSchemeName1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
/**The collection of information which identifies a specific phone or FAX number as defined by telecom services.
It consists of a "+" followed by the country code (from 1 to 3 characters) then a "-" and finally, any combination of numbers, "(", ")", "+" and "-" (up to 30 characters).*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The collection of information which identifies a specific phone or FAX number as defined by telecom services.\nIt consists of a \"+\" followed by the country code (from 1 to 3 characters) then a \"-\" and finally, any combination of numbers, \"(\", \")\", \"+\" and \"-\" (up to 30 characters).",
///  "type": "string",
///  "pattern": "^\\+[0-9]{1,3}-[0-9()+\\-]{1,30}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PhoneNumber(::std::string::String);
impl ::std::ops::Deref for PhoneNumber {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PhoneNumber> for ::std::string::String {
    fn from(value: PhoneNumber) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PhoneNumber {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\+[0-9]{1,3}-[0-9()+\\-]{1,30}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^\\+[0-9]{1,3}-[0-9()+\\-]{1,30}$\"".into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PhoneNumber {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PhoneNumber {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PhoneNumber {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PhoneNumber {
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
///Information that locates and identifies a specific address, as defined by postal services.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information that locates and identifies a specific address, as defined by postal services.",
///  "type": "object",
///  "properties": {
///    "address_line": {
///      "description": "Information that locates and identifies a specific address, as defined by postal services, presented in free format text.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Max70Text"
///      },
///      "maxItems": 3
///    },
///    "building_name": {
///      "description": "Name of the building or house.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "building_number": {
///      "description": "Number that identifies the position of a building on a street.",
///      "$ref": "#/definitions/Max16Text"
///    },
///    "country": {
///      "description": "Nation with its own government.",
///      "$ref": "#/definitions/CountryCode"
///    },
///    "country_sub_division": {
///      "description": "Identifies a subdivision of a country such as state, region, county.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "department": {
///      "description": "Identification of a division of a large organisation or building.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "district_name": {
///      "description": "Identifies a subdivision within a country sub-division.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "floor": {
///      "description": "Floor or storey within a building.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "post_box": {
///      "description": "Numbered box in a post office, assigned to a person or organisation, where letters are kept until called for.",
///      "$ref": "#/definitions/Max16Text"
///    },
///    "post_code": {
///      "description": "Identifier consisting of a group of letters and/or numbers that is added to a postal address to assist the sorting of mail.",
///      "$ref": "#/definitions/Max16Text"
///    },
///    "room": {
///      "description": "Building room number.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "street_name": {
///      "description": "Name of a street or thoroughfare.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "sub_department": {
///      "description": "Identification of a sub-division of a large organisation or building.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "town_location_name": {
///      "description": "Specific location name within the town.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "town_name": {
///      "description": "Name of a built-up area, with defined boundaries, and a local government.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PostalAddress241 {
    ///Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub address_line: ::std::vec::Vec<Max70Text>,
    ///Name of the building or house.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub building_name: ::std::option::Option<Max35Text>,
    ///Number that identifies the position of a building on a street.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub building_number: ::std::option::Option<Max16Text>,
    ///Nation with its own government.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<CountryCode>,
    ///Identifies a subdivision of a country such as state, region, county.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country_sub_division: ::std::option::Option<Max35Text>,
    ///Identification of a division of a large organisation or building.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub department: ::std::option::Option<Max70Text>,
    ///Identifies a subdivision within a country sub-division.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub district_name: ::std::option::Option<Max35Text>,
    ///Floor or storey within a building.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub floor: ::std::option::Option<Max70Text>,
    ///Numbered box in a post office, assigned to a person or organisation, where letters are kept until called for.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub post_box: ::std::option::Option<Max16Text>,
    ///Identifier consisting of a group of letters and/or numbers that is added to a postal address to assist the sorting of mail.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub post_code: ::std::option::Option<Max16Text>,
    ///Building room number.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub room: ::std::option::Option<Max70Text>,
    ///Name of a street or thoroughfare.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub street_name: ::std::option::Option<Max70Text>,
    ///Identification of a sub-division of a large organisation or building.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sub_department: ::std::option::Option<Max70Text>,
    ///Specific location name within the town.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub town_location_name: ::std::option::Option<Max35Text>,
    ///Name of a built-up area, with defined boundaries, and a local government.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub town_name: ::std::option::Option<Max35Text>,
}
impl ::std::default::Default for PostalAddress241 {
    fn default() -> Self {
        Self {
            address_line: Default::default(),
            building_name: Default::default(),
            building_number: Default::default(),
            country: Default::default(),
            country_sub_division: Default::default(),
            department: Default::default(),
            district_name: Default::default(),
            floor: Default::default(),
            post_box: Default::default(),
            post_code: Default::default(),
            room: Default::default(),
            street_name: Default::default(),
            sub_department: Default::default(),
            town_location_name: Default::default(),
            town_name: Default::default(),
        }
    }
}
impl PostalAddress241 {
    pub fn builder() -> builder::PostalAddress241 {
        Default::default()
    }
}
///Information that locates and identifies a specific address, as defined by postal services.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information that locates and identifies a specific address, as defined by postal services.",
///  "type": "object",
///  "required": [
///    "country",
///    "town_name"
///  ],
///  "properties": {
///    "address_line": {
///      "description": "Information that locates and identifies a specific address, as defined by postal services, presented in free format text.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Max70Text"
///      },
///      "maxItems": 2
///    },
///    "building_name": {
///      "description": "Name of the building or house.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "building_number": {
///      "description": "Number that identifies the position of a building on a street.",
///      "$ref": "#/definitions/Max16Text"
///    },
///    "country": {
///      "description": "Nation with its own government.",
///      "$ref": "#/definitions/CountryCode"
///    },
///    "country_sub_division": {
///      "description": "Identifies a subdivision of a country such as state, region, county.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "department": {
///      "description": "Identification of a division of a large organisation or building.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "district_name": {
///      "description": "Identifies a subdivision within a country sub-division.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "floor": {
///      "description": "Floor or storey within a building.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "post_box": {
///      "description": "Numbered box in a post office, assigned to a person or organisation, where letters are kept until called for.",
///      "$ref": "#/definitions/Max16Text"
///    },
///    "post_code": {
///      "description": "Identifier consisting of a group of letters and/or numbers that is added to a postal address to assist the sorting of mail.",
///      "$ref": "#/definitions/Max16Text"
///    },
///    "room": {
///      "description": "Building room number.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "street_name": {
///      "description": "Name of a street or thoroughfare.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "sub_department": {
///      "description": "Identification of a sub-division of a large organisation or building.",
///      "$ref": "#/definitions/Max70Text"
///    },
///    "town_location_name": {
///      "description": "Specific location name within the town.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "town_name": {
///      "description": "Name of a built-up area, with defined boundaries, and a local government.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PostalAddress242 {
    ///Information that locates and identifies a specific address, as defined by postal services, presented in free format text.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub address_line: ::std::vec::Vec<Max70Text>,
    ///Name of the building or house.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub building_name: ::std::option::Option<Max35Text>,
    ///Number that identifies the position of a building on a street.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub building_number: ::std::option::Option<Max16Text>,
    ///Nation with its own government.
    pub country: CountryCode,
    ///Identifies a subdivision of a country such as state, region, county.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country_sub_division: ::std::option::Option<Max35Text>,
    ///Identification of a division of a large organisation or building.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub department: ::std::option::Option<Max70Text>,
    ///Identifies a subdivision within a country sub-division.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub district_name: ::std::option::Option<Max35Text>,
    ///Floor or storey within a building.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub floor: ::std::option::Option<Max70Text>,
    ///Numbered box in a post office, assigned to a person or organisation, where letters are kept until called for.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub post_box: ::std::option::Option<Max16Text>,
    ///Identifier consisting of a group of letters and/or numbers that is added to a postal address to assist the sorting of mail.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub post_code: ::std::option::Option<Max16Text>,
    ///Building room number.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub room: ::std::option::Option<Max70Text>,
    ///Name of a street or thoroughfare.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub street_name: ::std::option::Option<Max70Text>,
    ///Identification of a sub-division of a large organisation or building.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sub_department: ::std::option::Option<Max70Text>,
    ///Specific location name within the town.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub town_location_name: ::std::option::Option<Max35Text>,
    ///Name of a built-up area, with defined boundaries, and a local government.
    pub town_name: Max35Text,
}
impl PostalAddress242 {
    pub fn builder() -> builder::PostalAddress242 {
        Default::default()
    }
}
///Information related to a proxy  identification of the account.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information related to a proxy  identification of the account.",
///  "type": "object",
///  "required": [
///    "identification"
///  ],
///  "properties": {
///    "identification": {
///      "description": "Identification used to indicate the account identification under another specified name.",
///      "$ref": "#/definitions/Max2048Text"
///    },
///    "type": {
///      "description": "Type of the proxy identification.",
///      "$ref": "#/definitions/ProxyAccountType1Choice"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProxyAccountIdentification1 {
    ///Identification used to indicate the account identification under another specified name.
    pub identification: Max2048Text,
    ///Type of the proxy identification.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<ProxyAccountType1Choice>,
}
impl ProxyAccountIdentification1 {
    pub fn builder() -> builder::ProxyAccountIdentification1 {
        Default::default()
    }
}
///Specifies the scheme used for the identification of an account alias.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the scheme used for the identification of an account alias.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Name of the identification scheme, in a coded form as published in an external list.",
///          "$ref": "#/definitions/ExternalProxyAccountType1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Name of the identification scheme, in a free text form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum ProxyAccountType1Choice {
    #[serde(rename = "code")]
    Code(ExternalProxyAccountType1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalProxyAccountType1Code> for ProxyAccountType1Choice {
    fn from(value: ExternalProxyAccountType1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for ProxyAccountType1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
/**Specifies the underlying reason for the payment transaction.
Usage: Purpose is used by the end-customers, that is initiating party, (ultimate) debtor, (ultimate) creditor to provide information concerning the nature of the payment. Purpose is a content element, which is not used for processing by any of the agents involved in the payment chain.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the underlying reason for the payment transaction.\nUsage: Purpose is used by the end-customers, that is initiating party, (ultimate) debtor, (ultimate) creditor to provide information concerning the nature of the payment. Purpose is a content element, which is not used for processing by any of the agents involved in the payment chain.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Underlying reason for the payment transaction, as published in an external purpose code list.",
///          "$ref": "#/definitions/ExternalPurpose1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Purpose, in a proprietary form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum Purpose2Choice {
    #[serde(rename = "code")]
    Code(ExternalPurpose1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalPurpose1Code> for Purpose2Choice {
    fn from(value: ExternalPurpose1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for Purpose2Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
///Set of elements used to identify the documents referred to in the remittance information.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Set of elements used to identify the documents referred to in the remittance information.",
///  "type": "object",
///  "properties": {
///    "line_details": {
///      "description": "Set of elements used to provide the content of the referred document line.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/DocumentLineInformation1__1"
///      }
///    },
///    "number": {
///      "description": "Unique and unambiguous identification of the referred document.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "related_date": {
///      "description": "Date associated with the referred document.",
///      "$ref": "#/definitions/ISODate"
///    },
///    "type": {
///      "description": "Specifies the type of referred document.",
///      "$ref": "#/definitions/ReferredDocumentType4"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReferredDocumentInformation71 {
    ///Set of elements used to provide the content of the referred document line.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub line_details: ::std::vec::Vec<DocumentLineInformation11>,
    ///Unique and unambiguous identification of the referred document.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub number: ::std::option::Option<Max35Text>,
    ///Date associated with the referred document.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub related_date: ::std::option::Option<IsoDate>,
    ///Specifies the type of referred document.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<ReferredDocumentType4>,
}
impl ::std::default::Default for ReferredDocumentInformation71 {
    fn default() -> Self {
        Self {
            line_details: Default::default(),
            number: Default::default(),
            related_date: Default::default(),
            type_: Default::default(),
        }
    }
}
impl ReferredDocumentInformation71 {
    pub fn builder() -> builder::ReferredDocumentInformation71 {
        Default::default()
    }
}
///Specifies the type of the document referred in the remittance information.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the type of the document referred in the remittance information.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Document type in a coded form.",
///          "$ref": "#/definitions/DocumentType6Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Proprietary identification of the type of the remittance document.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum ReferredDocumentType3Choice {
    #[serde(rename = "code")]
    Code(DocumentType6Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<DocumentType6Code> for ReferredDocumentType3Choice {
    fn from(value: DocumentType6Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for ReferredDocumentType3Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
///Specifies the type of the document referred in the remittance information.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the type of the document referred in the remittance information.",
///  "type": "object",
///  "required": [
///    "code_or_proprietary"
///  ],
///  "properties": {
///    "code_or_proprietary": {
///      "description": "Provides the type details of the referred document.",
///      "$ref": "#/definitions/ReferredDocumentType3Choice"
///    },
///    "issuer": {
///      "description": "Identification of the issuer of the reference document type.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReferredDocumentType4 {
    ///Provides the type details of the referred document.
    pub code_or_proprietary: ReferredDocumentType3Choice,
    ///Identification of the issuer of the reference document type.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub issuer: ::std::option::Option<Max35Text>,
}
impl ReferredDocumentType4 {
    pub fn builder() -> builder::ReferredDocumentType4 {
        Default::default()
    }
}
///Entity requiring the regulatory reporting information.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Entity requiring the regulatory reporting information.",
///  "type": "object",
///  "properties": {
///    "country": {
///      "description": "Country of the entity that requires the regulatory reporting information.",
///      "$ref": "#/definitions/CountryCode"
///    },
///    "name": {
///      "description": "Name of the entity requiring the regulatory reporting information.",
///      "$ref": "#/definitions/Max140Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RegulatoryAuthority2 {
    ///Country of the entity that requires the regulatory reporting information.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<CountryCode>,
    ///Name of the entity requiring the regulatory reporting information.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Max140Text>,
}
impl ::std::default::Default for RegulatoryAuthority2 {
    fn default() -> Self {
        Self {
            country: Default::default(),
            name: Default::default(),
        }
    }
}
impl RegulatoryAuthority2 {
    pub fn builder() -> builder::RegulatoryAuthority2 {
        Default::default()
    }
}
///Information needed due to regulatory and/or statutory requirements.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information needed due to regulatory and/or statutory requirements.",
///  "type": "object",
///  "properties": {
///    "authority": {
///      "description": "Entity requiring the regulatory reporting information.",
///      "$ref": "#/definitions/RegulatoryAuthority2"
///    },
///    "debit_credit_reporting_indicator": {
///      "description": "Identifies whether the regulatory reporting information applies to the debit side, to the credit side or to both debit and credit sides of the transaction.",
///      "$ref": "#/definitions/RegulatoryReportingType1Code"
///    },
///    "details": {
///      "description": "Set of elements used to provide details on the regulatory reporting information.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/StructuredRegulatoryReporting3"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RegulatoryReporting3 {
    ///Entity requiring the regulatory reporting information.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub authority: ::std::option::Option<RegulatoryAuthority2>,
    ///Identifies whether the regulatory reporting information applies to the debit side, to the credit side or to both debit and credit sides of the transaction.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub debit_credit_reporting_indicator: ::std::option::Option<
        RegulatoryReportingType1Code,
    >,
    ///Set of elements used to provide details on the regulatory reporting information.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub details: ::std::vec::Vec<StructuredRegulatoryReporting3>,
}
impl ::std::default::Default for RegulatoryReporting3 {
    fn default() -> Self {
        Self {
            authority: Default::default(),
            debit_credit_reporting_indicator: Default::default(),
            details: Default::default(),
        }
    }
}
impl RegulatoryReporting3 {
    pub fn builder() -> builder::RegulatoryReporting3 {
        Default::default()
    }
}
/**Identifies whether the regulatory reporting information applies to the debit side, to the credit side or to both debit and credit sides of the transaction.
*`CRED`-Regulatory information applies to the credit side.
*`DEBT`-Regulatory information applies to the debit side.
*`BOTH`-Regulatory information applies to both credit and debit sides.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Identifies whether the regulatory reporting information applies to the debit side, to the credit side or to both debit and credit sides of the transaction.\n*`CRED`-Regulatory information applies to the credit side.\n*`DEBT`-Regulatory information applies to the debit side.\n*`BOTH`-Regulatory information applies to both credit and debit sides.",
///  "type": "string",
///  "enum": [
///    "CRED",
///    "DEBT",
///    "BOTH"
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
pub enum RegulatoryReportingType1Code {
    #[serde(rename = "CRED")]
    Cred,
    #[serde(rename = "DEBT")]
    Debt,
    #[serde(rename = "BOTH")]
    Both,
}
impl ::std::fmt::Display for RegulatoryReportingType1Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cred => f.write_str("CRED"),
            Self::Debt => f.write_str("DEBT"),
            Self::Both => f.write_str("BOTH"),
        }
    }
}
impl ::std::str::FromStr for RegulatoryReportingType1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CRED" => Ok(Self::Cred),
            "DEBT" => Ok(Self::Debt),
            "BOTH" => Ok(Self::Both),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RegulatoryReportingType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RegulatoryReportingType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RegulatoryReportingType1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Nature of the amount and currency on a document referred to in the remittance section, typically either the original amount due/payable or the amount actually remitted for the referenced document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Nature of the amount and currency on a document referred to in the remittance section, typically either the original amount due/payable or the amount actually remitted for the referenced document.",
///  "type": "object",
///  "properties": {
///    "adjustment_amount_and_reason": {
///      "description": "Specifies detailed information on the amount and reason of the document adjustment.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/DocumentAdjustment1"
///      }
///    },
///    "credit_note_amount": {
///      "description": "Amount specified for the referred document is the amount of a credit note.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "discount_applied_amount": {
///      "description": "Amount specified for the referred document is the amount of discount to be applied to the amount due and payable to the creditor.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/DiscountAmountAndType1"
///      }
///    },
///    "due_payable_amount": {
///      "description": "Amount specified is the exact amount due and payable to the creditor.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "remitted_amount": {
///      "description": "Amount of money remitted for the referred document.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "tax_amount": {
///      "description": "Quantity of cash resulting from the calculation of the tax.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/TaxAmountAndType1"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RemittanceAmount2 {
    ///Specifies detailed information on the amount and reason of the document adjustment.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub adjustment_amount_and_reason: ::std::vec::Vec<DocumentAdjustment1>,
    ///Amount specified for the referred document is the amount of a credit note.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credit_note_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Amount specified for the referred document is the amount of discount to be applied to the amount due and payable to the creditor.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub discount_applied_amount: ::std::vec::Vec<DiscountAmountAndType1>,
    ///Amount specified is the exact amount due and payable to the creditor.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub due_payable_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Amount of money remitted for the referred document.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remitted_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Quantity of cash resulting from the calculation of the tax.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tax_amount: ::std::vec::Vec<TaxAmountAndType1>,
}
impl ::std::default::Default for RemittanceAmount2 {
    fn default() -> Self {
        Self {
            adjustment_amount_and_reason: Default::default(),
            credit_note_amount: Default::default(),
            discount_applied_amount: Default::default(),
            due_payable_amount: Default::default(),
            remitted_amount: Default::default(),
            tax_amount: Default::default(),
        }
    }
}
impl RemittanceAmount2 {
    pub fn builder() -> builder::RemittanceAmount2 {
        Default::default()
    }
}
///Nature of the amount and currency on a document referred to in the remittance section, typically either the original amount due/payable or the amount actually remitted for the referenced document.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Nature of the amount and currency on a document referred to in the remittance section, typically either the original amount due/payable or the amount actually remitted for the referenced document.",
///  "type": "object",
///  "properties": {
///    "adjustment_amount_and_reason": {
///      "description": "Specifies detailed information on the amount and reason of the adjustment.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/DocumentAdjustment1"
///      }
///    },
///    "credit_note_amount": {
///      "description": "Amount of a credit note.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "discount_applied_amount": {
///      "description": "Amount of discount to be applied to the amount due and payable to the creditor.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/DiscountAmountAndType1"
///      }
///    },
///    "due_payable_amount": {
///      "description": "Amount specified is the exact amount due and payable to the creditor.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "remitted_amount": {
///      "description": "Amount of money remitted.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "tax_amount": {
///      "description": "Amount of the tax.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/TaxAmountAndType1"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RemittanceAmount3 {
    ///Specifies detailed information on the amount and reason of the adjustment.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub adjustment_amount_and_reason: ::std::vec::Vec<DocumentAdjustment1>,
    ///Amount of a credit note.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credit_note_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Amount of discount to be applied to the amount due and payable to the creditor.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub discount_applied_amount: ::std::vec::Vec<DiscountAmountAndType1>,
    ///Amount specified is the exact amount due and payable to the creditor.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub due_payable_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Amount of money remitted.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remitted_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Amount of the tax.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tax_amount: ::std::vec::Vec<TaxAmountAndType1>,
}
impl ::std::default::Default for RemittanceAmount3 {
    fn default() -> Self {
        Self {
            adjustment_amount_and_reason: Default::default(),
            credit_note_amount: Default::default(),
            discount_applied_amount: Default::default(),
            due_payable_amount: Default::default(),
            remitted_amount: Default::default(),
            tax_amount: Default::default(),
        }
    }
}
impl RemittanceAmount3 {
    pub fn builder() -> builder::RemittanceAmount3 {
        Default::default()
    }
}
///Information supplied to enable the matching/reconciliation of an entry with the items that the payment is intended to settle, such as commercial invoices in an accounts' receivable system.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information supplied to enable the matching/reconciliation of an entry with the items that the payment is intended to settle, such as commercial invoices in an accounts' receivable system.",
///  "type": "object",
///  "properties": {
///    "structured": {
///      "description": "Information supplied to enable the matching/reconciliation of an entry with the items that the payment is intended to settle, such as commercial invoices in an accounts' receivable system, in a structured form.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/StructuredRemittanceInformation16__1"
///      }
///    },
///    "unstructured": {
///      "description": "Information supplied to enable the matching/reconciliation of an entry with the items that the payment is intended to settle, such as commercial invoices in an accounts' receivable system, in an unstructured form.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Max140Text"
///      },
///      "maxItems": 3
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RemittanceInformation161 {
    ///Information supplied to enable the matching/reconciliation of an entry with the items that the payment is intended to settle, such as commercial invoices in an accounts' receivable system, in a structured form.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub structured: ::std::vec::Vec<StructuredRemittanceInformation161>,
    ///Information supplied to enable the matching/reconciliation of an entry with the items that the payment is intended to settle, such as commercial invoices in an accounts' receivable system, in an unstructured form.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub unstructured: ::std::vec::Vec<Max140Text>,
}
impl ::std::default::Default for RemittanceInformation161 {
    fn default() -> Self {
        Self {
            structured: Default::default(),
            unstructured: Default::default(),
        }
    }
}
impl RemittanceInformation161 {
    pub fn builder() -> builder::RemittanceInformation161 {
        Default::default()
    }
}
///Provides information on the remittance advice.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides information on the remittance advice.",
///  "type": "object",
///  "properties": {
///    "remittance_identification": {
///      "description": "Unique identification, as assigned by the initiating party, to unambiguously identify the remittance information sent separately from the payment instruction, such as a remittance advice.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "remittance_location_details": {
///      "description": "Set of elements used to provide information on the location and/or delivery of the remittance information.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/RemittanceLocationData1__1"
///      },
///      "maxItems": 2
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RemittanceLocation71 {
    ///Unique identification, as assigned by the initiating party, to unambiguously identify the remittance information sent separately from the payment instruction, such as a remittance advice.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remittance_identification: ::std::option::Option<Max35Text>,
    ///Set of elements used to provide information on the location and/or delivery of the remittance information.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub remittance_location_details: ::std::vec::Vec<RemittanceLocationData11>,
}
impl ::std::default::Default for RemittanceLocation71 {
    fn default() -> Self {
        Self {
            remittance_identification: Default::default(),
            remittance_location_details: Default::default(),
        }
    }
}
impl RemittanceLocation71 {
    pub fn builder() -> builder::RemittanceLocation71 {
        Default::default()
    }
}
///Provides additional details on the remittance advice.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides additional details on the remittance advice.",
///  "type": "object",
///  "required": [
///    "method"
///  ],
///  "properties": {
///    "electronic_address": {
///      "description": "Electronic address to which an agent is to send the remittance information.",
///      "$ref": "#/definitions/Max2048Text"
///    },
///    "method": {
///      "description": "Method used to deliver the remittance advice information.",
///      "$ref": "#/definitions/RemittanceLocationMethod2Code"
///    },
///    "postal_address": {
///      "description": "Postal address to which an agent is to send the remittance information.",
///      "$ref": "#/definitions/NameAndAddress16__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RemittanceLocationData11 {
    ///Electronic address to which an agent is to send the remittance information.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub electronic_address: ::std::option::Option<Max2048Text>,
    ///Method used to deliver the remittance advice information.
    pub method: RemittanceLocationMethod2Code,
    ///Postal address to which an agent is to send the remittance information.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub postal_address: ::std::option::Option<NameAndAddress161>,
}
impl RemittanceLocationData11 {
    pub fn builder() -> builder::RemittanceLocationData11 {
        Default::default()
    }
}
/**Specifies the method used to deliver the remittance advice information.
*`FAXI`-Remittance advice information must be faxed.
*`EDIC`-Remittance advice information must be sent through Electronic Data Interchange (EDI).
*`URID`-Remittance advice information needs to be sent to a Uniform Resource Identifier (URI). URI is a compact string of characters that uniquely identify an abstract or physical resource. URI's are the super-set of identifiers, such as URLs, email addresses, ftp sites, etc, and as such, provide the syntax for all of the identification schemes.
*`EMAL`-Remittance advice information must be sent through e-mail.
*`POST`-Remittance advice information must be sent through postal services.
*`SMSM`-Remittance advice information must be sent through by phone as a short message service (SMS).*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the method used to deliver the remittance advice information.\n*`FAXI`-Remittance advice information must be faxed.\n*`EDIC`-Remittance advice information must be sent through Electronic Data Interchange (EDI).\n*`URID`-Remittance advice information needs to be sent to a Uniform Resource Identifier (URI). URI is a compact string of characters that uniquely identify an abstract or physical resource. URI's are the super-set of identifiers, such as URLs, email addresses, ftp sites, etc, and as such, provide the syntax for all of the identification schemes.\n*`EMAL`-Remittance advice information must be sent through e-mail.\n*`POST`-Remittance advice information must be sent through postal services.\n*`SMSM`-Remittance advice information must be sent through by phone as a short message service (SMS).",
///  "type": "string",
///  "enum": [
///    "FAXI",
///    "EDIC",
///    "URID",
///    "EMAL",
///    "POST",
///    "SMSM"
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
pub enum RemittanceLocationMethod2Code {
    #[serde(rename = "FAXI")]
    Faxi,
    #[serde(rename = "EDIC")]
    Edic,
    #[serde(rename = "URID")]
    Urid,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "SMSM")]
    Smsm,
}
impl ::std::fmt::Display for RemittanceLocationMethod2Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Faxi => f.write_str("FAXI"),
            Self::Edic => f.write_str("EDIC"),
            Self::Urid => f.write_str("URID"),
            Self::Emal => f.write_str("EMAL"),
            Self::Post => f.write_str("POST"),
            Self::Smsm => f.write_str("SMSM"),
        }
    }
}
impl ::std::str::FromStr for RemittanceLocationMethod2Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "FAXI" => Ok(Self::Faxi),
            "EDIC" => Ok(Self::Edic),
            "URID" => Ok(Self::Urid),
            "EMAL" => Ok(Self::Emal),
            "POST" => Ok(Self::Post),
            "SMSM" => Ok(Self::Smsm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RemittanceLocationMethod2Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RemittanceLocationMethod2Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RemittanceLocationMethod2Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Specifies the service level of the transaction.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the service level of the transaction.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Specifies a pre-agreed service or level of service between the parties, as published in an external service level code list.",
///          "$ref": "#/definitions/ExternalServiceLevel1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Specifies a pre-agreed service or level of service between the parties, as a proprietary code.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum ServiceLevel8Choice {
    #[serde(rename = "code")]
    Code(ExternalServiceLevel1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalServiceLevel1Code> for ServiceLevel8Choice {
    fn from(value: ExternalServiceLevel1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for ServiceLevel8Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
///Provides further details on the settlement of the instruction.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides further details on the settlement of the instruction.",
///  "type": "object",
///  "required": [
///    "clearing_system",
///    "settlement_method"
///  ],
///  "properties": {
///    "clearing_system": {
///      "description": "Specification of a pre-agreed offering between clearing agents or the channel through which the payment instruction is processed.",
///      "$ref": "#/definitions/ClearingSystemIdentification3Choice__1"
///    },
///    "settlement_method": {
///      "description": "Method used to settle the (batch of) payment instructions.",
///      "$ref": "#/definitions/SettlementMethod1Code__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SettlementInstruction71 {
    ///Specification of a pre-agreed offering between clearing agents or the channel through which the payment instruction is processed.
    pub clearing_system: ClearingSystemIdentification3Choice1,
    ///Method used to settle the (batch of) payment instructions.
    pub settlement_method: SettlementMethod1Code1,
}
impl SettlementInstruction71 {
    pub fn builder() -> builder::SettlementInstruction71 {
        Default::default()
    }
}
/**Specifies the method used to settle the credit transfer instruction.
*`CLRG`-Settlement is done through a payment clearing system.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the method used to settle the credit transfer instruction.\n*`CLRG`-Settlement is done through a payment clearing system.",
///  "type": "string",
///  "enum": [
///    "CLRG"
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
pub enum SettlementMethod1Code1 {
    #[serde(rename = "CLRG")]
    Clrg,
}
impl ::std::fmt::Display for SettlementMethod1Code1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Clrg => f.write_str("CLRG"),
        }
    }
}
impl ::std::str::FromStr for SettlementMethod1Code1 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CLRG" => Ok(Self::Clrg),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SettlementMethod1Code1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SettlementMethod1Code1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SettlementMethod1Code1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Information needed due to regulatory and statutory requirements.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information needed due to regulatory and statutory requirements.",
///  "type": "object",
///  "properties": {
///    "amount": {
///      "description": "Amount of money to be reported for regulatory and statutory requirements.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "code": {
///      "description": "Specifies the nature, purpose, and reason for the transaction to be reported for regulatory and statutory requirements in a coded form.",
///      "$ref": "#/definitions/Max10Text"
///    },
///    "country": {
///      "description": "Country related to the specified type of regulatory reporting details.",
///      "$ref": "#/definitions/CountryCode"
///    },
///    "date": {
///      "description": "Date related to the specified type of regulatory reporting details.",
///      "$ref": "#/definitions/ISODate"
///    },
///    "information": {
///      "description": "Additional details that cater for specific domestic regulatory requirements.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Max35Text"
///      }
///    },
///    "type": {
///      "description": "Specifies the type of the information supplied in the regulatory reporting details.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StructuredRegulatoryReporting3 {
    ///Amount of money to be reported for regulatory and statutory requirements.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Specifies the nature, purpose, and reason for the transaction to be reported for regulatory and statutory requirements in a coded form.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<Max10Text>,
    ///Country related to the specified type of regulatory reporting details.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<CountryCode>,
    ///Date related to the specified type of regulatory reporting details.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<IsoDate>,
    ///Additional details that cater for specific domestic regulatory requirements.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub information: ::std::vec::Vec<Max35Text>,
    ///Specifies the type of the information supplied in the regulatory reporting details.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<Max35Text>,
}
impl ::std::default::Default for StructuredRegulatoryReporting3 {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            code: Default::default(),
            country: Default::default(),
            date: Default::default(),
            information: Default::default(),
            type_: Default::default(),
        }
    }
}
impl StructuredRegulatoryReporting3 {
    pub fn builder() -> builder::StructuredRegulatoryReporting3 {
        Default::default()
    }
}
///Information supplied to enable the matching/reconciliation of an entry with the items that the payment is intended to settle, such as commercial invoices in an accounts' receivable system, in a structured form.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information supplied to enable the matching/reconciliation of an entry with the items that the payment is intended to settle, such as commercial invoices in an accounts' receivable system, in a structured form.",
///  "type": "object",
///  "properties": {
///    "additional_remittance_information": {
///      "description": "Additional information, in free text form, to complement the structured remittance information.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Max140Text"
///      },
///      "maxItems": 3
///    },
///    "creditor_reference_information": {
///      "description": "Reference information provided by the creditor to allow the identification of the underlying documents.",
///      "$ref": "#/definitions/CreditorReferenceInformation2"
///    },
///    "garnishment_remittance": {
///      "description": "Provides remittance information about a payment for garnishment-related purposes.",
///      "$ref": "#/definitions/Garnishment3__1"
///    },
///    "invoicee": {
///      "description": "Identification of the party to whom an invoice is issued, when it is different from the debtor or ultimate debtor.",
///      "$ref": "#/definitions/PartyIdentification135__4"
///    },
///    "invoicer": {
///      "description": "Identification of the organisation issuing the invoice, when it is different from the creditor or ultimate creditor.",
///      "$ref": "#/definitions/PartyIdentification135__4"
///    },
///    "referred_document_amount": {
///      "description": "Provides details on the amounts of the referred document.",
///      "$ref": "#/definitions/RemittanceAmount2"
///    },
///    "referred_document_information": {
///      "description": "Provides the identification and the content of the referred document.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ReferredDocumentInformation7__1"
///      }
///    },
///    "tax_remittance": {
///      "description": "Provides remittance information about a payment made for tax-related purposes.",
///      "$ref": "#/definitions/TaxInformation7"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StructuredRemittanceInformation161 {
    ///Additional information, in free text form, to complement the structured remittance information.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub additional_remittance_information: ::std::vec::Vec<Max140Text>,
    ///Reference information provided by the creditor to allow the identification of the underlying documents.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub creditor_reference_information: ::std::option::Option<
        CreditorReferenceInformation2,
    >,
    ///Provides remittance information about a payment for garnishment-related purposes.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub garnishment_remittance: ::std::option::Option<Garnishment31>,
    ///Identification of the party to whom an invoice is issued, when it is different from the debtor or ultimate debtor.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub invoicee: ::std::option::Option<PartyIdentification1354>,
    ///Identification of the organisation issuing the invoice, when it is different from the creditor or ultimate creditor.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub invoicer: ::std::option::Option<PartyIdentification1354>,
    ///Provides details on the amounts of the referred document.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub referred_document_amount: ::std::option::Option<RemittanceAmount2>,
    ///Provides the identification and the content of the referred document.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub referred_document_information: ::std::vec::Vec<ReferredDocumentInformation71>,
    ///Provides remittance information about a payment made for tax-related purposes.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tax_remittance: ::std::option::Option<TaxInformation7>,
}
impl ::std::default::Default for StructuredRemittanceInformation161 {
    fn default() -> Self {
        Self {
            additional_remittance_information: Default::default(),
            creditor_reference_information: Default::default(),
            garnishment_remittance: Default::default(),
            invoicee: Default::default(),
            invoicer: Default::default(),
            referred_document_amount: Default::default(),
            referred_document_information: Default::default(),
            tax_remittance: Default::default(),
        }
    }
}
impl StructuredRemittanceInformation161 {
    pub fn builder() -> builder::StructuredRemittanceInformation161 {
        Default::default()
    }
}
///Set of elements used to provide information on the tax amount(s) of tax record.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Set of elements used to provide information on the tax amount(s) of tax record.",
///  "type": "object",
///  "properties": {
///    "details": {
///      "description": "Set of elements used to provide details on the tax period and amount.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/TaxRecordDetails2"
///      }
///    },
///    "rate": {
///      "description": "Rate used to calculate the tax.",
///      "$ref": "#/definitions/PercentageRate"
///    },
///    "taxable_base_amount": {
///      "description": "Amount of money on which the tax is based.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "total_amount": {
///      "description": "Total amount that is the result of the calculation of the tax for the record.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaxAmount2 {
    ///Set of elements used to provide details on the tax period and amount.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub details: ::std::vec::Vec<TaxRecordDetails2>,
    ///Rate used to calculate the tax.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rate: ::std::option::Option<PercentageRate>,
    ///Amount of money on which the tax is based.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub taxable_base_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Total amount that is the result of the calculation of the tax for the record.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub total_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
impl ::std::default::Default for TaxAmount2 {
    fn default() -> Self {
        Self {
            details: Default::default(),
            rate: Default::default(),
            taxable_base_amount: Default::default(),
            total_amount: Default::default(),
        }
    }
}
impl TaxAmount2 {
    pub fn builder() -> builder::TaxAmount2 {
        Default::default()
    }
}
///Specifies the amount with a specific type.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the amount with a specific type.",
///  "type": "object",
///  "required": [
///    "amount"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Amount of money, which has been typed.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "type": {
///      "description": "Specifies the type of the amount.",
///      "$ref": "#/definitions/TaxAmountType1Choice"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaxAmountAndType1 {
    ///Amount of money, which has been typed.
    pub amount: ActiveOrHistoricCurrencyAndAmount,
    ///Specifies the type of the amount.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<TaxAmountType1Choice>,
}
impl TaxAmountAndType1 {
    pub fn builder() -> builder::TaxAmountAndType1 {
        Default::default()
    }
}
///Specifies the amount type.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the amount type.",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "code"
///      ],
///      "properties": {
///        "code": {
///          "description": "Specifies the amount type, in a coded form.",
///          "$ref": "#/definitions/ExternalTaxAmountType1Code"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "proprietary"
///      ],
///      "properties": {
///        "proprietary": {
///          "description": "Specifies the amount type, in a free-text form.",
///          "$ref": "#/definitions/Max35Text"
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
pub enum TaxAmountType1Choice {
    #[serde(rename = "code")]
    Code(ExternalTaxAmountType1Code),
    #[serde(rename = "proprietary")]
    Proprietary(Max35Text),
}
impl ::std::convert::From<ExternalTaxAmountType1Code> for TaxAmountType1Choice {
    fn from(value: ExternalTaxAmountType1Code) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<Max35Text> for TaxAmountType1Choice {
    fn from(value: Max35Text) -> Self {
        Self::Proprietary(value)
    }
}
///Details of the authorised tax paying party.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Details of the authorised tax paying party.",
///  "type": "object",
///  "properties": {
///    "name": {
///      "description": "Name of the debtor or the debtor's authorised representative.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "title": {
///      "description": "Title or position of debtor or the debtor's authorised representative.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaxAuthorisation1 {
    ///Name of the debtor or the debtor's authorised representative.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Max140Text>,
    ///Title or position of debtor or the debtor's authorised representative.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<Max35Text>,
}
impl ::std::default::Default for TaxAuthorisation1 {
    fn default() -> Self {
        Self {
            name: Default::default(),
            title: Default::default(),
        }
    }
}
impl TaxAuthorisation1 {
    pub fn builder() -> builder::TaxAuthorisation1 {
        Default::default()
    }
}
///Details about tax paid, or to be paid, to the government in accordance with the law, including pre-defined parameters such as thresholds and type of account.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Details about tax paid, or to be paid, to the government in accordance with the law, including pre-defined parameters such as thresholds and type of account.",
///  "type": "object",
///  "properties": {
///    "administration_zone": {
///      "description": "Territorial part of a country to which the tax payment is related.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "creditor": {
///      "description": "Party on the credit side of the transaction to which the tax applies.",
///      "$ref": "#/definitions/TaxParty1"
///    },
///    "date": {
///      "description": "Date by which tax is due.",
///      "$ref": "#/definitions/ISODate"
///    },
///    "debtor": {
///      "description": "Identifies the party on the debit side of the transaction to which the tax applies.",
///      "$ref": "#/definitions/TaxParty2"
///    },
///    "method": {
///      "description": "Method used to indicate the underlying business or how the tax is paid.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "record": {
///      "description": "Record of tax details.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/TaxRecord2"
///      }
///    },
///    "reference_number": {
///      "description": "Tax reference information that is specific to a taxing agency.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "sequence_number": {
///      "description": "Sequential number of the tax report.",
///      "$ref": "#/definitions/Number"
///    },
///    "total_tax_amount": {
///      "description": "Total amount of money as result of the calculation of the tax.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "total_taxable_base_amount": {
///      "description": "Total amount of money on which the tax is based.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "ultimate_debtor": {
///      "description": "Ultimate party that owes an amount of money to the (ultimate) creditor, in this case, to the taxing authority.",
///      "$ref": "#/definitions/TaxParty2"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaxInformation7 {
    ///Territorial part of a country to which the tax payment is related.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub administration_zone: ::std::option::Option<Max35Text>,
    ///Party on the credit side of the transaction to which the tax applies.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub creditor: ::std::option::Option<TaxParty1>,
    ///Date by which tax is due.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub date: ::std::option::Option<IsoDate>,
    ///Identifies the party on the debit side of the transaction to which the tax applies.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub debtor: ::std::option::Option<TaxParty2>,
    ///Method used to indicate the underlying business or how the tax is paid.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub method: ::std::option::Option<Max35Text>,
    ///Record of tax details.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub record: ::std::vec::Vec<TaxRecord2>,
    ///Tax reference information that is specific to a taxing agency.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reference_number: ::std::option::Option<Max140Text>,
    ///Sequential number of the tax report.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sequence_number: ::std::option::Option<Number>,
    ///Total amount of money as result of the calculation of the tax.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub total_tax_amount: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ///Total amount of money on which the tax is based.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub total_taxable_base_amount: ::std::option::Option<
        ActiveOrHistoricCurrencyAndAmount,
    >,
    ///Ultimate party that owes an amount of money to the (ultimate) creditor, in this case, to the taxing authority.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ultimate_debtor: ::std::option::Option<TaxParty2>,
}
impl ::std::default::Default for TaxInformation7 {
    fn default() -> Self {
        Self {
            administration_zone: Default::default(),
            creditor: Default::default(),
            date: Default::default(),
            debtor: Default::default(),
            method: Default::default(),
            record: Default::default(),
            reference_number: Default::default(),
            sequence_number: Default::default(),
            total_tax_amount: Default::default(),
            total_taxable_base_amount: Default::default(),
            ultimate_debtor: Default::default(),
        }
    }
}
impl TaxInformation7 {
    pub fn builder() -> builder::TaxInformation7 {
        Default::default()
    }
}
///Details about the entity involved in the tax paid or to be paid.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Details about the entity involved in the tax paid or to be paid.",
///  "type": "object",
///  "properties": {
///    "registration_identification": {
///      "description": "Unique identification, as assigned by an organisation, to unambiguously identify a party.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "tax_identification": {
///      "description": "Tax identification number of the creditor.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "tax_type": {
///      "description": "Type of tax payer.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaxParty1 {
    ///Unique identification, as assigned by an organisation, to unambiguously identify a party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub registration_identification: ::std::option::Option<Max35Text>,
    ///Tax identification number of the creditor.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tax_identification: ::std::option::Option<Max35Text>,
    ///Type of tax payer.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tax_type: ::std::option::Option<Max35Text>,
}
impl ::std::default::Default for TaxParty1 {
    fn default() -> Self {
        Self {
            registration_identification: Default::default(),
            tax_identification: Default::default(),
            tax_type: Default::default(),
        }
    }
}
impl TaxParty1 {
    pub fn builder() -> builder::TaxParty1 {
        Default::default()
    }
}
///Details about the entity involved in the tax paid or to be paid.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Details about the entity involved in the tax paid or to be paid.",
///  "type": "object",
///  "properties": {
///    "authorisation": {
///      "description": "Details of the authorised tax paying party.",
///      "$ref": "#/definitions/TaxAuthorisation1"
///    },
///    "registration_identification": {
///      "description": "Unique identification, as assigned by an organisation, to unambiguously identify a party.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "tax_identification": {
///      "description": "Tax identification number of the debtor.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "tax_type": {
///      "description": "Type of tax payer.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaxParty2 {
    ///Details of the authorised tax paying party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub authorisation: ::std::option::Option<TaxAuthorisation1>,
    ///Unique identification, as assigned by an organisation, to unambiguously identify a party.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub registration_identification: ::std::option::Option<Max35Text>,
    ///Tax identification number of the debtor.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tax_identification: ::std::option::Option<Max35Text>,
    ///Type of tax payer.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tax_type: ::std::option::Option<Max35Text>,
}
impl ::std::default::Default for TaxParty2 {
    fn default() -> Self {
        Self {
            authorisation: Default::default(),
            registration_identification: Default::default(),
            tax_identification: Default::default(),
            tax_type: Default::default(),
        }
    }
}
impl TaxParty2 {
    pub fn builder() -> builder::TaxParty2 {
        Default::default()
    }
}
///Period of time details related to the tax payment.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Period of time details related to the tax payment.",
///  "type": "object",
///  "properties": {
///    "from_to_date": {
///      "description": "Range of time between a start date and an end date for which the tax report is provided.",
///      "$ref": "#/definitions/DatePeriod2"
///    },
///    "type": {
///      "description": "Identification of the period related to the tax payment.",
///      "$ref": "#/definitions/TaxRecordPeriod1Code"
///    },
///    "year": {
///      "description": "Year related to the tax payment.",
///      "$ref": "#/definitions/ISODate"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaxPeriod2 {
    ///Range of time between a start date and an end date for which the tax report is provided.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub from_to_date: ::std::option::Option<DatePeriod2>,
    ///Identification of the period related to the tax payment.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<TaxRecordPeriod1Code>,
    ///Year related to the tax payment.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub year: ::std::option::Option<IsoDate>,
}
impl ::std::default::Default for TaxPeriod2 {
    fn default() -> Self {
        Self {
            from_to_date: Default::default(),
            type_: Default::default(),
            year: Default::default(),
        }
    }
}
impl TaxPeriod2 {
    pub fn builder() -> builder::TaxPeriod2 {
        Default::default()
    }
}
///Set of elements used to define the tax record.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Set of elements used to define the tax record.",
///  "type": "object",
///  "properties": {
///    "additional_information": {
///      "description": "Further details of the tax record.",
///      "$ref": "#/definitions/Max140Text"
///    },
///    "category": {
///      "description": "Specifies the tax code as published by the tax authority.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "category_details": {
///      "description": "Provides further details of the category tax code.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "certificate_identification": {
///      "description": "Identification number of the tax report as assigned by the taxing authority.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "debtor_status": {
///      "description": "Code provided by local authority to identify the status of the party that has drawn up the settlement document.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "forms_code": {
///      "description": "Identifies, in a coded form, on which template the tax report is to be provided.",
///      "$ref": "#/definitions/Max35Text"
///    },
///    "period": {
///      "description": "Set of elements used to provide details on the period of time related to the tax payment.",
///      "$ref": "#/definitions/TaxPeriod2"
///    },
///    "tax_amount": {
///      "description": "Set of elements used to provide information on the amount of the tax record.",
///      "$ref": "#/definitions/TaxAmount2"
///    },
///    "type": {
///      "description": "High level code to identify the type of tax details.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaxRecord2 {
    ///Further details of the tax record.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub additional_information: ::std::option::Option<Max140Text>,
    ///Specifies the tax code as published by the tax authority.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub category: ::std::option::Option<Max35Text>,
    ///Provides further details of the category tax code.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub category_details: ::std::option::Option<Max35Text>,
    ///Identification number of the tax report as assigned by the taxing authority.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate_identification: ::std::option::Option<Max35Text>,
    ///Code provided by local authority to identify the status of the party that has drawn up the settlement document.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub debtor_status: ::std::option::Option<Max35Text>,
    ///Identifies, in a coded form, on which template the tax report is to be provided.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub forms_code: ::std::option::Option<Max35Text>,
    ///Set of elements used to provide details on the period of time related to the tax payment.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<TaxPeriod2>,
    ///Set of elements used to provide information on the amount of the tax record.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tax_amount: ::std::option::Option<TaxAmount2>,
    ///High level code to identify the type of tax details.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<Max35Text>,
}
impl ::std::default::Default for TaxRecord2 {
    fn default() -> Self {
        Self {
            additional_information: Default::default(),
            category: Default::default(),
            category_details: Default::default(),
            certificate_identification: Default::default(),
            debtor_status: Default::default(),
            forms_code: Default::default(),
            period: Default::default(),
            tax_amount: Default::default(),
            type_: Default::default(),
        }
    }
}
impl TaxRecord2 {
    pub fn builder() -> builder::TaxRecord2 {
        Default::default()
    }
}
///Provides information on the individual tax amount(s) per period of the tax record.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides information on the individual tax amount(s) per period of the tax record.",
///  "type": "object",
///  "required": [
///    "amount"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Underlying tax amount related to the specified period.",
///      "$ref": "#/definitions/ActiveOrHistoricCurrencyAndAmount"
///    },
///    "period": {
///      "description": "Set of elements used to provide details on the period of time related to the tax payment.",
///      "$ref": "#/definitions/TaxPeriod2"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaxRecordDetails2 {
    ///Underlying tax amount related to the specified period.
    pub amount: ActiveOrHistoricCurrencyAndAmount,
    ///Set of elements used to provide details on the period of time related to the tax payment.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<TaxPeriod2>,
}
impl TaxRecordDetails2 {
    pub fn builder() -> builder::TaxRecordDetails2 {
        Default::default()
    }
}
/**Specifies the period related to the tax payment.
*`MM01`-Tax is related to the second month of the period.
*`MM02`-Tax is related to the first month of the period.
*`MM03`-Tax is related to the third month of the period.
*`MM04`-Tax is related to the fourth month of the period.
*`MM05`-Tax is related to the fifth month of the period.
*`MM06`-Tax is related to the sixth month of the period.
*`MM07`-Tax is related to the seventh month of the period.
*`MM08`-Tax is related to the eighth month of the period.
*`MM09`-Tax is related to the ninth month of the period.
*`MM10`-Tax is related to the tenth month of the period.
*`MM11`-Tax is related to the eleventh month of the period.
*`MM12`-Tax is related to the twelfth month of the period.
*`QTR1`-Tax is related to the first quarter of the period.
*`QTR2`-Tax is related to the second quarter of the period.
*`QTR3`-Tax is related to the third quarter of the period.
*`QTR4`-Tax is related to the forth quarter of the period.
*`HLF1`-Tax is related to the first half of the period.
*`HLF2`-Tax is related to the second half of the period.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the period related to the tax payment.\n*`MM01`-Tax is related to the second month of the period.\n*`MM02`-Tax is related to the first month of the period.\n*`MM03`-Tax is related to the third month of the period.\n*`MM04`-Tax is related to the fourth month of the period.\n*`MM05`-Tax is related to the fifth month of the period.\n*`MM06`-Tax is related to the sixth month of the period.\n*`MM07`-Tax is related to the seventh month of the period.\n*`MM08`-Tax is related to the eighth month of the period.\n*`MM09`-Tax is related to the ninth month of the period.\n*`MM10`-Tax is related to the tenth month of the period.\n*`MM11`-Tax is related to the eleventh month of the period.\n*`MM12`-Tax is related to the twelfth month of the period.\n*`QTR1`-Tax is related to the first quarter of the period.\n*`QTR2`-Tax is related to the second quarter of the period.\n*`QTR3`-Tax is related to the third quarter of the period.\n*`QTR4`-Tax is related to the forth quarter of the period.\n*`HLF1`-Tax is related to the first half of the period.\n*`HLF2`-Tax is related to the second half of the period.",
///  "type": "string",
///  "enum": [
///    "MM01",
///    "MM02",
///    "MM03",
///    "MM04",
///    "MM05",
///    "MM06",
///    "MM07",
///    "MM08",
///    "MM09",
///    "MM10",
///    "MM11",
///    "MM12",
///    "QTR1",
///    "QTR2",
///    "QTR3",
///    "QTR4",
///    "HLF1",
///    "HLF2"
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
pub enum TaxRecordPeriod1Code {
    #[serde(rename = "MM01")]
    Mm01,
    #[serde(rename = "MM02")]
    Mm02,
    #[serde(rename = "MM03")]
    Mm03,
    #[serde(rename = "MM04")]
    Mm04,
    #[serde(rename = "MM05")]
    Mm05,
    #[serde(rename = "MM06")]
    Mm06,
    #[serde(rename = "MM07")]
    Mm07,
    #[serde(rename = "MM08")]
    Mm08,
    #[serde(rename = "MM09")]
    Mm09,
    #[serde(rename = "MM10")]
    Mm10,
    #[serde(rename = "MM11")]
    Mm11,
    #[serde(rename = "MM12")]
    Mm12,
    #[serde(rename = "QTR1")]
    Qtr1,
    #[serde(rename = "QTR2")]
    Qtr2,
    #[serde(rename = "QTR3")]
    Qtr3,
    #[serde(rename = "QTR4")]
    Qtr4,
    #[serde(rename = "HLF1")]
    Hlf1,
    #[serde(rename = "HLF2")]
    Hlf2,
}
impl ::std::fmt::Display for TaxRecordPeriod1Code {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mm01 => f.write_str("MM01"),
            Self::Mm02 => f.write_str("MM02"),
            Self::Mm03 => f.write_str("MM03"),
            Self::Mm04 => f.write_str("MM04"),
            Self::Mm05 => f.write_str("MM05"),
            Self::Mm06 => f.write_str("MM06"),
            Self::Mm07 => f.write_str("MM07"),
            Self::Mm08 => f.write_str("MM08"),
            Self::Mm09 => f.write_str("MM09"),
            Self::Mm10 => f.write_str("MM10"),
            Self::Mm11 => f.write_str("MM11"),
            Self::Mm12 => f.write_str("MM12"),
            Self::Qtr1 => f.write_str("QTR1"),
            Self::Qtr2 => f.write_str("QTR2"),
            Self::Qtr3 => f.write_str("QTR3"),
            Self::Qtr4 => f.write_str("QTR4"),
            Self::Hlf1 => f.write_str("HLF1"),
            Self::Hlf2 => f.write_str("HLF2"),
        }
    }
}
impl ::std::str::FromStr for TaxRecordPeriod1Code {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MM01" => Ok(Self::Mm01),
            "MM02" => Ok(Self::Mm02),
            "MM03" => Ok(Self::Mm03),
            "MM04" => Ok(Self::Mm04),
            "MM05" => Ok(Self::Mm05),
            "MM06" => Ok(Self::Mm06),
            "MM07" => Ok(Self::Mm07),
            "MM08" => Ok(Self::Mm08),
            "MM09" => Ok(Self::Mm09),
            "MM10" => Ok(Self::Mm10),
            "MM11" => Ok(Self::Mm11),
            "MM12" => Ok(Self::Mm12),
            "QTR1" => Ok(Self::Qtr1),
            "QTR2" => Ok(Self::Qtr2),
            "QTR3" => Ok(Self::Qtr3),
            "QTR4" => Ok(Self::Qtr4),
            "HLF1" => Ok(Self::Hlf1),
            "HLF2" => Ok(Self::Hlf2),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TaxRecordPeriod1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TaxRecordPeriod1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TaxRecordPeriod1Code {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A flag indicating a True or False value.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A flag indicating a True or False value.",
///  "type": "boolean"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TrueFalseIndicator(pub bool);
impl ::std::ops::Deref for TrueFalseIndicator {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<TrueFalseIndicator> for bool {
    fn from(value: TrueFalseIndicator) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for TrueFalseIndicator {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for TrueFalseIndicator {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for TrueFalseIndicator {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for TrueFalseIndicator {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for TrueFalseIndicator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
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
    pub struct AccountIdentification4Choice1 {
        other: ::std::result::Result<
            super::GenericAccountIdentification1,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AccountIdentification4Choice1 {
        fn default() -> Self {
            Self {
                other: Err("no value supplied for other".to_string()),
            }
        }
    }
    impl AccountIdentification4Choice1 {
        pub fn other<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GenericAccountIdentification1>,
            T::Error: ::std::fmt::Display,
        {
            self.other = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for other: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AccountIdentification4Choice1>
    for super::AccountIdentification4Choice1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AccountIdentification4Choice1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { other: value.other? })
        }
    }
    impl ::std::convert::From<super::AccountIdentification4Choice1>
    for AccountIdentification4Choice1 {
        fn from(value: super::AccountIdentification4Choice1) -> Self {
            Self { other: Ok(value.other) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActiveCurrencyAndAmount2decimalsCopy {
        amount: ::std::result::Result<
            super::ActiveCurrencyAndAmount2decimalsCopyAmount,
            ::std::string::String,
        >,
        currency: ::std::result::Result<
            super::ActiveOrHistoricCurrencyCodeFixed,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ActiveCurrencyAndAmount2decimalsCopy {
        fn default() -> Self {
            Self {
                amount: Err("no value supplied for amount".to_string()),
                currency: Err("no value supplied for currency".to_string()),
            }
        }
    }
    impl ActiveCurrencyAndAmount2decimalsCopy {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::ActiveCurrencyAndAmount2decimalsCopyAmount,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActiveOrHistoricCurrencyCodeFixed>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for currency: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ActiveCurrencyAndAmount2decimalsCopy>
    for super::ActiveCurrencyAndAmount2decimalsCopy {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActiveCurrencyAndAmount2decimalsCopy,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                currency: value.currency?,
            })
        }
    }
    impl ::std::convert::From<super::ActiveCurrencyAndAmount2decimalsCopy>
    for ActiveCurrencyAndAmount2decimalsCopy {
        fn from(value: super::ActiveCurrencyAndAmount2decimalsCopy) -> Self {
            Self {
                amount: Ok(value.amount),
                currency: Ok(value.currency),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActiveOrHistoricCurrencyAndAmount {
        amount: ::std::result::Result<
            super::ActiveOrHistoricCurrencyAndAmountAmount,
            ::std::string::String,
        >,
        currency: ::std::result::Result<
            super::ActiveOrHistoricCurrencyCode,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ActiveOrHistoricCurrencyAndAmount {
        fn default() -> Self {
            Self {
                amount: Err("no value supplied for amount".to_string()),
                currency: Err("no value supplied for currency".to_string()),
            }
        }
    }
    impl ActiveOrHistoricCurrencyAndAmount {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActiveOrHistoricCurrencyAndAmountAmount>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActiveOrHistoricCurrencyCode>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for currency: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ActiveOrHistoricCurrencyAndAmount>
    for super::ActiveOrHistoricCurrencyAndAmount {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActiveOrHistoricCurrencyAndAmount,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                currency: value.currency?,
            })
        }
    }
    impl ::std::convert::From<super::ActiveOrHistoricCurrencyAndAmount>
    for ActiveOrHistoricCurrencyAndAmount {
        fn from(value: super::ActiveOrHistoricCurrencyAndAmount) -> Self {
            Self {
                amount: Ok(value.amount),
                currency: Ok(value.currency),
            }
        }
    }
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
    pub struct BranchAndFinancialInstitutionIdentification63 {
        financial_institution_identification: ::std::result::Result<
            super::FinancialInstitutionIdentification183,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BranchAndFinancialInstitutionIdentification63 {
        fn default() -> Self {
            Self {
                financial_institution_identification: Err(
                    "no value supplied for financial_institution_identification"
                        .to_string(),
                ),
            }
        }
    }
    impl BranchAndFinancialInstitutionIdentification63 {
        pub fn financial_institution_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FinancialInstitutionIdentification183>,
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
    impl ::std::convert::TryFrom<BranchAndFinancialInstitutionIdentification63>
    for super::BranchAndFinancialInstitutionIdentification63 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BranchAndFinancialInstitutionIdentification63,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                financial_institution_identification: value
                    .financial_institution_identification?,
            })
        }
    }
    impl ::std::convert::From<super::BranchAndFinancialInstitutionIdentification63>
    for BranchAndFinancialInstitutionIdentification63 {
        fn from(value: super::BranchAndFinancialInstitutionIdentification63) -> Self {
            Self {
                financial_institution_identification: Ok(
                    value.financial_institution_identification,
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CashAccount38 {
        currency: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyCode>,
            ::std::string::String,
        >,
        identification: ::std::result::Result<
            super::AccountIdentification4Choice,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        proxy: ::std::result::Result<
            ::std::option::Option<super::ProxyAccountIdentification1>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::CashAccountType2Choice>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CashAccount38 {
        fn default() -> Self {
            Self {
                currency: Ok(Default::default()),
                identification: Err("no value supplied for identification".to_string()),
                name: Ok(Default::default()),
                proxy: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl CashAccount38 {
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyCode>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for currency: {e}")
                });
            self
        }
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AccountIdentification4Choice>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn proxy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ProxyAccountIdentification1>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.proxy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for proxy: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CashAccountType2Choice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CashAccount38> for super::CashAccount38 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CashAccount38,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                currency: value.currency?,
                identification: value.identification?,
                name: value.name?,
                proxy: value.proxy?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CashAccount38> for CashAccount38 {
        fn from(value: super::CashAccount38) -> Self {
            Self {
                currency: Ok(value.currency),
                identification: Ok(value.identification),
                name: Ok(value.name),
                proxy: Ok(value.proxy),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CashAccount381 {
        identification: ::std::result::Result<
            super::AccountIdentification4Choice1,
            ::std::string::String,
        >,
        proxy: ::std::result::Result<
            ::std::option::Option<super::ProxyAccountIdentification1>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CashAccount381 {
        fn default() -> Self {
            Self {
                identification: Err("no value supplied for identification".to_string()),
                proxy: Ok(Default::default()),
            }
        }
    }
    impl CashAccount381 {
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AccountIdentification4Choice1>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
        pub fn proxy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ProxyAccountIdentification1>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.proxy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for proxy: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CashAccount381> for super::CashAccount381 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CashAccount381,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                identification: value.identification?,
                proxy: value.proxy?,
            })
        }
    }
    impl ::std::convert::From<super::CashAccount381> for CashAccount381 {
        fn from(value: super::CashAccount381) -> Self {
            Self {
                identification: Ok(value.identification),
                proxy: Ok(value.proxy),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Charges71 {
        agent: ::std::result::Result<
            super::BranchAndFinancialInstitutionIdentification61,
            ::std::string::String,
        >,
        amount: ::std::result::Result<
            super::ActiveOrHistoricCurrencyAndAmount,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Charges71 {
        fn default() -> Self {
            Self {
                agent: Err("no value supplied for agent".to_string()),
                amount: Err("no value supplied for amount".to_string()),
            }
        }
    }
    impl Charges71 {
        pub fn agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::BranchAndFinancialInstitutionIdentification61,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.agent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent: {e}"));
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActiveOrHistoricCurrencyAndAmount>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Charges71> for super::Charges71 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Charges71,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent: value.agent?,
                amount: value.amount?,
            })
        }
    }
    impl ::std::convert::From<super::Charges71> for Charges71 {
        fn from(value: super::Charges71) -> Self {
            Self {
                agent: Ok(value.agent),
                amount: Ok(value.amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClearingSystemIdentification2Choice1 {
        code: ::std::result::Result<
            super::ExternalClearingSystemIdentification1Code,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ClearingSystemIdentification2Choice1 {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
            }
        }
    }
    impl ClearingSystemIdentification2Choice1 {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExternalClearingSystemIdentification1Code>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClearingSystemIdentification2Choice1>
    for super::ClearingSystemIdentification2Choice1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClearingSystemIdentification2Choice1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { code: value.code? })
        }
    }
    impl ::std::convert::From<super::ClearingSystemIdentification2Choice1>
    for ClearingSystemIdentification2Choice1 {
        fn from(value: super::ClearingSystemIdentification2Choice1) -> Self {
            Self { code: Ok(value.code) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClearingSystemIdentification3Choice1 {
        code: ::std::result::Result<
            super::ExternalCashClearingSystem1CodeFixed,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ClearingSystemIdentification3Choice1 {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
            }
        }
    }
    impl ClearingSystemIdentification3Choice1 {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExternalCashClearingSystem1CodeFixed>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClearingSystemIdentification3Choice1>
    for super::ClearingSystemIdentification3Choice1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClearingSystemIdentification3Choice1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { code: value.code? })
        }
    }
    impl ::std::convert::From<super::ClearingSystemIdentification3Choice1>
    for ClearingSystemIdentification3Choice1 {
        fn from(value: super::ClearingSystemIdentification3Choice1) -> Self {
            Self { code: Ok(value.code) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClearingSystemMemberIdentification2 {
        clearing_system_identification: ::std::result::Result<
            ::std::option::Option<super::ClearingSystemIdentification2Choice>,
            ::std::string::String,
        >,
        member_identification: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ClearingSystemMemberIdentification2 {
        fn default() -> Self {
            Self {
                clearing_system_identification: Ok(Default::default()),
                member_identification: Err(
                    "no value supplied for member_identification".to_string(),
                ),
            }
        }
    }
    impl ClearingSystemMemberIdentification2 {
        pub fn clearing_system_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ClearingSystemIdentification2Choice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.clearing_system_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for clearing_system_identification: {e}"
                    )
                });
            self
        }
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
    impl ::std::convert::TryFrom<ClearingSystemMemberIdentification2>
    for super::ClearingSystemMemberIdentification2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClearingSystemMemberIdentification2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                clearing_system_identification: value.clearing_system_identification?,
                member_identification: value.member_identification?,
            })
        }
    }
    impl ::std::convert::From<super::ClearingSystemMemberIdentification2>
    for ClearingSystemMemberIdentification2 {
        fn from(value: super::ClearingSystemMemberIdentification2) -> Self {
            Self {
                clearing_system_identification: Ok(value.clearing_system_identification),
                member_identification: Ok(value.member_identification),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClearingSystemMemberIdentification21 {
        clearing_system_identification: ::std::result::Result<
            ::std::option::Option<super::ClearingSystemIdentification2Choice1>,
            ::std::string::String,
        >,
        member_identification: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ClearingSystemMemberIdentification21 {
        fn default() -> Self {
            Self {
                clearing_system_identification: Ok(Default::default()),
                member_identification: Err(
                    "no value supplied for member_identification".to_string(),
                ),
            }
        }
    }
    impl ClearingSystemMemberIdentification21 {
        pub fn clearing_system_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ClearingSystemIdentification2Choice1>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.clearing_system_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for clearing_system_identification: {e}"
                    )
                });
            self
        }
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
                clearing_system_identification: value.clearing_system_identification?,
                member_identification: value.member_identification?,
            })
        }
    }
    impl ::std::convert::From<super::ClearingSystemMemberIdentification21>
    for ClearingSystemMemberIdentification21 {
        fn from(value: super::ClearingSystemMemberIdentification21) -> Self {
            Self {
                clearing_system_identification: Ok(value.clearing_system_identification),
                member_identification: Ok(value.member_identification),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClearingSystemMemberIdentification22 {
        member_identification: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ClearingSystemMemberIdentification22 {
        fn default() -> Self {
            Self {
                member_identification: Err(
                    "no value supplied for member_identification".to_string(),
                ),
            }
        }
    }
    impl ClearingSystemMemberIdentification22 {
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
    impl ::std::convert::TryFrom<ClearingSystemMemberIdentification22>
    for super::ClearingSystemMemberIdentification22 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClearingSystemMemberIdentification22,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                member_identification: value.member_identification?,
            })
        }
    }
    impl ::std::convert::From<super::ClearingSystemMemberIdentification22>
    for ClearingSystemMemberIdentification22 {
        fn from(value: super::ClearingSystemMemberIdentification22) -> Self {
            Self {
                member_identification: Ok(value.member_identification),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Contact41 {
        email_address: ::std::result::Result<
            ::std::option::Option<super::Max2048Text>,
            ::std::string::String,
        >,
        fax_number: ::std::result::Result<
            ::std::option::Option<super::PhoneNumber>,
            ::std::string::String,
        >,
        mobile_number: ::std::result::Result<
            ::std::option::Option<super::PhoneNumber>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        phone_number: ::std::result::Result<
            ::std::option::Option<super::PhoneNumber>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Contact41 {
        fn default() -> Self {
            Self {
                email_address: Ok(Default::default()),
                fax_number: Ok(Default::default()),
                mobile_number: Ok(Default::default()),
                name: Ok(Default::default()),
                phone_number: Ok(Default::default()),
            }
        }
    }
    impl Contact41 {
        pub fn email_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max2048Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.email_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for email_address: {e}")
                });
            self
        }
        pub fn fax_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PhoneNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.fax_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fax_number: {e}")
                });
            self
        }
        pub fn mobile_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PhoneNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.mobile_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for mobile_number: {e}")
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn phone_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PhoneNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.phone_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for phone_number: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Contact41> for super::Contact41 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Contact41,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email_address: value.email_address?,
                fax_number: value.fax_number?,
                mobile_number: value.mobile_number?,
                name: value.name?,
                phone_number: value.phone_number?,
            })
        }
    }
    impl ::std::convert::From<super::Contact41> for Contact41 {
        fn from(value: super::Contact41) -> Self {
            Self {
                email_address: Ok(value.email_address),
                fax_number: Ok(value.fax_number),
                mobile_number: Ok(value.mobile_number),
                name: Ok(value.name),
                phone_number: Ok(value.phone_number),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Contact42 {
        email_address: ::std::result::Result<
            ::std::option::Option<super::Max2048Text>,
            ::std::string::String,
        >,
        mobile_number: ::std::result::Result<
            ::std::option::Option<super::PhoneNumber>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Contact42 {
        fn default() -> Self {
            Self {
                email_address: Ok(Default::default()),
                mobile_number: Ok(Default::default()),
            }
        }
    }
    impl Contact42 {
        pub fn email_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max2048Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.email_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for email_address: {e}")
                });
            self
        }
        pub fn mobile_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PhoneNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.mobile_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for mobile_number: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Contact42> for super::Contact42 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Contact42,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email_address: value.email_address?,
                mobile_number: value.mobile_number?,
            })
        }
    }
    impl ::std::convert::From<super::Contact42> for Contact42 {
        fn from(value: super::Contact42) -> Self {
            Self {
                email_address: Ok(value.email_address),
                mobile_number: Ok(value.mobile_number),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Contact43 {
        department: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        email_address: ::std::result::Result<
            ::std::option::Option<super::Max2048Text>,
            ::std::string::String,
        >,
        fax_number: ::std::result::Result<
            ::std::option::Option<super::PhoneNumber>,
            ::std::string::String,
        >,
        mobile_number: ::std::result::Result<
            ::std::option::Option<super::PhoneNumber>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        phone_number: ::std::result::Result<
            ::std::option::Option<super::PhoneNumber>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Contact43 {
        fn default() -> Self {
            Self {
                department: Ok(Default::default()),
                email_address: Ok(Default::default()),
                fax_number: Ok(Default::default()),
                mobile_number: Ok(Default::default()),
                name: Ok(Default::default()),
                phone_number: Ok(Default::default()),
            }
        }
    }
    impl Contact43 {
        pub fn department<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.department = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for department: {e}")
                });
            self
        }
        pub fn email_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max2048Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.email_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for email_address: {e}")
                });
            self
        }
        pub fn fax_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PhoneNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.fax_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fax_number: {e}")
                });
            self
        }
        pub fn mobile_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PhoneNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.mobile_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for mobile_number: {e}")
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn phone_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PhoneNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.phone_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for phone_number: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Contact43> for super::Contact43 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Contact43,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                department: value.department?,
                email_address: value.email_address?,
                fax_number: value.fax_number?,
                mobile_number: value.mobile_number?,
                name: value.name?,
                phone_number: value.phone_number?,
            })
        }
    }
    impl ::std::convert::From<super::Contact43> for Contact43 {
        fn from(value: super::Contact43) -> Self {
            Self {
                department: Ok(value.department),
                email_address: Ok(value.email_address),
                fax_number: Ok(value.fax_number),
                mobile_number: Ok(value.mobile_number),
                name: Ok(value.name),
                phone_number: Ok(value.phone_number),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreditTransferTransaction391 {
        acceptance_date_time: ::std::result::Result<
            ::std::option::Option<super::IsoNormalisedDateTime>,
            ::std::string::String,
        >,
        charge_bearer: ::std::result::Result<
            super::ChargeBearerType1Code1,
            ::std::string::String,
        >,
        charges_information: ::std::result::Result<
            ::std::vec::Vec<super::Charges71>,
            ::std::string::String,
        >,
        creditor: ::std::result::Result<
            super::PartyIdentification1353,
            ::std::string::String,
        >,
        creditor_account: ::std::result::Result<
            super::CashAccount381,
            ::std::string::String,
        >,
        creditor_agent: ::std::result::Result<
            super::BranchAndFinancialInstitutionIdentification61,
            ::std::string::String,
        >,
        creditor_agent_account: ::std::result::Result<
            ::std::option::Option<super::CashAccount38>,
            ::std::string::String,
        >,
        debtor: ::std::result::Result<
            super::PartyIdentification1353,
            ::std::string::String,
        >,
        debtor_account: ::std::result::Result<
            super::CashAccount38,
            ::std::string::String,
        >,
        debtor_agent: ::std::result::Result<
            super::BranchAndFinancialInstitutionIdentification61,
            ::std::string::String,
        >,
        debtor_agent_account: ::std::result::Result<
            ::std::option::Option<super::CashAccount38>,
            ::std::string::String,
        >,
        exchange_rate: ::std::result::Result<
            ::std::option::Option<super::BaseOneRate>,
            ::std::string::String,
        >,
        initiating_party: ::std::result::Result<
            ::std::option::Option<super::PartyIdentification1352>,
            ::std::string::String,
        >,
        instructed_agent: ::std::result::Result<
            super::BranchAndFinancialInstitutionIdentification63,
            ::std::string::String,
        >,
        instructed_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        instructing_agent: ::std::result::Result<
            super::BranchAndFinancialInstitutionIdentification63,
            ::std::string::String,
        >,
        instruction_for_creditor_agent: ::std::result::Result<
            ::std::vec::Vec<super::InstructionForCreditorAgent1>,
            ::std::string::String,
        >,
        interbank_settlement_amount: ::std::result::Result<
            super::ActiveCurrencyAndAmount2decimalsCopy,
            ::std::string::String,
        >,
        interbank_settlement_date: ::std::result::Result<
            super::IsoDate,
            ::std::string::String,
        >,
        payment_identification: ::std::result::Result<
            super::PaymentIdentification71,
            ::std::string::String,
        >,
        payment_type_information: ::std::result::Result<
            super::PaymentTypeInformation281,
            ::std::string::String,
        >,
        previous_instructing_agent1: ::std::result::Result<
            ::std::option::Option<super::BranchAndFinancialInstitutionIdentification62>,
            ::std::string::String,
        >,
        previous_instructing_agent1_account: ::std::result::Result<
            ::std::option::Option<super::CashAccount38>,
            ::std::string::String,
        >,
        previous_instructing_agent2: ::std::result::Result<
            ::std::option::Option<super::BranchAndFinancialInstitutionIdentification62>,
            ::std::string::String,
        >,
        previous_instructing_agent2_account: ::std::result::Result<
            ::std::option::Option<super::CashAccount38>,
            ::std::string::String,
        >,
        previous_instructing_agent3: ::std::result::Result<
            ::std::option::Option<super::BranchAndFinancialInstitutionIdentification62>,
            ::std::string::String,
        >,
        previous_instructing_agent3_account: ::std::result::Result<
            ::std::option::Option<super::CashAccount38>,
            ::std::string::String,
        >,
        purpose: ::std::result::Result<
            ::std::option::Option<super::Purpose2Choice>,
            ::std::string::String,
        >,
        regulatory_reporting: ::std::result::Result<
            ::std::vec::Vec<super::RegulatoryReporting3>,
            ::std::string::String,
        >,
        related_remittance_information: ::std::result::Result<
            ::std::option::Option<super::RemittanceLocation71>,
            ::std::string::String,
        >,
        remittance_information: ::std::result::Result<
            ::std::option::Option<super::RemittanceInformation161>,
            ::std::string::String,
        >,
        ultimate_creditor: ::std::result::Result<
            ::std::option::Option<super::PartyIdentification1351>,
            ::std::string::String,
        >,
        ultimate_debtor: ::std::result::Result<
            ::std::option::Option<super::PartyIdentification1351>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreditTransferTransaction391 {
        fn default() -> Self {
            Self {
                acceptance_date_time: Ok(Default::default()),
                charge_bearer: Err("no value supplied for charge_bearer".to_string()),
                charges_information: Ok(Default::default()),
                creditor: Err("no value supplied for creditor".to_string()),
                creditor_account: Err(
                    "no value supplied for creditor_account".to_string(),
                ),
                creditor_agent: Err("no value supplied for creditor_agent".to_string()),
                creditor_agent_account: Ok(Default::default()),
                debtor: Err("no value supplied for debtor".to_string()),
                debtor_account: Err("no value supplied for debtor_account".to_string()),
                debtor_agent: Err("no value supplied for debtor_agent".to_string()),
                debtor_agent_account: Ok(Default::default()),
                exchange_rate: Ok(Default::default()),
                initiating_party: Ok(Default::default()),
                instructed_agent: Err(
                    "no value supplied for instructed_agent".to_string(),
                ),
                instructed_amount: Ok(Default::default()),
                instructing_agent: Err(
                    "no value supplied for instructing_agent".to_string(),
                ),
                instruction_for_creditor_agent: Ok(Default::default()),
                interbank_settlement_amount: Err(
                    "no value supplied for interbank_settlement_amount".to_string(),
                ),
                interbank_settlement_date: Err(
                    "no value supplied for interbank_settlement_date".to_string(),
                ),
                payment_identification: Err(
                    "no value supplied for payment_identification".to_string(),
                ),
                payment_type_information: Err(
                    "no value supplied for payment_type_information".to_string(),
                ),
                previous_instructing_agent1: Ok(Default::default()),
                previous_instructing_agent1_account: Ok(Default::default()),
                previous_instructing_agent2: Ok(Default::default()),
                previous_instructing_agent2_account: Ok(Default::default()),
                previous_instructing_agent3: Ok(Default::default()),
                previous_instructing_agent3_account: Ok(Default::default()),
                purpose: Ok(Default::default()),
                regulatory_reporting: Ok(Default::default()),
                related_remittance_information: Ok(Default::default()),
                remittance_information: Ok(Default::default()),
                ultimate_creditor: Ok(Default::default()),
                ultimate_debtor: Ok(Default::default()),
            }
        }
    }
    impl CreditTransferTransaction391 {
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
        pub fn charge_bearer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChargeBearerType1Code1>,
            T::Error: ::std::fmt::Display,
        {
            self.charge_bearer = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for charge_bearer: {e}")
                });
            self
        }
        pub fn charges_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Charges71>>,
            T::Error: ::std::fmt::Display,
        {
            self.charges_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for charges_information: {e}"
                    )
                });
            self
        }
        pub fn creditor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PartyIdentification1353>,
            T::Error: ::std::fmt::Display,
        {
            self.creditor = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for creditor: {e}")
                });
            self
        }
        pub fn creditor_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CashAccount381>,
            T::Error: ::std::fmt::Display,
        {
            self.creditor_account = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for creditor_account: {e}")
                });
            self
        }
        pub fn creditor_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::BranchAndFinancialInstitutionIdentification61,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.creditor_agent = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for creditor_agent: {e}")
                });
            self
        }
        pub fn creditor_agent_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CashAccount38>>,
            T::Error: ::std::fmt::Display,
        {
            self.creditor_agent_account = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for creditor_agent_account: {e}"
                    )
                });
            self
        }
        pub fn debtor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PartyIdentification1353>,
            T::Error: ::std::fmt::Display,
        {
            self.debtor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for debtor: {e}"));
            self
        }
        pub fn debtor_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CashAccount38>,
            T::Error: ::std::fmt::Display,
        {
            self.debtor_account = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for debtor_account: {e}")
                });
            self
        }
        pub fn debtor_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::BranchAndFinancialInstitutionIdentification61,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.debtor_agent = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for debtor_agent: {e}")
                });
            self
        }
        pub fn debtor_agent_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CashAccount38>>,
            T::Error: ::std::fmt::Display,
        {
            self.debtor_agent_account = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for debtor_agent_account: {e}"
                    )
                });
            self
        }
        pub fn exchange_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BaseOneRate>>,
            T::Error: ::std::fmt::Display,
        {
            self.exchange_rate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for exchange_rate: {e}")
                });
            self
        }
        pub fn initiating_party<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PartyIdentification1352>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.initiating_party = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for initiating_party: {e}")
                });
            self
        }
        pub fn instructed_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::BranchAndFinancialInstitutionIdentification63,
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
        pub fn instructed_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.instructed_amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for instructed_amount: {e}")
                });
            self
        }
        pub fn instructing_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::BranchAndFinancialInstitutionIdentification63,
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
        pub fn instruction_for_creditor_agent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::InstructionForCreditorAgent1>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.instruction_for_creditor_agent = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for instruction_for_creditor_agent: {e}"
                    )
                });
            self
        }
        pub fn interbank_settlement_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActiveCurrencyAndAmount2decimalsCopy>,
            T::Error: ::std::fmt::Display,
        {
            self.interbank_settlement_amount = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for interbank_settlement_amount: {e}"
                    )
                });
            self
        }
        pub fn interbank_settlement_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IsoDate>,
            T::Error: ::std::fmt::Display,
        {
            self.interbank_settlement_date = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for interbank_settlement_date: {e}"
                    )
                });
            self
        }
        pub fn payment_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaymentIdentification71>,
            T::Error: ::std::fmt::Display,
        {
            self.payment_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for payment_identification: {e}"
                    )
                });
            self
        }
        pub fn payment_type_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PaymentTypeInformation281>,
            T::Error: ::std::fmt::Display,
        {
            self.payment_type_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for payment_type_information: {e}"
                    )
                });
            self
        }
        pub fn previous_instructing_agent1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::BranchAndFinancialInstitutionIdentification62,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.previous_instructing_agent1 = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for previous_instructing_agent1: {e}"
                    )
                });
            self
        }
        pub fn previous_instructing_agent1_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CashAccount38>>,
            T::Error: ::std::fmt::Display,
        {
            self.previous_instructing_agent1_account = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for previous_instructing_agent1_account: {e}"
                    )
                });
            self
        }
        pub fn previous_instructing_agent2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::BranchAndFinancialInstitutionIdentification62,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.previous_instructing_agent2 = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for previous_instructing_agent2: {e}"
                    )
                });
            self
        }
        pub fn previous_instructing_agent2_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CashAccount38>>,
            T::Error: ::std::fmt::Display,
        {
            self.previous_instructing_agent2_account = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for previous_instructing_agent2_account: {e}"
                    )
                });
            self
        }
        pub fn previous_instructing_agent3<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::BranchAndFinancialInstitutionIdentification62,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.previous_instructing_agent3 = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for previous_instructing_agent3: {e}"
                    )
                });
            self
        }
        pub fn previous_instructing_agent3_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CashAccount38>>,
            T::Error: ::std::fmt::Display,
        {
            self.previous_instructing_agent3_account = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for previous_instructing_agent3_account: {e}"
                    )
                });
            self
        }
        pub fn purpose<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Purpose2Choice>>,
            T::Error: ::std::fmt::Display,
        {
            self.purpose = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for purpose: {e}")
                });
            self
        }
        pub fn regulatory_reporting<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RegulatoryReporting3>>,
            T::Error: ::std::fmt::Display,
        {
            self.regulatory_reporting = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for regulatory_reporting: {e}"
                    )
                });
            self
        }
        pub fn related_remittance_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RemittanceLocation71>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.related_remittance_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for related_remittance_information: {e}"
                    )
                });
            self
        }
        pub fn remittance_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RemittanceInformation161>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.remittance_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for remittance_information: {e}"
                    )
                });
            self
        }
        pub fn ultimate_creditor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PartyIdentification1351>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ultimate_creditor = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ultimate_creditor: {e}")
                });
            self
        }
        pub fn ultimate_debtor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PartyIdentification1351>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ultimate_debtor = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ultimate_debtor: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CreditTransferTransaction391>
    for super::CreditTransferTransaction391 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreditTransferTransaction391,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acceptance_date_time: value.acceptance_date_time?,
                charge_bearer: value.charge_bearer?,
                charges_information: value.charges_information?,
                creditor: value.creditor?,
                creditor_account: value.creditor_account?,
                creditor_agent: value.creditor_agent?,
                creditor_agent_account: value.creditor_agent_account?,
                debtor: value.debtor?,
                debtor_account: value.debtor_account?,
                debtor_agent: value.debtor_agent?,
                debtor_agent_account: value.debtor_agent_account?,
                exchange_rate: value.exchange_rate?,
                initiating_party: value.initiating_party?,
                instructed_agent: value.instructed_agent?,
                instructed_amount: value.instructed_amount?,
                instructing_agent: value.instructing_agent?,
                instruction_for_creditor_agent: value.instruction_for_creditor_agent?,
                interbank_settlement_amount: value.interbank_settlement_amount?,
                interbank_settlement_date: value.interbank_settlement_date?,
                payment_identification: value.payment_identification?,
                payment_type_information: value.payment_type_information?,
                previous_instructing_agent1: value.previous_instructing_agent1?,
                previous_instructing_agent1_account: value
                    .previous_instructing_agent1_account?,
                previous_instructing_agent2: value.previous_instructing_agent2?,
                previous_instructing_agent2_account: value
                    .previous_instructing_agent2_account?,
                previous_instructing_agent3: value.previous_instructing_agent3?,
                previous_instructing_agent3_account: value
                    .previous_instructing_agent3_account?,
                purpose: value.purpose?,
                regulatory_reporting: value.regulatory_reporting?,
                related_remittance_information: value.related_remittance_information?,
                remittance_information: value.remittance_information?,
                ultimate_creditor: value.ultimate_creditor?,
                ultimate_debtor: value.ultimate_debtor?,
            })
        }
    }
    impl ::std::convert::From<super::CreditTransferTransaction391>
    for CreditTransferTransaction391 {
        fn from(value: super::CreditTransferTransaction391) -> Self {
            Self {
                acceptance_date_time: Ok(value.acceptance_date_time),
                charge_bearer: Ok(value.charge_bearer),
                charges_information: Ok(value.charges_information),
                creditor: Ok(value.creditor),
                creditor_account: Ok(value.creditor_account),
                creditor_agent: Ok(value.creditor_agent),
                creditor_agent_account: Ok(value.creditor_agent_account),
                debtor: Ok(value.debtor),
                debtor_account: Ok(value.debtor_account),
                debtor_agent: Ok(value.debtor_agent),
                debtor_agent_account: Ok(value.debtor_agent_account),
                exchange_rate: Ok(value.exchange_rate),
                initiating_party: Ok(value.initiating_party),
                instructed_agent: Ok(value.instructed_agent),
                instructed_amount: Ok(value.instructed_amount),
                instructing_agent: Ok(value.instructing_agent),
                instruction_for_creditor_agent: Ok(value.instruction_for_creditor_agent),
                interbank_settlement_amount: Ok(value.interbank_settlement_amount),
                interbank_settlement_date: Ok(value.interbank_settlement_date),
                payment_identification: Ok(value.payment_identification),
                payment_type_information: Ok(value.payment_type_information),
                previous_instructing_agent1: Ok(value.previous_instructing_agent1),
                previous_instructing_agent1_account: Ok(
                    value.previous_instructing_agent1_account,
                ),
                previous_instructing_agent2: Ok(value.previous_instructing_agent2),
                previous_instructing_agent2_account: Ok(
                    value.previous_instructing_agent2_account,
                ),
                previous_instructing_agent3: Ok(value.previous_instructing_agent3),
                previous_instructing_agent3_account: Ok(
                    value.previous_instructing_agent3_account,
                ),
                purpose: Ok(value.purpose),
                regulatory_reporting: Ok(value.regulatory_reporting),
                related_remittance_information: Ok(value.related_remittance_information),
                remittance_information: Ok(value.remittance_information),
                ultimate_creditor: Ok(value.ultimate_creditor),
                ultimate_debtor: Ok(value.ultimate_debtor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreditorReferenceInformation2 {
        reference: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::CreditorReferenceType2>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreditorReferenceInformation2 {
        fn default() -> Self {
            Self {
                reference: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl CreditorReferenceInformation2 {
        pub fn reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for reference: {e}")
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreditorReferenceType2>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreditorReferenceInformation2>
    for super::CreditorReferenceInformation2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreditorReferenceInformation2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reference: value.reference?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreditorReferenceInformation2>
    for CreditorReferenceInformation2 {
        fn from(value: super::CreditorReferenceInformation2) -> Self {
            Self {
                reference: Ok(value.reference),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreditorReferenceType2 {
        code_or_proprietary: ::std::result::Result<
            super::CreditorReferenceType1Choice,
            ::std::string::String,
        >,
        issuer: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreditorReferenceType2 {
        fn default() -> Self {
            Self {
                code_or_proprietary: Err(
                    "no value supplied for code_or_proprietary".to_string(),
                ),
                issuer: Ok(Default::default()),
            }
        }
    }
    impl CreditorReferenceType2 {
        pub fn code_or_proprietary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreditorReferenceType1Choice>,
            T::Error: ::std::fmt::Display,
        {
            self.code_or_proprietary = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for code_or_proprietary: {e}"
                    )
                });
            self
        }
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreditorReferenceType2>
    for super::CreditorReferenceType2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreditorReferenceType2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_or_proprietary: value.code_or_proprietary?,
                issuer: value.issuer?,
            })
        }
    }
    impl ::std::convert::From<super::CreditorReferenceType2> for CreditorReferenceType2 {
        fn from(value: super::CreditorReferenceType2) -> Self {
            Self {
                code_or_proprietary: Ok(value.code_or_proprietary),
                issuer: Ok(value.issuer),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DateAndPlaceOfBirth1 {
        birth_date: ::std::result::Result<super::IsoDate, ::std::string::String>,
        city_of_birth: ::std::result::Result<super::Max35Text, ::std::string::String>,
        country_of_birth: ::std::result::Result<
            super::CountryCode,
            ::std::string::String,
        >,
        province_of_birth: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DateAndPlaceOfBirth1 {
        fn default() -> Self {
            Self {
                birth_date: Err("no value supplied for birth_date".to_string()),
                city_of_birth: Err("no value supplied for city_of_birth".to_string()),
                country_of_birth: Err(
                    "no value supplied for country_of_birth".to_string(),
                ),
                province_of_birth: Ok(Default::default()),
            }
        }
    }
    impl DateAndPlaceOfBirth1 {
        pub fn birth_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IsoDate>,
            T::Error: ::std::fmt::Display,
        {
            self.birth_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for birth_date: {e}")
                });
            self
        }
        pub fn city_of_birth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.city_of_birth = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for city_of_birth: {e}")
                });
            self
        }
        pub fn country_of_birth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CountryCode>,
            T::Error: ::std::fmt::Display,
        {
            self.country_of_birth = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country_of_birth: {e}")
                });
            self
        }
        pub fn province_of_birth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.province_of_birth = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for province_of_birth: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DateAndPlaceOfBirth1> for super::DateAndPlaceOfBirth1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DateAndPlaceOfBirth1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                birth_date: value.birth_date?,
                city_of_birth: value.city_of_birth?,
                country_of_birth: value.country_of_birth?,
                province_of_birth: value.province_of_birth?,
            })
        }
    }
    impl ::std::convert::From<super::DateAndPlaceOfBirth1> for DateAndPlaceOfBirth1 {
        fn from(value: super::DateAndPlaceOfBirth1) -> Self {
            Self {
                birth_date: Ok(value.birth_date),
                city_of_birth: Ok(value.city_of_birth),
                country_of_birth: Ok(value.country_of_birth),
                province_of_birth: Ok(value.province_of_birth),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DatePeriod2 {
        from_date: ::std::result::Result<super::IsoDate, ::std::string::String>,
        to_date: ::std::result::Result<super::IsoDate, ::std::string::String>,
    }
    impl ::std::default::Default for DatePeriod2 {
        fn default() -> Self {
            Self {
                from_date: Err("no value supplied for from_date".to_string()),
                to_date: Err("no value supplied for to_date".to_string()),
            }
        }
    }
    impl DatePeriod2 {
        pub fn from_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IsoDate>,
            T::Error: ::std::fmt::Display,
        {
            self.from_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for from_date: {e}")
                });
            self
        }
        pub fn to_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IsoDate>,
            T::Error: ::std::fmt::Display,
        {
            self.to_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for to_date: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DatePeriod2> for super::DatePeriod2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DatePeriod2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                from_date: value.from_date?,
                to_date: value.to_date?,
            })
        }
    }
    impl ::std::convert::From<super::DatePeriod2> for DatePeriod2 {
        fn from(value: super::DatePeriod2) -> Self {
            Self {
                from_date: Ok(value.from_date),
                to_date: Ok(value.to_date),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DiscountAmountAndType1 {
        amount: ::std::result::Result<
            super::ActiveOrHistoricCurrencyAndAmount,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::DiscountAmountType1Choice>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DiscountAmountAndType1 {
        fn default() -> Self {
            Self {
                amount: Err("no value supplied for amount".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl DiscountAmountAndType1 {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActiveOrHistoricCurrencyAndAmount>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DiscountAmountType1Choice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiscountAmountAndType1>
    for super::DiscountAmountAndType1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiscountAmountAndType1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::DiscountAmountAndType1> for DiscountAmountAndType1 {
        fn from(value: super::DiscountAmountAndType1) -> Self {
            Self {
                amount: Ok(value.amount),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DocumentAdjustment1 {
        additional_information: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        amount: ::std::result::Result<
            super::ActiveOrHistoricCurrencyAndAmount,
            ::std::string::String,
        >,
        credit_debit_indicator: ::std::result::Result<
            ::std::option::Option<super::CreditDebitCode>,
            ::std::string::String,
        >,
        reason: ::std::result::Result<
            ::std::option::Option<super::Max4Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DocumentAdjustment1 {
        fn default() -> Self {
            Self {
                additional_information: Ok(Default::default()),
                amount: Err("no value supplied for amount".to_string()),
                credit_debit_indicator: Ok(Default::default()),
                reason: Ok(Default::default()),
            }
        }
    }
    impl DocumentAdjustment1 {
        pub fn additional_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.additional_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for additional_information: {e}"
                    )
                });
            self
        }
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActiveOrHistoricCurrencyAndAmount>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn credit_debit_indicator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CreditDebitCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.credit_debit_indicator = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credit_debit_indicator: {e}"
                    )
                });
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max4Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DocumentAdjustment1> for super::DocumentAdjustment1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DocumentAdjustment1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_information: value.additional_information?,
                amount: value.amount?,
                credit_debit_indicator: value.credit_debit_indicator?,
                reason: value.reason?,
            })
        }
    }
    impl ::std::convert::From<super::DocumentAdjustment1> for DocumentAdjustment1 {
        fn from(value: super::DocumentAdjustment1) -> Self {
            Self {
                additional_information: Ok(value.additional_information),
                amount: Ok(value.amount),
                credit_debit_indicator: Ok(value.credit_debit_indicator),
                reason: Ok(value.reason),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DocumentLineIdentification1 {
        number: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        related_date: ::std::result::Result<
            ::std::option::Option<super::IsoDate>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::DocumentLineType1>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DocumentLineIdentification1 {
        fn default() -> Self {
            Self {
                number: Ok(Default::default()),
                related_date: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl DocumentLineIdentification1 {
        pub fn number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number: {e}"));
            self
        }
        pub fn related_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IsoDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.related_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for related_date: {e}")
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DocumentLineType1>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DocumentLineIdentification1>
    for super::DocumentLineIdentification1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DocumentLineIdentification1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                number: value.number?,
                related_date: value.related_date?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::DocumentLineIdentification1>
    for DocumentLineIdentification1 {
        fn from(value: super::DocumentLineIdentification1) -> Self {
            Self {
                number: Ok(value.number),
                related_date: Ok(value.related_date),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DocumentLineInformation11 {
        amount: ::std::result::Result<
            ::std::option::Option<super::RemittanceAmount3>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        identification: ::std::result::Result<
            ::std::vec::Vec<super::DocumentLineIdentification1>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DocumentLineInformation11 {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                description: Ok(Default::default()),
                identification: Err("no value supplied for identification".to_string()),
            }
        }
    }
    impl DocumentLineInformation11 {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RemittanceAmount3>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {e}")
                });
            self
        }
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::DocumentLineIdentification1>,
            >,
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
    impl ::std::convert::TryFrom<DocumentLineInformation11>
    for super::DocumentLineInformation11 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DocumentLineInformation11,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                description: value.description?,
                identification: value.identification?,
            })
        }
    }
    impl ::std::convert::From<super::DocumentLineInformation11>
    for DocumentLineInformation11 {
        fn from(value: super::DocumentLineInformation11) -> Self {
            Self {
                amount: Ok(value.amount),
                description: Ok(value.description),
                identification: Ok(value.identification),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DocumentLineType1 {
        code_or_proprietary: ::std::result::Result<
            super::DocumentLineType1Choice,
            ::std::string::String,
        >,
        issuer: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DocumentLineType1 {
        fn default() -> Self {
            Self {
                code_or_proprietary: Err(
                    "no value supplied for code_or_proprietary".to_string(),
                ),
                issuer: Ok(Default::default()),
            }
        }
    }
    impl DocumentLineType1 {
        pub fn code_or_proprietary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DocumentLineType1Choice>,
            T::Error: ::std::fmt::Display,
        {
            self.code_or_proprietary = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for code_or_proprietary: {e}"
                    )
                });
            self
        }
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DocumentLineType1> for super::DocumentLineType1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DocumentLineType1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_or_proprietary: value.code_or_proprietary?,
                issuer: value.issuer?,
            })
        }
    }
    impl ::std::convert::From<super::DocumentLineType1> for DocumentLineType1 {
        fn from(value: super::DocumentLineType1) -> Self {
            Self {
                code_or_proprietary: Ok(value.code_or_proprietary),
                issuer: Ok(value.issuer),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FiToFiCustomerCreditTransferV08 {
        credit_transfer_transaction_information: ::std::result::Result<
            super::CreditTransferTransaction391,
            ::std::string::String,
        >,
        group_header: ::std::result::Result<
            super::GroupHeader931,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FiToFiCustomerCreditTransferV08 {
        fn default() -> Self {
            Self {
                credit_transfer_transaction_information: Err(
                    "no value supplied for credit_transfer_transaction_information"
                        .to_string(),
                ),
                group_header: Err("no value supplied for group_header".to_string()),
            }
        }
    }
    impl FiToFiCustomerCreditTransferV08 {
        pub fn credit_transfer_transaction_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreditTransferTransaction391>,
            T::Error: ::std::fmt::Display,
        {
            self.credit_transfer_transaction_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credit_transfer_transaction_information: {e}"
                    )
                });
            self
        }
        pub fn group_header<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupHeader931>,
            T::Error: ::std::fmt::Display,
        {
            self.group_header = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for group_header: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FiToFiCustomerCreditTransferV08>
    for super::FiToFiCustomerCreditTransferV08 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FiToFiCustomerCreditTransferV08,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                credit_transfer_transaction_information: value
                    .credit_transfer_transaction_information?,
                group_header: value.group_header?,
            })
        }
    }
    impl ::std::convert::From<super::FiToFiCustomerCreditTransferV08>
    for FiToFiCustomerCreditTransferV08 {
        fn from(value: super::FiToFiCustomerCreditTransferV08) -> Self {
            Self {
                credit_transfer_transaction_information: Ok(
                    value.credit_transfer_transaction_information,
                ),
                group_header: Ok(value.group_header),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FinancialInstitutionIdentification181 {
        bicfi: ::std::result::Result<
            ::std::option::Option<super::BicfiDec2014Identifier>,
            ::std::string::String,
        >,
        clearing_system_member_identification: ::std::result::Result<
            ::std::option::Option<super::ClearingSystemMemberIdentification21>,
            ::std::string::String,
        >,
        lei: ::std::result::Result<
            ::std::option::Option<super::LeiIdentifier>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        postal_address: ::std::result::Result<
            ::std::option::Option<super::PostalAddress241>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FinancialInstitutionIdentification181 {
        fn default() -> Self {
            Self {
                bicfi: Ok(Default::default()),
                clearing_system_member_identification: Ok(Default::default()),
                lei: Ok(Default::default()),
                name: Ok(Default::default()),
                postal_address: Ok(Default::default()),
            }
        }
    }
    impl FinancialInstitutionIdentification181 {
        pub fn bicfi<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::BicfiDec2014Identifier>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bicfi = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bicfi: {e}"));
            self
        }
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
        pub fn lei<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LeiIdentifier>>,
            T::Error: ::std::fmt::Display,
        {
            self.lei = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lei: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn postal_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostalAddress241>>,
            T::Error: ::std::fmt::Display,
        {
            self.postal_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for postal_address: {e}")
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
                bicfi: value.bicfi?,
                clearing_system_member_identification: value
                    .clearing_system_member_identification?,
                lei: value.lei?,
                name: value.name?,
                postal_address: value.postal_address?,
            })
        }
    }
    impl ::std::convert::From<super::FinancialInstitutionIdentification181>
    for FinancialInstitutionIdentification181 {
        fn from(value: super::FinancialInstitutionIdentification181) -> Self {
            Self {
                bicfi: Ok(value.bicfi),
                clearing_system_member_identification: Ok(
                    value.clearing_system_member_identification,
                ),
                lei: Ok(value.lei),
                name: Ok(value.name),
                postal_address: Ok(value.postal_address),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FinancialInstitutionIdentification182 {
        bicfi: ::std::result::Result<
            ::std::option::Option<super::BicfiDec2014Identifier>,
            ::std::string::String,
        >,
        clearing_system_member_identification: ::std::result::Result<
            ::std::option::Option<super::ClearingSystemMemberIdentification2>,
            ::std::string::String,
        >,
        lei: ::std::result::Result<
            ::std::option::Option<super::LeiIdentifier>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        other: ::std::result::Result<
            ::std::option::Option<super::GenericFinancialIdentification1>,
            ::std::string::String,
        >,
        postal_address: ::std::result::Result<
            ::std::option::Option<super::PostalAddress241>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FinancialInstitutionIdentification182 {
        fn default() -> Self {
            Self {
                bicfi: Ok(Default::default()),
                clearing_system_member_identification: Ok(Default::default()),
                lei: Ok(Default::default()),
                name: Ok(Default::default()),
                other: Ok(Default::default()),
                postal_address: Ok(Default::default()),
            }
        }
    }
    impl FinancialInstitutionIdentification182 {
        pub fn bicfi<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::BicfiDec2014Identifier>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bicfi = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bicfi: {e}"));
            self
        }
        pub fn clearing_system_member_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ClearingSystemMemberIdentification2>,
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
        pub fn lei<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LeiIdentifier>>,
            T::Error: ::std::fmt::Display,
        {
            self.lei = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lei: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn other<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::GenericFinancialIdentification1>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.other = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for other: {e}"));
            self
        }
        pub fn postal_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostalAddress241>>,
            T::Error: ::std::fmt::Display,
        {
            self.postal_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for postal_address: {e}")
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
                bicfi: value.bicfi?,
                clearing_system_member_identification: value
                    .clearing_system_member_identification?,
                lei: value.lei?,
                name: value.name?,
                other: value.other?,
                postal_address: value.postal_address?,
            })
        }
    }
    impl ::std::convert::From<super::FinancialInstitutionIdentification182>
    for FinancialInstitutionIdentification182 {
        fn from(value: super::FinancialInstitutionIdentification182) -> Self {
            Self {
                bicfi: Ok(value.bicfi),
                clearing_system_member_identification: Ok(
                    value.clearing_system_member_identification,
                ),
                lei: Ok(value.lei),
                name: Ok(value.name),
                other: Ok(value.other),
                postal_address: Ok(value.postal_address),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FinancialInstitutionIdentification183 {
        clearing_system_member_identification: ::std::result::Result<
            super::ClearingSystemMemberIdentification22,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FinancialInstitutionIdentification183 {
        fn default() -> Self {
            Self {
                clearing_system_member_identification: Err(
                    "no value supplied for clearing_system_member_identification"
                        .to_string(),
                ),
            }
        }
    }
    impl FinancialInstitutionIdentification183 {
        pub fn clearing_system_member_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ClearingSystemMemberIdentification22>,
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
    impl ::std::convert::TryFrom<FinancialInstitutionIdentification183>
    for super::FinancialInstitutionIdentification183 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FinancialInstitutionIdentification183,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                clearing_system_member_identification: value
                    .clearing_system_member_identification?,
            })
        }
    }
    impl ::std::convert::From<super::FinancialInstitutionIdentification183>
    for FinancialInstitutionIdentification183 {
        fn from(value: super::FinancialInstitutionIdentification183) -> Self {
            Self {
                clearing_system_member_identification: Ok(
                    value.clearing_system_member_identification,
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Garnishment31 {
        date: ::std::result::Result<
            ::std::option::Option<super::IsoDate>,
            ::std::string::String,
        >,
        employee_termination_indicator: ::std::result::Result<
            ::std::option::Option<super::TrueFalseIndicator>,
            ::std::string::String,
        >,
        family_medical_insurance_indicator: ::std::result::Result<
            ::std::option::Option<super::TrueFalseIndicator>,
            ::std::string::String,
        >,
        garnishee: ::std::result::Result<
            ::std::option::Option<super::PartyIdentification1355>,
            ::std::string::String,
        >,
        garnishment_administrator: ::std::result::Result<
            ::std::option::Option<super::PartyIdentification1355>,
            ::std::string::String,
        >,
        reference_number: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        remitted_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::GarnishmentType1, ::std::string::String>,
    }
    impl ::std::default::Default for Garnishment31 {
        fn default() -> Self {
            Self {
                date: Ok(Default::default()),
                employee_termination_indicator: Ok(Default::default()),
                family_medical_insurance_indicator: Ok(Default::default()),
                garnishee: Ok(Default::default()),
                garnishment_administrator: Ok(Default::default()),
                reference_number: Ok(Default::default()),
                remitted_amount: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Garnishment31 {
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IsoDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {e}"));
            self
        }
        pub fn employee_termination_indicator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrueFalseIndicator>>,
            T::Error: ::std::fmt::Display,
        {
            self.employee_termination_indicator = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for employee_termination_indicator: {e}"
                    )
                });
            self
        }
        pub fn family_medical_insurance_indicator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrueFalseIndicator>>,
            T::Error: ::std::fmt::Display,
        {
            self.family_medical_insurance_indicator = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for family_medical_insurance_indicator: {e}"
                    )
                });
            self
        }
        pub fn garnishee<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PartyIdentification1355>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.garnishee = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for garnishee: {e}")
                });
            self
        }
        pub fn garnishment_administrator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PartyIdentification1355>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.garnishment_administrator = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for garnishment_administrator: {e}"
                    )
                });
            self
        }
        pub fn reference_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for reference_number: {e}")
                });
            self
        }
        pub fn remitted_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.remitted_amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for remitted_amount: {e}")
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GarnishmentType1>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Garnishment31> for super::Garnishment31 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Garnishment31,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                date: value.date?,
                employee_termination_indicator: value.employee_termination_indicator?,
                family_medical_insurance_indicator: value
                    .family_medical_insurance_indicator?,
                garnishee: value.garnishee?,
                garnishment_administrator: value.garnishment_administrator?,
                reference_number: value.reference_number?,
                remitted_amount: value.remitted_amount?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Garnishment31> for Garnishment31 {
        fn from(value: super::Garnishment31) -> Self {
            Self {
                date: Ok(value.date),
                employee_termination_indicator: Ok(value.employee_termination_indicator),
                family_medical_insurance_indicator: Ok(
                    value.family_medical_insurance_indicator,
                ),
                garnishee: Ok(value.garnishee),
                garnishment_administrator: Ok(value.garnishment_administrator),
                reference_number: Ok(value.reference_number),
                remitted_amount: Ok(value.remitted_amount),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GarnishmentType1 {
        code_or_proprietary: ::std::result::Result<
            super::GarnishmentType1Choice,
            ::std::string::String,
        >,
        issuer: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GarnishmentType1 {
        fn default() -> Self {
            Self {
                code_or_proprietary: Err(
                    "no value supplied for code_or_proprietary".to_string(),
                ),
                issuer: Ok(Default::default()),
            }
        }
    }
    impl GarnishmentType1 {
        pub fn code_or_proprietary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GarnishmentType1Choice>,
            T::Error: ::std::fmt::Display,
        {
            self.code_or_proprietary = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for code_or_proprietary: {e}"
                    )
                });
            self
        }
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GarnishmentType1> for super::GarnishmentType1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GarnishmentType1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_or_proprietary: value.code_or_proprietary?,
                issuer: value.issuer?,
            })
        }
    }
    impl ::std::convert::From<super::GarnishmentType1> for GarnishmentType1 {
        fn from(value: super::GarnishmentType1) -> Self {
            Self {
                code_or_proprietary: Ok(value.code_or_proprietary),
                issuer: Ok(value.issuer),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GenericAccountIdentification1 {
        identification: ::std::result::Result<super::Max34Text, ::std::string::String>,
        issuer: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        scheme_name: ::std::result::Result<
            ::std::option::Option<super::AccountSchemeName1Choice>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GenericAccountIdentification1 {
        fn default() -> Self {
            Self {
                identification: Err("no value supplied for identification".to_string()),
                issuer: Ok(Default::default()),
                scheme_name: Ok(Default::default()),
            }
        }
    }
    impl GenericAccountIdentification1 {
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max34Text>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {e}"));
            self
        }
        pub fn scheme_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AccountSchemeName1Choice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.scheme_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_name: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<GenericAccountIdentification1>
    for super::GenericAccountIdentification1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GenericAccountIdentification1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                identification: value.identification?,
                issuer: value.issuer?,
                scheme_name: value.scheme_name?,
            })
        }
    }
    impl ::std::convert::From<super::GenericAccountIdentification1>
    for GenericAccountIdentification1 {
        fn from(value: super::GenericAccountIdentification1) -> Self {
            Self {
                identification: Ok(value.identification),
                issuer: Ok(value.issuer),
                scheme_name: Ok(value.scheme_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GenericFinancialIdentification1 {
        identification: ::std::result::Result<super::Max35Text, ::std::string::String>,
        issuer: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        scheme_name: ::std::result::Result<
            ::std::option::Option<super::FinancialIdentificationSchemeName1Choice>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GenericFinancialIdentification1 {
        fn default() -> Self {
            Self {
                identification: Err("no value supplied for identification".to_string()),
                issuer: Ok(Default::default()),
                scheme_name: Ok(Default::default()),
            }
        }
    }
    impl GenericFinancialIdentification1 {
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
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {e}"));
            self
        }
        pub fn scheme_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FinancialIdentificationSchemeName1Choice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.scheme_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_name: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<GenericFinancialIdentification1>
    for super::GenericFinancialIdentification1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GenericFinancialIdentification1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                identification: value.identification?,
                issuer: value.issuer?,
                scheme_name: value.scheme_name?,
            })
        }
    }
    impl ::std::convert::From<super::GenericFinancialIdentification1>
    for GenericFinancialIdentification1 {
        fn from(value: super::GenericFinancialIdentification1) -> Self {
            Self {
                identification: Ok(value.identification),
                issuer: Ok(value.issuer),
                scheme_name: Ok(value.scheme_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GenericOrganisationIdentification1 {
        identification: ::std::result::Result<super::Max35Text, ::std::string::String>,
        issuer: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        scheme_name: ::std::result::Result<
            ::std::option::Option<super::OrganisationIdentificationSchemeName1Choice>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GenericOrganisationIdentification1 {
        fn default() -> Self {
            Self {
                identification: Err("no value supplied for identification".to_string()),
                issuer: Ok(Default::default()),
                scheme_name: Ok(Default::default()),
            }
        }
    }
    impl GenericOrganisationIdentification1 {
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
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {e}"));
            self
        }
        pub fn scheme_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::OrganisationIdentificationSchemeName1Choice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.scheme_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_name: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<GenericOrganisationIdentification1>
    for super::GenericOrganisationIdentification1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GenericOrganisationIdentification1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                identification: value.identification?,
                issuer: value.issuer?,
                scheme_name: value.scheme_name?,
            })
        }
    }
    impl ::std::convert::From<super::GenericOrganisationIdentification1>
    for GenericOrganisationIdentification1 {
        fn from(value: super::GenericOrganisationIdentification1) -> Self {
            Self {
                identification: Ok(value.identification),
                issuer: Ok(value.issuer),
                scheme_name: Ok(value.scheme_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GenericPersonIdentification1 {
        identification: ::std::result::Result<super::Max35Text, ::std::string::String>,
        issuer: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        scheme_name: ::std::result::Result<
            ::std::option::Option<super::PersonIdentificationSchemeName1Choice>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GenericPersonIdentification1 {
        fn default() -> Self {
            Self {
                identification: Err("no value supplied for identification".to_string()),
                issuer: Ok(Default::default()),
                scheme_name: Ok(Default::default()),
            }
        }
    }
    impl GenericPersonIdentification1 {
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
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {e}"));
            self
        }
        pub fn scheme_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PersonIdentificationSchemeName1Choice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.scheme_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scheme_name: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<GenericPersonIdentification1>
    for super::GenericPersonIdentification1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GenericPersonIdentification1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                identification: value.identification?,
                issuer: value.issuer?,
                scheme_name: value.scheme_name?,
            })
        }
    }
    impl ::std::convert::From<super::GenericPersonIdentification1>
    for GenericPersonIdentification1 {
        fn from(value: super::GenericPersonIdentification1) -> Self {
            Self {
                identification: Ok(value.identification),
                issuer: Ok(value.issuer),
                scheme_name: Ok(value.scheme_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupHeader931 {
        creation_date_time: ::std::result::Result<
            super::IsoNormalisedDateTime,
            ::std::string::String,
        >,
        message_identification: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
        number_of_transactions: ::std::result::Result<
            super::Max15NumericTextFixed,
            ::std::string::String,
        >,
        settlement_information: ::std::result::Result<
            super::SettlementInstruction71,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GroupHeader931 {
        fn default() -> Self {
            Self {
                creation_date_time: Err(
                    "no value supplied for creation_date_time".to_string(),
                ),
                message_identification: Err(
                    "no value supplied for message_identification".to_string(),
                ),
                number_of_transactions: Err(
                    "no value supplied for number_of_transactions".to_string(),
                ),
                settlement_information: Err(
                    "no value supplied for settlement_information".to_string(),
                ),
            }
        }
    }
    impl GroupHeader931 {
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
        pub fn number_of_transactions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max15NumericTextFixed>,
            T::Error: ::std::fmt::Display,
        {
            self.number_of_transactions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for number_of_transactions: {e}"
                    )
                });
            self
        }
        pub fn settlement_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SettlementInstruction71>,
            T::Error: ::std::fmt::Display,
        {
            self.settlement_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for settlement_information: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<GroupHeader931> for super::GroupHeader931 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupHeader931,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                creation_date_time: value.creation_date_time?,
                message_identification: value.message_identification?,
                number_of_transactions: value.number_of_transactions?,
                settlement_information: value.settlement_information?,
            })
        }
    }
    impl ::std::convert::From<super::GroupHeader931> for GroupHeader931 {
        fn from(value: super::GroupHeader931) -> Self {
            Self {
                creation_date_time: Ok(value.creation_date_time),
                message_identification: Ok(value.message_identification),
                number_of_transactions: Ok(value.number_of_transactions),
                settlement_information: Ok(value.settlement_information),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InstructionForCreditorAgent1 {
        code: ::std::result::Result<
            ::std::option::Option<super::Instruction3Code>,
            ::std::string::String,
        >,
        instruction_information: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for InstructionForCreditorAgent1 {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                instruction_information: Ok(Default::default()),
            }
        }
    }
    impl InstructionForCreditorAgent1 {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Instruction3Code>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn instruction_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.instruction_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for instruction_information: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<InstructionForCreditorAgent1>
    for super::InstructionForCreditorAgent1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InstructionForCreditorAgent1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                instruction_information: value.instruction_information?,
            })
        }
    }
    impl ::std::convert::From<super::InstructionForCreditorAgent1>
    for InstructionForCreditorAgent1 {
        fn from(value: super::InstructionForCreditorAgent1) -> Self {
            Self {
                code: Ok(value.code),
                instruction_information: Ok(value.instruction_information),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LocalInstrument2Choice1 {
        proprietary: ::std::result::Result<super::Max35Text, ::std::string::String>,
    }
    impl ::std::default::Default for LocalInstrument2Choice1 {
        fn default() -> Self {
            Self {
                proprietary: Err("no value supplied for proprietary".to_string()),
            }
        }
    }
    impl LocalInstrument2Choice1 {
        pub fn proprietary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.proprietary = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proprietary: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LocalInstrument2Choice1>
    for super::LocalInstrument2Choice1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LocalInstrument2Choice1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                proprietary: value.proprietary?,
            })
        }
    }
    impl ::std::convert::From<super::LocalInstrument2Choice1>
    for LocalInstrument2Choice1 {
        fn from(value: super::LocalInstrument2Choice1) -> Self {
            Self {
                proprietary: Ok(value.proprietary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NameAndAddress161 {
        address: ::std::result::Result<super::PostalAddress241, ::std::string::String>,
        name: ::std::result::Result<super::Max140Text, ::std::string::String>,
    }
    impl ::std::default::Default for NameAndAddress161 {
        fn default() -> Self {
            Self {
                address: Err("no value supplied for address".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl NameAndAddress161 {
        pub fn address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PostalAddress241>,
            T::Error: ::std::fmt::Display,
        {
            self.address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for address: {e}")
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max140Text>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NameAndAddress161> for super::NameAndAddress161 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NameAndAddress161,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                address: value.address?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::NameAndAddress161> for NameAndAddress161 {
        fn from(value: super::NameAndAddress161) -> Self {
            Self {
                address: Ok(value.address),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganisationIdentification291 {
        any_bic: ::std::result::Result<
            ::std::option::Option<super::AnyBicDec2014Identifier>,
            ::std::string::String,
        >,
        lei: ::std::result::Result<
            ::std::option::Option<super::LeiIdentifier>,
            ::std::string::String,
        >,
        other: ::std::result::Result<
            ::std::vec::Vec<super::GenericOrganisationIdentification1>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganisationIdentification291 {
        fn default() -> Self {
            Self {
                any_bic: Ok(Default::default()),
                lei: Ok(Default::default()),
                other: Ok(Default::default()),
            }
        }
    }
    impl OrganisationIdentification291 {
        pub fn any_bic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AnyBicDec2014Identifier>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.any_bic = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for any_bic: {e}")
                });
            self
        }
        pub fn lei<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LeiIdentifier>>,
            T::Error: ::std::fmt::Display,
        {
            self.lei = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lei: {e}"));
            self
        }
        pub fn other<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::GenericOrganisationIdentification1>,
            >,
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
            Ok(Self {
                any_bic: value.any_bic?,
                lei: value.lei?,
                other: value.other?,
            })
        }
    }
    impl ::std::convert::From<super::OrganisationIdentification291>
    for OrganisationIdentification291 {
        fn from(value: super::OrganisationIdentification291) -> Self {
            Self {
                any_bic: Ok(value.any_bic),
                lei: Ok(value.lei),
                other: Ok(value.other),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PartyIdentification1351 {
        country_of_residence: ::std::result::Result<
            ::std::option::Option<super::CountryCode>,
            ::std::string::String,
        >,
        identification: ::std::result::Result<
            ::std::option::Option<super::Party38Choice1>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::Max140Text, ::std::string::String>,
        postal_address: ::std::result::Result<
            ::std::option::Option<super::PostalAddress242>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PartyIdentification1351 {
        fn default() -> Self {
            Self {
                country_of_residence: Ok(Default::default()),
                identification: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                postal_address: Ok(Default::default()),
            }
        }
    }
    impl PartyIdentification1351 {
        pub fn country_of_residence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CountryCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_of_residence = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for country_of_residence: {e}"
                    )
                });
            self
        }
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Party38Choice1>>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max140Text>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn postal_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostalAddress242>>,
            T::Error: ::std::fmt::Display,
        {
            self.postal_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for postal_address: {e}")
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
                country_of_residence: value.country_of_residence?,
                identification: value.identification?,
                name: value.name?,
                postal_address: value.postal_address?,
            })
        }
    }
    impl ::std::convert::From<super::PartyIdentification1351>
    for PartyIdentification1351 {
        fn from(value: super::PartyIdentification1351) -> Self {
            Self {
                country_of_residence: Ok(value.country_of_residence),
                identification: Ok(value.identification),
                name: Ok(value.name),
                postal_address: Ok(value.postal_address),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PartyIdentification1352 {
        contact_details: ::std::result::Result<
            ::std::option::Option<super::Contact41>,
            ::std::string::String,
        >,
        country_of_residence: ::std::result::Result<
            ::std::option::Option<super::CountryCode>,
            ::std::string::String,
        >,
        identification: ::std::result::Result<
            ::std::option::Option<super::Party38Choice1>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::Max140Text, ::std::string::String>,
        postal_address: ::std::result::Result<
            ::std::option::Option<super::PostalAddress242>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PartyIdentification1352 {
        fn default() -> Self {
            Self {
                contact_details: Ok(Default::default()),
                country_of_residence: Ok(Default::default()),
                identification: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                postal_address: Ok(Default::default()),
            }
        }
    }
    impl PartyIdentification1352 {
        pub fn contact_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Contact41>>,
            T::Error: ::std::fmt::Display,
        {
            self.contact_details = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for contact_details: {e}")
                });
            self
        }
        pub fn country_of_residence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CountryCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_of_residence = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for country_of_residence: {e}"
                    )
                });
            self
        }
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Party38Choice1>>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max140Text>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn postal_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostalAddress242>>,
            T::Error: ::std::fmt::Display,
        {
            self.postal_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for postal_address: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PartyIdentification1352>
    for super::PartyIdentification1352 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PartyIdentification1352,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                contact_details: value.contact_details?,
                country_of_residence: value.country_of_residence?,
                identification: value.identification?,
                name: value.name?,
                postal_address: value.postal_address?,
            })
        }
    }
    impl ::std::convert::From<super::PartyIdentification1352>
    for PartyIdentification1352 {
        fn from(value: super::PartyIdentification1352) -> Self {
            Self {
                contact_details: Ok(value.contact_details),
                country_of_residence: Ok(value.country_of_residence),
                identification: Ok(value.identification),
                name: Ok(value.name),
                postal_address: Ok(value.postal_address),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PartyIdentification1353 {
        contact_details: ::std::result::Result<
            ::std::option::Option<super::Contact42>,
            ::std::string::String,
        >,
        country_of_residence: ::std::result::Result<
            ::std::option::Option<super::CountryCode>,
            ::std::string::String,
        >,
        identification: ::std::result::Result<
            ::std::option::Option<super::Party38Choice1>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::Max140Text, ::std::string::String>,
        postal_address: ::std::result::Result<
            ::std::option::Option<super::PostalAddress241>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PartyIdentification1353 {
        fn default() -> Self {
            Self {
                contact_details: Ok(Default::default()),
                country_of_residence: Ok(Default::default()),
                identification: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                postal_address: Ok(Default::default()),
            }
        }
    }
    impl PartyIdentification1353 {
        pub fn contact_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Contact42>>,
            T::Error: ::std::fmt::Display,
        {
            self.contact_details = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for contact_details: {e}")
                });
            self
        }
        pub fn country_of_residence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CountryCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_of_residence = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for country_of_residence: {e}"
                    )
                });
            self
        }
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Party38Choice1>>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max140Text>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn postal_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostalAddress241>>,
            T::Error: ::std::fmt::Display,
        {
            self.postal_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for postal_address: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PartyIdentification1353>
    for super::PartyIdentification1353 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PartyIdentification1353,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                contact_details: value.contact_details?,
                country_of_residence: value.country_of_residence?,
                identification: value.identification?,
                name: value.name?,
                postal_address: value.postal_address?,
            })
        }
    }
    impl ::std::convert::From<super::PartyIdentification1353>
    for PartyIdentification1353 {
        fn from(value: super::PartyIdentification1353) -> Self {
            Self {
                contact_details: Ok(value.contact_details),
                country_of_residence: Ok(value.country_of_residence),
                identification: Ok(value.identification),
                name: Ok(value.name),
                postal_address: Ok(value.postal_address),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PartyIdentification1354 {
        contact_details: ::std::result::Result<
            ::std::option::Option<super::Contact43>,
            ::std::string::String,
        >,
        country_of_residence: ::std::result::Result<
            ::std::option::Option<super::CountryCode>,
            ::std::string::String,
        >,
        identification: ::std::result::Result<
            ::std::option::Option<super::Party38Choice1>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        postal_address: ::std::result::Result<
            ::std::option::Option<super::PostalAddress242>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PartyIdentification1354 {
        fn default() -> Self {
            Self {
                contact_details: Ok(Default::default()),
                country_of_residence: Ok(Default::default()),
                identification: Ok(Default::default()),
                name: Ok(Default::default()),
                postal_address: Ok(Default::default()),
            }
        }
    }
    impl PartyIdentification1354 {
        pub fn contact_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Contact43>>,
            T::Error: ::std::fmt::Display,
        {
            self.contact_details = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for contact_details: {e}")
                });
            self
        }
        pub fn country_of_residence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CountryCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_of_residence = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for country_of_residence: {e}"
                    )
                });
            self
        }
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Party38Choice1>>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn postal_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostalAddress242>>,
            T::Error: ::std::fmt::Display,
        {
            self.postal_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for postal_address: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PartyIdentification1354>
    for super::PartyIdentification1354 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PartyIdentification1354,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                contact_details: value.contact_details?,
                country_of_residence: value.country_of_residence?,
                identification: value.identification?,
                name: value.name?,
                postal_address: value.postal_address?,
            })
        }
    }
    impl ::std::convert::From<super::PartyIdentification1354>
    for PartyIdentification1354 {
        fn from(value: super::PartyIdentification1354) -> Self {
            Self {
                contact_details: Ok(value.contact_details),
                country_of_residence: Ok(value.country_of_residence),
                identification: Ok(value.identification),
                name: Ok(value.name),
                postal_address: Ok(value.postal_address),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PartyIdentification1355 {
        country_of_residence: ::std::result::Result<
            ::std::option::Option<super::CountryCode>,
            ::std::string::String,
        >,
        identification: ::std::result::Result<
            ::std::option::Option<super::Party38Choice1>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        postal_address: ::std::result::Result<
            ::std::option::Option<super::PostalAddress242>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PartyIdentification1355 {
        fn default() -> Self {
            Self {
                country_of_residence: Ok(Default::default()),
                identification: Ok(Default::default()),
                name: Ok(Default::default()),
                postal_address: Ok(Default::default()),
            }
        }
    }
    impl PartyIdentification1355 {
        pub fn country_of_residence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CountryCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_of_residence = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for country_of_residence: {e}"
                    )
                });
            self
        }
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Party38Choice1>>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn postal_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PostalAddress242>>,
            T::Error: ::std::fmt::Display,
        {
            self.postal_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for postal_address: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PartyIdentification1355>
    for super::PartyIdentification1355 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PartyIdentification1355,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                country_of_residence: value.country_of_residence?,
                identification: value.identification?,
                name: value.name?,
                postal_address: value.postal_address?,
            })
        }
    }
    impl ::std::convert::From<super::PartyIdentification1355>
    for PartyIdentification1355 {
        fn from(value: super::PartyIdentification1355) -> Self {
            Self {
                country_of_residence: Ok(value.country_of_residence),
                identification: Ok(value.identification),
                name: Ok(value.name),
                postal_address: Ok(value.postal_address),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaymentIdentification71 {
        clearing_system_reference: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        end_to_end_identification: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
        instruction_identification: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        transaction_identification: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        uetr: ::std::result::Result<super::UuiDv4Identifier, ::std::string::String>,
    }
    impl ::std::default::Default for PaymentIdentification71 {
        fn default() -> Self {
            Self {
                clearing_system_reference: Ok(Default::default()),
                end_to_end_identification: Err(
                    "no value supplied for end_to_end_identification".to_string(),
                ),
                instruction_identification: Ok(Default::default()),
                transaction_identification: Ok(Default::default()),
                uetr: Err("no value supplied for uetr".to_string()),
            }
        }
    }
    impl PaymentIdentification71 {
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
        pub fn end_to_end_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.end_to_end_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for end_to_end_identification: {e}"
                    )
                });
            self
        }
        pub fn instruction_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.instruction_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for instruction_identification: {e}"
                    )
                });
            self
        }
        pub fn transaction_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.transaction_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for transaction_identification: {e}"
                    )
                });
            self
        }
        pub fn uetr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UuiDv4Identifier>,
            T::Error: ::std::fmt::Display,
        {
            self.uetr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uetr: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PaymentIdentification71>
    for super::PaymentIdentification71 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaymentIdentification71,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                clearing_system_reference: value.clearing_system_reference?,
                end_to_end_identification: value.end_to_end_identification?,
                instruction_identification: value.instruction_identification?,
                transaction_identification: value.transaction_identification?,
                uetr: value.uetr?,
            })
        }
    }
    impl ::std::convert::From<super::PaymentIdentification71>
    for PaymentIdentification71 {
        fn from(value: super::PaymentIdentification71) -> Self {
            Self {
                clearing_system_reference: Ok(value.clearing_system_reference),
                end_to_end_identification: Ok(value.end_to_end_identification),
                instruction_identification: Ok(value.instruction_identification),
                transaction_identification: Ok(value.transaction_identification),
                uetr: Ok(value.uetr),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PaymentTypeInformation281 {
        category_purpose: ::std::result::Result<
            ::std::option::Option<super::CategoryPurpose1Choice>,
            ::std::string::String,
        >,
        local_instrument: ::std::result::Result<
            super::LocalInstrument2Choice1,
            ::std::string::String,
        >,
        service_level: ::std::result::Result<
            ::std::vec::Vec<super::ServiceLevel8Choice>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PaymentTypeInformation281 {
        fn default() -> Self {
            Self {
                category_purpose: Ok(Default::default()),
                local_instrument: Err(
                    "no value supplied for local_instrument".to_string(),
                ),
                service_level: Ok(Default::default()),
            }
        }
    }
    impl PaymentTypeInformation281 {
        pub fn category_purpose<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CategoryPurpose1Choice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.category_purpose = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for category_purpose: {e}")
                });
            self
        }
        pub fn local_instrument<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LocalInstrument2Choice1>,
            T::Error: ::std::fmt::Display,
        {
            self.local_instrument = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for local_instrument: {e}")
                });
            self
        }
        pub fn service_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ServiceLevel8Choice>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_level = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for service_level: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PaymentTypeInformation281>
    for super::PaymentTypeInformation281 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PaymentTypeInformation281,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                category_purpose: value.category_purpose?,
                local_instrument: value.local_instrument?,
                service_level: value.service_level?,
            })
        }
    }
    impl ::std::convert::From<super::PaymentTypeInformation281>
    for PaymentTypeInformation281 {
        fn from(value: super::PaymentTypeInformation281) -> Self {
            Self {
                category_purpose: Ok(value.category_purpose),
                local_instrument: Ok(value.local_instrument),
                service_level: Ok(value.service_level),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PersonIdentification131 {
        date_and_place_of_birth: ::std::result::Result<
            ::std::option::Option<super::DateAndPlaceOfBirth1>,
            ::std::string::String,
        >,
        other: ::std::result::Result<
            ::std::vec::Vec<super::GenericPersonIdentification1>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PersonIdentification131 {
        fn default() -> Self {
            Self {
                date_and_place_of_birth: Ok(Default::default()),
                other: Ok(Default::default()),
            }
        }
    }
    impl PersonIdentification131 {
        pub fn date_and_place_of_birth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DateAndPlaceOfBirth1>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.date_and_place_of_birth = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for date_and_place_of_birth: {e}"
                    )
                });
            self
        }
        pub fn other<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::GenericPersonIdentification1>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.other = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for other: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PersonIdentification131>
    for super::PersonIdentification131 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PersonIdentification131,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                date_and_place_of_birth: value.date_and_place_of_birth?,
                other: value.other?,
            })
        }
    }
    impl ::std::convert::From<super::PersonIdentification131>
    for PersonIdentification131 {
        fn from(value: super::PersonIdentification131) -> Self {
            Self {
                date_and_place_of_birth: Ok(value.date_and_place_of_birth),
                other: Ok(value.other),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostalAddress241 {
        address_line: ::std::result::Result<
            ::std::vec::Vec<super::Max70Text>,
            ::std::string::String,
        >,
        building_name: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        building_number: ::std::result::Result<
            ::std::option::Option<super::Max16Text>,
            ::std::string::String,
        >,
        country: ::std::result::Result<
            ::std::option::Option<super::CountryCode>,
            ::std::string::String,
        >,
        country_sub_division: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        department: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        district_name: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        floor: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        post_box: ::std::result::Result<
            ::std::option::Option<super::Max16Text>,
            ::std::string::String,
        >,
        post_code: ::std::result::Result<
            ::std::option::Option<super::Max16Text>,
            ::std::string::String,
        >,
        room: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        street_name: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        sub_department: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        town_location_name: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        town_name: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PostalAddress241 {
        fn default() -> Self {
            Self {
                address_line: Ok(Default::default()),
                building_name: Ok(Default::default()),
                building_number: Ok(Default::default()),
                country: Ok(Default::default()),
                country_sub_division: Ok(Default::default()),
                department: Ok(Default::default()),
                district_name: Ok(Default::default()),
                floor: Ok(Default::default()),
                post_box: Ok(Default::default()),
                post_code: Ok(Default::default()),
                room: Ok(Default::default()),
                street_name: Ok(Default::default()),
                sub_department: Ok(Default::default()),
                town_location_name: Ok(Default::default()),
                town_name: Ok(Default::default()),
            }
        }
    }
    impl PostalAddress241 {
        pub fn address_line<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.address_line = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for address_line: {e}")
                });
            self
        }
        pub fn building_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.building_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for building_name: {e}")
                });
            self
        }
        pub fn building_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max16Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.building_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for building_number: {e}")
                });
            self
        }
        pub fn country<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CountryCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.country = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country: {e}")
                });
            self
        }
        pub fn country_sub_division<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_sub_division = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for country_sub_division: {e}"
                    )
                });
            self
        }
        pub fn department<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.department = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for department: {e}")
                });
            self
        }
        pub fn district_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.district_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for district_name: {e}")
                });
            self
        }
        pub fn floor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.floor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for floor: {e}"));
            self
        }
        pub fn post_box<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max16Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.post_box = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for post_box: {e}")
                });
            self
        }
        pub fn post_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max16Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.post_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for post_code: {e}")
                });
            self
        }
        pub fn room<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.room = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for room: {e}"));
            self
        }
        pub fn street_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.street_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for street_name: {e}")
                });
            self
        }
        pub fn sub_department<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.sub_department = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sub_department: {e}")
                });
            self
        }
        pub fn town_location_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.town_location_name = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for town_location_name: {e}"
                    )
                });
            self
        }
        pub fn town_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.town_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for town_name: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PostalAddress241> for super::PostalAddress241 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostalAddress241,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                address_line: value.address_line?,
                building_name: value.building_name?,
                building_number: value.building_number?,
                country: value.country?,
                country_sub_division: value.country_sub_division?,
                department: value.department?,
                district_name: value.district_name?,
                floor: value.floor?,
                post_box: value.post_box?,
                post_code: value.post_code?,
                room: value.room?,
                street_name: value.street_name?,
                sub_department: value.sub_department?,
                town_location_name: value.town_location_name?,
                town_name: value.town_name?,
            })
        }
    }
    impl ::std::convert::From<super::PostalAddress241> for PostalAddress241 {
        fn from(value: super::PostalAddress241) -> Self {
            Self {
                address_line: Ok(value.address_line),
                building_name: Ok(value.building_name),
                building_number: Ok(value.building_number),
                country: Ok(value.country),
                country_sub_division: Ok(value.country_sub_division),
                department: Ok(value.department),
                district_name: Ok(value.district_name),
                floor: Ok(value.floor),
                post_box: Ok(value.post_box),
                post_code: Ok(value.post_code),
                room: Ok(value.room),
                street_name: Ok(value.street_name),
                sub_department: Ok(value.sub_department),
                town_location_name: Ok(value.town_location_name),
                town_name: Ok(value.town_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PostalAddress242 {
        address_line: ::std::result::Result<
            ::std::vec::Vec<super::Max70Text>,
            ::std::string::String,
        >,
        building_name: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        building_number: ::std::result::Result<
            ::std::option::Option<super::Max16Text>,
            ::std::string::String,
        >,
        country: ::std::result::Result<super::CountryCode, ::std::string::String>,
        country_sub_division: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        department: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        district_name: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        floor: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        post_box: ::std::result::Result<
            ::std::option::Option<super::Max16Text>,
            ::std::string::String,
        >,
        post_code: ::std::result::Result<
            ::std::option::Option<super::Max16Text>,
            ::std::string::String,
        >,
        room: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        street_name: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        sub_department: ::std::result::Result<
            ::std::option::Option<super::Max70Text>,
            ::std::string::String,
        >,
        town_location_name: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        town_name: ::std::result::Result<super::Max35Text, ::std::string::String>,
    }
    impl ::std::default::Default for PostalAddress242 {
        fn default() -> Self {
            Self {
                address_line: Ok(Default::default()),
                building_name: Ok(Default::default()),
                building_number: Ok(Default::default()),
                country: Err("no value supplied for country".to_string()),
                country_sub_division: Ok(Default::default()),
                department: Ok(Default::default()),
                district_name: Ok(Default::default()),
                floor: Ok(Default::default()),
                post_box: Ok(Default::default()),
                post_code: Ok(Default::default()),
                room: Ok(Default::default()),
                street_name: Ok(Default::default()),
                sub_department: Ok(Default::default()),
                town_location_name: Ok(Default::default()),
                town_name: Err("no value supplied for town_name".to_string()),
            }
        }
    }
    impl PostalAddress242 {
        pub fn address_line<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.address_line = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for address_line: {e}")
                });
            self
        }
        pub fn building_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.building_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for building_name: {e}")
                });
            self
        }
        pub fn building_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max16Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.building_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for building_number: {e}")
                });
            self
        }
        pub fn country<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CountryCode>,
            T::Error: ::std::fmt::Display,
        {
            self.country = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country: {e}")
                });
            self
        }
        pub fn country_sub_division<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_sub_division = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for country_sub_division: {e}"
                    )
                });
            self
        }
        pub fn department<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.department = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for department: {e}")
                });
            self
        }
        pub fn district_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.district_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for district_name: {e}")
                });
            self
        }
        pub fn floor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.floor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for floor: {e}"));
            self
        }
        pub fn post_box<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max16Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.post_box = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for post_box: {e}")
                });
            self
        }
        pub fn post_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max16Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.post_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for post_code: {e}")
                });
            self
        }
        pub fn room<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.room = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for room: {e}"));
            self
        }
        pub fn street_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.street_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for street_name: {e}")
                });
            self
        }
        pub fn sub_department<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max70Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.sub_department = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sub_department: {e}")
                });
            self
        }
        pub fn town_location_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.town_location_name = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for town_location_name: {e}"
                    )
                });
            self
        }
        pub fn town_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.town_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for town_name: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PostalAddress242> for super::PostalAddress242 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PostalAddress242,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                address_line: value.address_line?,
                building_name: value.building_name?,
                building_number: value.building_number?,
                country: value.country?,
                country_sub_division: value.country_sub_division?,
                department: value.department?,
                district_name: value.district_name?,
                floor: value.floor?,
                post_box: value.post_box?,
                post_code: value.post_code?,
                room: value.room?,
                street_name: value.street_name?,
                sub_department: value.sub_department?,
                town_location_name: value.town_location_name?,
                town_name: value.town_name?,
            })
        }
    }
    impl ::std::convert::From<super::PostalAddress242> for PostalAddress242 {
        fn from(value: super::PostalAddress242) -> Self {
            Self {
                address_line: Ok(value.address_line),
                building_name: Ok(value.building_name),
                building_number: Ok(value.building_number),
                country: Ok(value.country),
                country_sub_division: Ok(value.country_sub_division),
                department: Ok(value.department),
                district_name: Ok(value.district_name),
                floor: Ok(value.floor),
                post_box: Ok(value.post_box),
                post_code: Ok(value.post_code),
                room: Ok(value.room),
                street_name: Ok(value.street_name),
                sub_department: Ok(value.sub_department),
                town_location_name: Ok(value.town_location_name),
                town_name: Ok(value.town_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProxyAccountIdentification1 {
        identification: ::std::result::Result<super::Max2048Text, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::ProxyAccountType1Choice>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProxyAccountIdentification1 {
        fn default() -> Self {
            Self {
                identification: Err("no value supplied for identification".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ProxyAccountIdentification1 {
        pub fn identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max2048Text>,
            T::Error: ::std::fmt::Display,
        {
            self.identification = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identification: {e}")
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ProxyAccountType1Choice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProxyAccountIdentification1>
    for super::ProxyAccountIdentification1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProxyAccountIdentification1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                identification: value.identification?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ProxyAccountIdentification1>
    for ProxyAccountIdentification1 {
        fn from(value: super::ProxyAccountIdentification1) -> Self {
            Self {
                identification: Ok(value.identification),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReferredDocumentInformation71 {
        line_details: ::std::result::Result<
            ::std::vec::Vec<super::DocumentLineInformation11>,
            ::std::string::String,
        >,
        number: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        related_date: ::std::result::Result<
            ::std::option::Option<super::IsoDate>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::ReferredDocumentType4>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ReferredDocumentInformation71 {
        fn default() -> Self {
            Self {
                line_details: Ok(Default::default()),
                number: Ok(Default::default()),
                related_date: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ReferredDocumentInformation71 {
        pub fn line_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::DocumentLineInformation11>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.line_details = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for line_details: {e}")
                });
            self
        }
        pub fn number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number: {e}"));
            self
        }
        pub fn related_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IsoDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.related_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for related_date: {e}")
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ReferredDocumentType4>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReferredDocumentInformation71>
    for super::ReferredDocumentInformation71 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReferredDocumentInformation71,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                line_details: value.line_details?,
                number: value.number?,
                related_date: value.related_date?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ReferredDocumentInformation71>
    for ReferredDocumentInformation71 {
        fn from(value: super::ReferredDocumentInformation71) -> Self {
            Self {
                line_details: Ok(value.line_details),
                number: Ok(value.number),
                related_date: Ok(value.related_date),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReferredDocumentType4 {
        code_or_proprietary: ::std::result::Result<
            super::ReferredDocumentType3Choice,
            ::std::string::String,
        >,
        issuer: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ReferredDocumentType4 {
        fn default() -> Self {
            Self {
                code_or_proprietary: Err(
                    "no value supplied for code_or_proprietary".to_string(),
                ),
                issuer: Ok(Default::default()),
            }
        }
    }
    impl ReferredDocumentType4 {
        pub fn code_or_proprietary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReferredDocumentType3Choice>,
            T::Error: ::std::fmt::Display,
        {
            self.code_or_proprietary = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for code_or_proprietary: {e}"
                    )
                });
            self
        }
        pub fn issuer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.issuer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issuer: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReferredDocumentType4>
    for super::ReferredDocumentType4 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReferredDocumentType4,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_or_proprietary: value.code_or_proprietary?,
                issuer: value.issuer?,
            })
        }
    }
    impl ::std::convert::From<super::ReferredDocumentType4> for ReferredDocumentType4 {
        fn from(value: super::ReferredDocumentType4) -> Self {
            Self {
                code_or_proprietary: Ok(value.code_or_proprietary),
                issuer: Ok(value.issuer),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RegulatoryAuthority2 {
        country: ::std::result::Result<
            ::std::option::Option<super::CountryCode>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RegulatoryAuthority2 {
        fn default() -> Self {
            Self {
                country: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl RegulatoryAuthority2 {
        pub fn country<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CountryCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.country = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country: {e}")
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RegulatoryAuthority2> for super::RegulatoryAuthority2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RegulatoryAuthority2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                country: value.country?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::RegulatoryAuthority2> for RegulatoryAuthority2 {
        fn from(value: super::RegulatoryAuthority2) -> Self {
            Self {
                country: Ok(value.country),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RegulatoryReporting3 {
        authority: ::std::result::Result<
            ::std::option::Option<super::RegulatoryAuthority2>,
            ::std::string::String,
        >,
        debit_credit_reporting_indicator: ::std::result::Result<
            ::std::option::Option<super::RegulatoryReportingType1Code>,
            ::std::string::String,
        >,
        details: ::std::result::Result<
            ::std::vec::Vec<super::StructuredRegulatoryReporting3>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RegulatoryReporting3 {
        fn default() -> Self {
            Self {
                authority: Ok(Default::default()),
                debit_credit_reporting_indicator: Ok(Default::default()),
                details: Ok(Default::default()),
            }
        }
    }
    impl RegulatoryReporting3 {
        pub fn authority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RegulatoryAuthority2>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.authority = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for authority: {e}")
                });
            self
        }
        pub fn debit_credit_reporting_indicator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RegulatoryReportingType1Code>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.debit_credit_reporting_indicator = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for debit_credit_reporting_indicator: {e}"
                    )
                });
            self
        }
        pub fn details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::StructuredRegulatoryReporting3>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.details = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for details: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RegulatoryReporting3> for super::RegulatoryReporting3 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RegulatoryReporting3,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authority: value.authority?,
                debit_credit_reporting_indicator: value
                    .debit_credit_reporting_indicator?,
                details: value.details?,
            })
        }
    }
    impl ::std::convert::From<super::RegulatoryReporting3> for RegulatoryReporting3 {
        fn from(value: super::RegulatoryReporting3) -> Self {
            Self {
                authority: Ok(value.authority),
                debit_credit_reporting_indicator: Ok(
                    value.debit_credit_reporting_indicator,
                ),
                details: Ok(value.details),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RemittanceAmount2 {
        adjustment_amount_and_reason: ::std::result::Result<
            ::std::vec::Vec<super::DocumentAdjustment1>,
            ::std::string::String,
        >,
        credit_note_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        discount_applied_amount: ::std::result::Result<
            ::std::vec::Vec<super::DiscountAmountAndType1>,
            ::std::string::String,
        >,
        due_payable_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        remitted_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        tax_amount: ::std::result::Result<
            ::std::vec::Vec<super::TaxAmountAndType1>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RemittanceAmount2 {
        fn default() -> Self {
            Self {
                adjustment_amount_and_reason: Ok(Default::default()),
                credit_note_amount: Ok(Default::default()),
                discount_applied_amount: Ok(Default::default()),
                due_payable_amount: Ok(Default::default()),
                remitted_amount: Ok(Default::default()),
                tax_amount: Ok(Default::default()),
            }
        }
    }
    impl RemittanceAmount2 {
        pub fn adjustment_amount_and_reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DocumentAdjustment1>>,
            T::Error: ::std::fmt::Display,
        {
            self.adjustment_amount_and_reason = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for adjustment_amount_and_reason: {e}"
                    )
                });
            self
        }
        pub fn credit_note_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.credit_note_amount = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credit_note_amount: {e}"
                    )
                });
            self
        }
        pub fn discount_applied_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DiscountAmountAndType1>>,
            T::Error: ::std::fmt::Display,
        {
            self.discount_applied_amount = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for discount_applied_amount: {e}"
                    )
                });
            self
        }
        pub fn due_payable_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.due_payable_amount = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for due_payable_amount: {e}"
                    )
                });
            self
        }
        pub fn remitted_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.remitted_amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for remitted_amount: {e}")
                });
            self
        }
        pub fn tax_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TaxAmountAndType1>>,
            T::Error: ::std::fmt::Display,
        {
            self.tax_amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tax_amount: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RemittanceAmount2> for super::RemittanceAmount2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RemittanceAmount2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                adjustment_amount_and_reason: value.adjustment_amount_and_reason?,
                credit_note_amount: value.credit_note_amount?,
                discount_applied_amount: value.discount_applied_amount?,
                due_payable_amount: value.due_payable_amount?,
                remitted_amount: value.remitted_amount?,
                tax_amount: value.tax_amount?,
            })
        }
    }
    impl ::std::convert::From<super::RemittanceAmount2> for RemittanceAmount2 {
        fn from(value: super::RemittanceAmount2) -> Self {
            Self {
                adjustment_amount_and_reason: Ok(value.adjustment_amount_and_reason),
                credit_note_amount: Ok(value.credit_note_amount),
                discount_applied_amount: Ok(value.discount_applied_amount),
                due_payable_amount: Ok(value.due_payable_amount),
                remitted_amount: Ok(value.remitted_amount),
                tax_amount: Ok(value.tax_amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RemittanceAmount3 {
        adjustment_amount_and_reason: ::std::result::Result<
            ::std::vec::Vec<super::DocumentAdjustment1>,
            ::std::string::String,
        >,
        credit_note_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        discount_applied_amount: ::std::result::Result<
            ::std::vec::Vec<super::DiscountAmountAndType1>,
            ::std::string::String,
        >,
        due_payable_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        remitted_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        tax_amount: ::std::result::Result<
            ::std::vec::Vec<super::TaxAmountAndType1>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RemittanceAmount3 {
        fn default() -> Self {
            Self {
                adjustment_amount_and_reason: Ok(Default::default()),
                credit_note_amount: Ok(Default::default()),
                discount_applied_amount: Ok(Default::default()),
                due_payable_amount: Ok(Default::default()),
                remitted_amount: Ok(Default::default()),
                tax_amount: Ok(Default::default()),
            }
        }
    }
    impl RemittanceAmount3 {
        pub fn adjustment_amount_and_reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DocumentAdjustment1>>,
            T::Error: ::std::fmt::Display,
        {
            self.adjustment_amount_and_reason = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for adjustment_amount_and_reason: {e}"
                    )
                });
            self
        }
        pub fn credit_note_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.credit_note_amount = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for credit_note_amount: {e}"
                    )
                });
            self
        }
        pub fn discount_applied_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DiscountAmountAndType1>>,
            T::Error: ::std::fmt::Display,
        {
            self.discount_applied_amount = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for discount_applied_amount: {e}"
                    )
                });
            self
        }
        pub fn due_payable_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.due_payable_amount = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for due_payable_amount: {e}"
                    )
                });
            self
        }
        pub fn remitted_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.remitted_amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for remitted_amount: {e}")
                });
            self
        }
        pub fn tax_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TaxAmountAndType1>>,
            T::Error: ::std::fmt::Display,
        {
            self.tax_amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tax_amount: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RemittanceAmount3> for super::RemittanceAmount3 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RemittanceAmount3,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                adjustment_amount_and_reason: value.adjustment_amount_and_reason?,
                credit_note_amount: value.credit_note_amount?,
                discount_applied_amount: value.discount_applied_amount?,
                due_payable_amount: value.due_payable_amount?,
                remitted_amount: value.remitted_amount?,
                tax_amount: value.tax_amount?,
            })
        }
    }
    impl ::std::convert::From<super::RemittanceAmount3> for RemittanceAmount3 {
        fn from(value: super::RemittanceAmount3) -> Self {
            Self {
                adjustment_amount_and_reason: Ok(value.adjustment_amount_and_reason),
                credit_note_amount: Ok(value.credit_note_amount),
                discount_applied_amount: Ok(value.discount_applied_amount),
                due_payable_amount: Ok(value.due_payable_amount),
                remitted_amount: Ok(value.remitted_amount),
                tax_amount: Ok(value.tax_amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RemittanceInformation161 {
        structured: ::std::result::Result<
            ::std::vec::Vec<super::StructuredRemittanceInformation161>,
            ::std::string::String,
        >,
        unstructured: ::std::result::Result<
            ::std::vec::Vec<super::Max140Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RemittanceInformation161 {
        fn default() -> Self {
            Self {
                structured: Ok(Default::default()),
                unstructured: Ok(Default::default()),
            }
        }
    }
    impl RemittanceInformation161 {
        pub fn structured<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::StructuredRemittanceInformation161>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.structured = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for structured: {e}")
                });
            self
        }
        pub fn unstructured<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.unstructured = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for unstructured: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RemittanceInformation161>
    for super::RemittanceInformation161 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RemittanceInformation161,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                structured: value.structured?,
                unstructured: value.unstructured?,
            })
        }
    }
    impl ::std::convert::From<super::RemittanceInformation161>
    for RemittanceInformation161 {
        fn from(value: super::RemittanceInformation161) -> Self {
            Self {
                structured: Ok(value.structured),
                unstructured: Ok(value.unstructured),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RemittanceLocation71 {
        remittance_identification: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        remittance_location_details: ::std::result::Result<
            ::std::vec::Vec<super::RemittanceLocationData11>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RemittanceLocation71 {
        fn default() -> Self {
            Self {
                remittance_identification: Ok(Default::default()),
                remittance_location_details: Ok(Default::default()),
            }
        }
    }
    impl RemittanceLocation71 {
        pub fn remittance_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.remittance_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for remittance_identification: {e}"
                    )
                });
            self
        }
        pub fn remittance_location_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RemittanceLocationData11>>,
            T::Error: ::std::fmt::Display,
        {
            self.remittance_location_details = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for remittance_location_details: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RemittanceLocation71> for super::RemittanceLocation71 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RemittanceLocation71,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                remittance_identification: value.remittance_identification?,
                remittance_location_details: value.remittance_location_details?,
            })
        }
    }
    impl ::std::convert::From<super::RemittanceLocation71> for RemittanceLocation71 {
        fn from(value: super::RemittanceLocation71) -> Self {
            Self {
                remittance_identification: Ok(value.remittance_identification),
                remittance_location_details: Ok(value.remittance_location_details),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RemittanceLocationData11 {
        electronic_address: ::std::result::Result<
            ::std::option::Option<super::Max2048Text>,
            ::std::string::String,
        >,
        method: ::std::result::Result<
            super::RemittanceLocationMethod2Code,
            ::std::string::String,
        >,
        postal_address: ::std::result::Result<
            ::std::option::Option<super::NameAndAddress161>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RemittanceLocationData11 {
        fn default() -> Self {
            Self {
                electronic_address: Ok(Default::default()),
                method: Err("no value supplied for method".to_string()),
                postal_address: Ok(Default::default()),
            }
        }
    }
    impl RemittanceLocationData11 {
        pub fn electronic_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max2048Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.electronic_address = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for electronic_address: {e}"
                    )
                });
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RemittanceLocationMethod2Code>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn postal_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NameAndAddress161>>,
            T::Error: ::std::fmt::Display,
        {
            self.postal_address = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for postal_address: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RemittanceLocationData11>
    for super::RemittanceLocationData11 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RemittanceLocationData11,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                electronic_address: value.electronic_address?,
                method: value.method?,
                postal_address: value.postal_address?,
            })
        }
    }
    impl ::std::convert::From<super::RemittanceLocationData11>
    for RemittanceLocationData11 {
        fn from(value: super::RemittanceLocationData11) -> Self {
            Self {
                electronic_address: Ok(value.electronic_address),
                method: Ok(value.method),
                postal_address: Ok(value.postal_address),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SettlementInstruction71 {
        clearing_system: ::std::result::Result<
            super::ClearingSystemIdentification3Choice1,
            ::std::string::String,
        >,
        settlement_method: ::std::result::Result<
            super::SettlementMethod1Code1,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SettlementInstruction71 {
        fn default() -> Self {
            Self {
                clearing_system: Err(
                    "no value supplied for clearing_system".to_string(),
                ),
                settlement_method: Err(
                    "no value supplied for settlement_method".to_string(),
                ),
            }
        }
    }
    impl SettlementInstruction71 {
        pub fn clearing_system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ClearingSystemIdentification3Choice1>,
            T::Error: ::std::fmt::Display,
        {
            self.clearing_system = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for clearing_system: {e}")
                });
            self
        }
        pub fn settlement_method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SettlementMethod1Code1>,
            T::Error: ::std::fmt::Display,
        {
            self.settlement_method = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for settlement_method: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SettlementInstruction71>
    for super::SettlementInstruction71 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SettlementInstruction71,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                clearing_system: value.clearing_system?,
                settlement_method: value.settlement_method?,
            })
        }
    }
    impl ::std::convert::From<super::SettlementInstruction71>
    for SettlementInstruction71 {
        fn from(value: super::SettlementInstruction71) -> Self {
            Self {
                clearing_system: Ok(value.clearing_system),
                settlement_method: Ok(value.settlement_method),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StructuredRegulatoryReporting3 {
        amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        code: ::std::result::Result<
            ::std::option::Option<super::Max10Text>,
            ::std::string::String,
        >,
        country: ::std::result::Result<
            ::std::option::Option<super::CountryCode>,
            ::std::string::String,
        >,
        date: ::std::result::Result<
            ::std::option::Option<super::IsoDate>,
            ::std::string::String,
        >,
        information: ::std::result::Result<
            ::std::vec::Vec<super::Max35Text>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StructuredRegulatoryReporting3 {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                code: Ok(Default::default()),
                country: Ok(Default::default()),
                date: Ok(Default::default()),
                information: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl StructuredRegulatoryReporting3 {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max10Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn country<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CountryCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.country = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country: {e}")
                });
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IsoDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {e}"));
            self
        }
        pub fn information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.information = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for information: {e}")
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StructuredRegulatoryReporting3>
    for super::StructuredRegulatoryReporting3 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StructuredRegulatoryReporting3,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                code: value.code?,
                country: value.country?,
                date: value.date?,
                information: value.information?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::StructuredRegulatoryReporting3>
    for StructuredRegulatoryReporting3 {
        fn from(value: super::StructuredRegulatoryReporting3) -> Self {
            Self {
                amount: Ok(value.amount),
                code: Ok(value.code),
                country: Ok(value.country),
                date: Ok(value.date),
                information: Ok(value.information),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StructuredRemittanceInformation161 {
        additional_remittance_information: ::std::result::Result<
            ::std::vec::Vec<super::Max140Text>,
            ::std::string::String,
        >,
        creditor_reference_information: ::std::result::Result<
            ::std::option::Option<super::CreditorReferenceInformation2>,
            ::std::string::String,
        >,
        garnishment_remittance: ::std::result::Result<
            ::std::option::Option<super::Garnishment31>,
            ::std::string::String,
        >,
        invoicee: ::std::result::Result<
            ::std::option::Option<super::PartyIdentification1354>,
            ::std::string::String,
        >,
        invoicer: ::std::result::Result<
            ::std::option::Option<super::PartyIdentification1354>,
            ::std::string::String,
        >,
        referred_document_amount: ::std::result::Result<
            ::std::option::Option<super::RemittanceAmount2>,
            ::std::string::String,
        >,
        referred_document_information: ::std::result::Result<
            ::std::vec::Vec<super::ReferredDocumentInformation71>,
            ::std::string::String,
        >,
        tax_remittance: ::std::result::Result<
            ::std::option::Option<super::TaxInformation7>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StructuredRemittanceInformation161 {
        fn default() -> Self {
            Self {
                additional_remittance_information: Ok(Default::default()),
                creditor_reference_information: Ok(Default::default()),
                garnishment_remittance: Ok(Default::default()),
                invoicee: Ok(Default::default()),
                invoicer: Ok(Default::default()),
                referred_document_amount: Ok(Default::default()),
                referred_document_information: Ok(Default::default()),
                tax_remittance: Ok(Default::default()),
            }
        }
    }
    impl StructuredRemittanceInformation161 {
        pub fn additional_remittance_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.additional_remittance_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for additional_remittance_information: {e}"
                    )
                });
            self
        }
        pub fn creditor_reference_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreditorReferenceInformation2>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.creditor_reference_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for creditor_reference_information: {e}"
                    )
                });
            self
        }
        pub fn garnishment_remittance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Garnishment31>>,
            T::Error: ::std::fmt::Display,
        {
            self.garnishment_remittance = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for garnishment_remittance: {e}"
                    )
                });
            self
        }
        pub fn invoicee<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PartyIdentification1354>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.invoicee = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for invoicee: {e}")
                });
            self
        }
        pub fn invoicer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PartyIdentification1354>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.invoicer = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for invoicer: {e}")
                });
            self
        }
        pub fn referred_document_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RemittanceAmount2>>,
            T::Error: ::std::fmt::Display,
        {
            self.referred_document_amount = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for referred_document_amount: {e}"
                    )
                });
            self
        }
        pub fn referred_document_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::ReferredDocumentInformation71>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.referred_document_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for referred_document_information: {e}"
                    )
                });
            self
        }
        pub fn tax_remittance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaxInformation7>>,
            T::Error: ::std::fmt::Display,
        {
            self.tax_remittance = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tax_remittance: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<StructuredRemittanceInformation161>
    for super::StructuredRemittanceInformation161 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StructuredRemittanceInformation161,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_remittance_information: value
                    .additional_remittance_information?,
                creditor_reference_information: value.creditor_reference_information?,
                garnishment_remittance: value.garnishment_remittance?,
                invoicee: value.invoicee?,
                invoicer: value.invoicer?,
                referred_document_amount: value.referred_document_amount?,
                referred_document_information: value.referred_document_information?,
                tax_remittance: value.tax_remittance?,
            })
        }
    }
    impl ::std::convert::From<super::StructuredRemittanceInformation161>
    for StructuredRemittanceInformation161 {
        fn from(value: super::StructuredRemittanceInformation161) -> Self {
            Self {
                additional_remittance_information: Ok(
                    value.additional_remittance_information,
                ),
                creditor_reference_information: Ok(value.creditor_reference_information),
                garnishment_remittance: Ok(value.garnishment_remittance),
                invoicee: Ok(value.invoicee),
                invoicer: Ok(value.invoicer),
                referred_document_amount: Ok(value.referred_document_amount),
                referred_document_information: Ok(value.referred_document_information),
                tax_remittance: Ok(value.tax_remittance),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaxAmount2 {
        details: ::std::result::Result<
            ::std::vec::Vec<super::TaxRecordDetails2>,
            ::std::string::String,
        >,
        rate: ::std::result::Result<
            ::std::option::Option<super::PercentageRate>,
            ::std::string::String,
        >,
        taxable_base_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        total_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaxAmount2 {
        fn default() -> Self {
            Self {
                details: Ok(Default::default()),
                rate: Ok(Default::default()),
                taxable_base_amount: Ok(Default::default()),
                total_amount: Ok(Default::default()),
            }
        }
    }
    impl TaxAmount2 {
        pub fn details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TaxRecordDetails2>>,
            T::Error: ::std::fmt::Display,
        {
            self.details = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for details: {e}")
                });
            self
        }
        pub fn rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PercentageRate>>,
            T::Error: ::std::fmt::Display,
        {
            self.rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate: {e}"));
            self
        }
        pub fn taxable_base_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.taxable_base_amount = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for taxable_base_amount: {e}"
                    )
                });
            self
        }
        pub fn total_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.total_amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for total_amount: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TaxAmount2> for super::TaxAmount2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaxAmount2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                details: value.details?,
                rate: value.rate?,
                taxable_base_amount: value.taxable_base_amount?,
                total_amount: value.total_amount?,
            })
        }
    }
    impl ::std::convert::From<super::TaxAmount2> for TaxAmount2 {
        fn from(value: super::TaxAmount2) -> Self {
            Self {
                details: Ok(value.details),
                rate: Ok(value.rate),
                taxable_base_amount: Ok(value.taxable_base_amount),
                total_amount: Ok(value.total_amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaxAmountAndType1 {
        amount: ::std::result::Result<
            super::ActiveOrHistoricCurrencyAndAmount,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::TaxAmountType1Choice>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaxAmountAndType1 {
        fn default() -> Self {
            Self {
                amount: Err("no value supplied for amount".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl TaxAmountAndType1 {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActiveOrHistoricCurrencyAndAmount>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TaxAmountType1Choice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TaxAmountAndType1> for super::TaxAmountAndType1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaxAmountAndType1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TaxAmountAndType1> for TaxAmountAndType1 {
        fn from(value: super::TaxAmountAndType1) -> Self {
            Self {
                amount: Ok(value.amount),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaxAuthorisation1 {
        name: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaxAuthorisation1 {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl TaxAuthorisation1 {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TaxAuthorisation1> for super::TaxAuthorisation1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaxAuthorisation1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::TaxAuthorisation1> for TaxAuthorisation1 {
        fn from(value: super::TaxAuthorisation1) -> Self {
            Self {
                name: Ok(value.name),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaxInformation7 {
        administration_zone: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        creditor: ::std::result::Result<
            ::std::option::Option<super::TaxParty1>,
            ::std::string::String,
        >,
        date: ::std::result::Result<
            ::std::option::Option<super::IsoDate>,
            ::std::string::String,
        >,
        debtor: ::std::result::Result<
            ::std::option::Option<super::TaxParty2>,
            ::std::string::String,
        >,
        method: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        record: ::std::result::Result<
            ::std::vec::Vec<super::TaxRecord2>,
            ::std::string::String,
        >,
        reference_number: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        sequence_number: ::std::result::Result<
            ::std::option::Option<super::Number>,
            ::std::string::String,
        >,
        total_tax_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        total_taxable_base_amount: ::std::result::Result<
            ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            ::std::string::String,
        >,
        ultimate_debtor: ::std::result::Result<
            ::std::option::Option<super::TaxParty2>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaxInformation7 {
        fn default() -> Self {
            Self {
                administration_zone: Ok(Default::default()),
                creditor: Ok(Default::default()),
                date: Ok(Default::default()),
                debtor: Ok(Default::default()),
                method: Ok(Default::default()),
                record: Ok(Default::default()),
                reference_number: Ok(Default::default()),
                sequence_number: Ok(Default::default()),
                total_tax_amount: Ok(Default::default()),
                total_taxable_base_amount: Ok(Default::default()),
                ultimate_debtor: Ok(Default::default()),
            }
        }
    }
    impl TaxInformation7 {
        pub fn administration_zone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.administration_zone = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for administration_zone: {e}"
                    )
                });
            self
        }
        pub fn creditor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaxParty1>>,
            T::Error: ::std::fmt::Display,
        {
            self.creditor = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for creditor: {e}")
                });
            self
        }
        pub fn date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IsoDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date: {e}"));
            self
        }
        pub fn debtor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaxParty2>>,
            T::Error: ::std::fmt::Display,
        {
            self.debtor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for debtor: {e}"));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
            self
        }
        pub fn record<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TaxRecord2>>,
            T::Error: ::std::fmt::Display,
        {
            self.record = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for record: {e}"));
            self
        }
        pub fn reference_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for reference_number: {e}")
                });
            self
        }
        pub fn sequence_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Number>>,
            T::Error: ::std::fmt::Display,
        {
            self.sequence_number = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sequence_number: {e}")
                });
            self
        }
        pub fn total_tax_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.total_tax_amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for total_tax_amount: {e}")
                });
            self
        }
        pub fn total_taxable_base_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ActiveOrHistoricCurrencyAndAmount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.total_taxable_base_amount = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for total_taxable_base_amount: {e}"
                    )
                });
            self
        }
        pub fn ultimate_debtor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaxParty2>>,
            T::Error: ::std::fmt::Display,
        {
            self.ultimate_debtor = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ultimate_debtor: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TaxInformation7> for super::TaxInformation7 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaxInformation7,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                administration_zone: value.administration_zone?,
                creditor: value.creditor?,
                date: value.date?,
                debtor: value.debtor?,
                method: value.method?,
                record: value.record?,
                reference_number: value.reference_number?,
                sequence_number: value.sequence_number?,
                total_tax_amount: value.total_tax_amount?,
                total_taxable_base_amount: value.total_taxable_base_amount?,
                ultimate_debtor: value.ultimate_debtor?,
            })
        }
    }
    impl ::std::convert::From<super::TaxInformation7> for TaxInformation7 {
        fn from(value: super::TaxInformation7) -> Self {
            Self {
                administration_zone: Ok(value.administration_zone),
                creditor: Ok(value.creditor),
                date: Ok(value.date),
                debtor: Ok(value.debtor),
                method: Ok(value.method),
                record: Ok(value.record),
                reference_number: Ok(value.reference_number),
                sequence_number: Ok(value.sequence_number),
                total_tax_amount: Ok(value.total_tax_amount),
                total_taxable_base_amount: Ok(value.total_taxable_base_amount),
                ultimate_debtor: Ok(value.ultimate_debtor),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaxParty1 {
        registration_identification: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        tax_identification: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        tax_type: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaxParty1 {
        fn default() -> Self {
            Self {
                registration_identification: Ok(Default::default()),
                tax_identification: Ok(Default::default()),
                tax_type: Ok(Default::default()),
            }
        }
    }
    impl TaxParty1 {
        pub fn registration_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.registration_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for registration_identification: {e}"
                    )
                });
            self
        }
        pub fn tax_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.tax_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for tax_identification: {e}"
                    )
                });
            self
        }
        pub fn tax_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.tax_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tax_type: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TaxParty1> for super::TaxParty1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaxParty1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                registration_identification: value.registration_identification?,
                tax_identification: value.tax_identification?,
                tax_type: value.tax_type?,
            })
        }
    }
    impl ::std::convert::From<super::TaxParty1> for TaxParty1 {
        fn from(value: super::TaxParty1) -> Self {
            Self {
                registration_identification: Ok(value.registration_identification),
                tax_identification: Ok(value.tax_identification),
                tax_type: Ok(value.tax_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaxParty2 {
        authorisation: ::std::result::Result<
            ::std::option::Option<super::TaxAuthorisation1>,
            ::std::string::String,
        >,
        registration_identification: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        tax_identification: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        tax_type: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaxParty2 {
        fn default() -> Self {
            Self {
                authorisation: Ok(Default::default()),
                registration_identification: Ok(Default::default()),
                tax_identification: Ok(Default::default()),
                tax_type: Ok(Default::default()),
            }
        }
    }
    impl TaxParty2 {
        pub fn authorisation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaxAuthorisation1>>,
            T::Error: ::std::fmt::Display,
        {
            self.authorisation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for authorisation: {e}")
                });
            self
        }
        pub fn registration_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.registration_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for registration_identification: {e}"
                    )
                });
            self
        }
        pub fn tax_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.tax_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for tax_identification: {e}"
                    )
                });
            self
        }
        pub fn tax_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.tax_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tax_type: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TaxParty2> for super::TaxParty2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaxParty2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authorisation: value.authorisation?,
                registration_identification: value.registration_identification?,
                tax_identification: value.tax_identification?,
                tax_type: value.tax_type?,
            })
        }
    }
    impl ::std::convert::From<super::TaxParty2> for TaxParty2 {
        fn from(value: super::TaxParty2) -> Self {
            Self {
                authorisation: Ok(value.authorisation),
                registration_identification: Ok(value.registration_identification),
                tax_identification: Ok(value.tax_identification),
                tax_type: Ok(value.tax_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaxPeriod2 {
        from_to_date: ::std::result::Result<
            ::std::option::Option<super::DatePeriod2>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::TaxRecordPeriod1Code>,
            ::std::string::String,
        >,
        year: ::std::result::Result<
            ::std::option::Option<super::IsoDate>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaxPeriod2 {
        fn default() -> Self {
            Self {
                from_to_date: Ok(Default::default()),
                type_: Ok(Default::default()),
                year: Ok(Default::default()),
            }
        }
    }
    impl TaxPeriod2 {
        pub fn from_to_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DatePeriod2>>,
            T::Error: ::std::fmt::Display,
        {
            self.from_to_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for from_to_date: {e}")
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TaxRecordPeriod1Code>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn year<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IsoDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.year = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for year: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TaxPeriod2> for super::TaxPeriod2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaxPeriod2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                from_to_date: value.from_to_date?,
                type_: value.type_?,
                year: value.year?,
            })
        }
    }
    impl ::std::convert::From<super::TaxPeriod2> for TaxPeriod2 {
        fn from(value: super::TaxPeriod2) -> Self {
            Self {
                from_to_date: Ok(value.from_to_date),
                type_: Ok(value.type_),
                year: Ok(value.year),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaxRecord2 {
        additional_information: ::std::result::Result<
            ::std::option::Option<super::Max140Text>,
            ::std::string::String,
        >,
        category: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        category_details: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        certificate_identification: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        debtor_status: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        forms_code: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
        period: ::std::result::Result<
            ::std::option::Option<super::TaxPeriod2>,
            ::std::string::String,
        >,
        tax_amount: ::std::result::Result<
            ::std::option::Option<super::TaxAmount2>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::Max35Text>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaxRecord2 {
        fn default() -> Self {
            Self {
                additional_information: Ok(Default::default()),
                category: Ok(Default::default()),
                category_details: Ok(Default::default()),
                certificate_identification: Ok(Default::default()),
                debtor_status: Ok(Default::default()),
                forms_code: Ok(Default::default()),
                period: Ok(Default::default()),
                tax_amount: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl TaxRecord2 {
        pub fn additional_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max140Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.additional_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for additional_information: {e}"
                    )
                });
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for category: {e}")
                });
            self
        }
        pub fn category_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.category_details = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for category_details: {e}")
                });
            self
        }
        pub fn certificate_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_identification = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for certificate_identification: {e}"
                    )
                });
            self
        }
        pub fn debtor_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.debtor_status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for debtor_status: {e}")
                });
            self
        }
        pub fn forms_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.forms_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for forms_code: {e}")
                });
            self
        }
        pub fn period<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaxPeriod2>>,
            T::Error: ::std::fmt::Display,
        {
            self.period = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for period: {e}"));
            self
        }
        pub fn tax_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaxAmount2>>,
            T::Error: ::std::fmt::Display,
        {
            self.tax_amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tax_amount: {e}")
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max35Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TaxRecord2> for super::TaxRecord2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaxRecord2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_information: value.additional_information?,
                category: value.category?,
                category_details: value.category_details?,
                certificate_identification: value.certificate_identification?,
                debtor_status: value.debtor_status?,
                forms_code: value.forms_code?,
                period: value.period?,
                tax_amount: value.tax_amount?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TaxRecord2> for TaxRecord2 {
        fn from(value: super::TaxRecord2) -> Self {
            Self {
                additional_information: Ok(value.additional_information),
                category: Ok(value.category),
                category_details: Ok(value.category_details),
                certificate_identification: Ok(value.certificate_identification),
                debtor_status: Ok(value.debtor_status),
                forms_code: Ok(value.forms_code),
                period: Ok(value.period),
                tax_amount: Ok(value.tax_amount),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaxRecordDetails2 {
        amount: ::std::result::Result<
            super::ActiveOrHistoricCurrencyAndAmount,
            ::std::string::String,
        >,
        period: ::std::result::Result<
            ::std::option::Option<super::TaxPeriod2>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaxRecordDetails2 {
        fn default() -> Self {
            Self {
                amount: Err("no value supplied for amount".to_string()),
                period: Ok(Default::default()),
            }
        }
    }
    impl TaxRecordDetails2 {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActiveOrHistoricCurrencyAndAmount>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn period<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaxPeriod2>>,
            T::Error: ::std::fmt::Display,
        {
            self.period = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for period: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TaxRecordDetails2> for super::TaxRecordDetails2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaxRecordDetails2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                period: value.period?,
            })
        }
    }
    impl ::std::convert::From<super::TaxRecordDetails2> for TaxRecordDetails2 {
        fn from(value: super::TaxRecordDetails2) -> Self {
            Self {
                amount: Ok(value.amount),
                period: Ok(value.period),
            }
        }
    }
}
