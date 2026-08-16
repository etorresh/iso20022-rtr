/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
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
///Provides information on an event that happened in a system.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provides information on an event that happened in a system.",
///  "type": "object",
///  "required": [
///    "event_code",
///    "event_description"
///  ],
///  "properties": {
///    "event_code": {
///      "description": "Proprietary code used to specify an event that occurred in a system.",
///      "$ref": "#/definitions/Max4AlphaNumericText_fixed"
///    },
///    "event_description": {
///      "description": "Free text used to describe an event which occurred in a system.",
///      "$ref": "#/definitions/Max350Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Event11 {
    ///Proprietary code used to specify an event that occurred in a system.
    pub event_code: Max4AlphaNumericTextFixed,
    ///Free text used to describe an event which occurred in a system.
    pub event_description: Max350Text,
}
impl Event11 {
    pub fn builder() -> builder::Event11 {
        Default::default()
    }
}
///Specifies a character string with a maximum length of 350 characters.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a character string with a maximum length of 350 characters.",
///  "type": "string",
///  "maxLength": 350,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Max350Text(::std::string::String);
impl ::std::ops::Deref for Max350Text {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Max350Text> for ::std::string::String {
    fn from(value: Max350Text) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Max350Text {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 350usize {
            return Err("longer than 350 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Max350Text {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max350Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max350Text {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Max350Text {
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
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
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
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
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
/**
*`HBRT`-null*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "\n*`HBRT`-null",
///  "type": "string",
///  "enum": [
///    "HBRT"
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
    PartialOrd,
)]
pub enum Max4AlphaNumericTextFixed {
    #[serde(rename = "HBRT")]
    Hbrt,
}
impl ::std::fmt::Display for Max4AlphaNumericTextFixed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Hbrt => f.write_str("HBRT"),
        }
    }
}
impl ::std::str::FromStr for Max4AlphaNumericTextFixed {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "HBRT" => Ok(Self::Hbrt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Max4AlphaNumericTextFixed {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Max4AlphaNumericTextFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Max4AlphaNumericTextFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**The SystemEventAcknowledgement message is sent by a participant of a central system to the central system to acknowledge the notification of an occurrence of an event in a central system.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The SystemEventAcknowledgement message is sent by a participant of a central system to the central system to acknowledge the notification of an occurrence of an event in a central system.\r\n",
///  "type": "object",
///  "required": [
///    "acknowledgement_details",
///    "message_identification"
///  ],
///  "properties": {
///    "acknowledgement_details": {
///      "description": "Details of the system event being acknowledged.",
///      "$ref": "#/definitions/Event1__1"
///    },
///    "message_identification": {
///      "description": "Unique and unambiguous identifier for the message, as assigned by the sender.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SystemEventAcknowledgementV01 {
    ///Details of the system event being acknowledged.
    pub acknowledgement_details: Event11,
    ///Unique and unambiguous identifier for the message, as assigned by the sender.
    pub message_identification: Max35Text,
}
impl SystemEventAcknowledgementV01 {
    pub fn builder() -> builder::SystemEventAcknowledgementV01 {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Event11 {
        event_code: ::std::result::Result<super::Max4AlphaNumericTextFixed, ::std::string::String>,
        event_description: ::std::result::Result<super::Max350Text, ::std::string::String>,
    }
    impl ::std::default::Default for Event11 {
        fn default() -> Self {
            Self {
                event_code: Err("no value supplied for event_code".to_string()),
                event_description: Err("no value supplied for event_description".to_string()),
            }
        }
    }
    impl Event11 {
        pub fn event_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max4AlphaNumericTextFixed>,
            T::Error: ::std::fmt::Display,
        {
            self.event_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_code: {e}"));
            self
        }
        pub fn event_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max350Text>,
            T::Error: ::std::fmt::Display,
        {
            self.event_description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_description: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Event11> for super::Event11 {
        type Error = super::error::ConversionError;
        fn try_from(value: Event11) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_code: value.event_code?,
                event_description: value.event_description?,
            })
        }
    }
    impl ::std::convert::From<super::Event11> for Event11 {
        fn from(value: super::Event11) -> Self {
            Self {
                event_code: Ok(value.event_code),
                event_description: Ok(value.event_description),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SystemEventAcknowledgementV01 {
        acknowledgement_details: ::std::result::Result<super::Event11, ::std::string::String>,
        message_identification: ::std::result::Result<super::Max35Text, ::std::string::String>,
    }
    impl ::std::default::Default for SystemEventAcknowledgementV01 {
        fn default() -> Self {
            Self {
                acknowledgement_details: Err(
                    "no value supplied for acknowledgement_details".to_string()
                ),
                message_identification: Err(
                    "no value supplied for message_identification".to_string()
                ),
            }
        }
    }
    impl SystemEventAcknowledgementV01 {
        pub fn acknowledgement_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Event11>,
            T::Error: ::std::fmt::Display,
        {
            self.acknowledgement_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for acknowledgement_details: {e}")
            });
            self
        }
        pub fn message_identification<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.message_identification = value.try_into().map_err(|e| {
                format!("error converting supplied value for message_identification: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<SystemEventAcknowledgementV01>
        for super::SystemEventAcknowledgementV01
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SystemEventAcknowledgementV01,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acknowledgement_details: value.acknowledgement_details?,
                message_identification: value.message_identification?,
            })
        }
    }
    impl ::std::convert::From<super::SystemEventAcknowledgementV01> for SystemEventAcknowledgementV01 {
        fn from(value: super::SystemEventAcknowledgementV01) -> Self {
            Self {
                acknowledgement_details: Ok(value.acknowledgement_details),
                message_identification: Ok(value.message_identification),
            }
        }
    }
}
