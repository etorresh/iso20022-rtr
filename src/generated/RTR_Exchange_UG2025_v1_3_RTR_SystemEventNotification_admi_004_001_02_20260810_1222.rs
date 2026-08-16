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
pub struct Event21 {
    ///Proprietary code used to specify an event that occurred in a system.
    pub event_code: Max4AlphaNumericTextFixed,
    ///Free text used to describe an event which occurred in a system.
    pub event_description: Max350Text,
}
impl Event21 {
    pub fn builder() -> builder::Event21 {
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
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
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
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
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
    PartialOrd
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
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "HBRT" => Ok(Self::Hbrt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Max4AlphaNumericTextFixed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
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
/**Scope
The SystemEventNotification message is sent by a central system to notify the occurrence of an event in a central system.
Usage
The message can be used by a central settlement system to inform its participants of an event that is going to occur in the system, for instance that the system will be down at a certain time, etc.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Scope\r\nThe SystemEventNotification message is sent by a central system to notify the occurrence of an event in a central system.\r\nUsage\r\nThe message can be used by a central settlement system to inform its participants of an event that is going to occur in the system, for instance that the system will be down at a certain time, etc.",
///  "type": "object",
///  "required": [
///    "event_information"
///  ],
///  "properties": {
///    "event_information": {
///      "description": "Detailed information about a system event.",
///      "$ref": "#/definitions/Event2__1"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SystemEventNotificationV02 {
    ///Detailed information about a system event.
    pub event_information: Event21,
}
impl SystemEventNotificationV02 {
    pub fn builder() -> builder::SystemEventNotificationV02 {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Event21 {
        event_code: ::std::result::Result<
            super::Max4AlphaNumericTextFixed,
            ::std::string::String,
        >,
        event_description: ::std::result::Result<
            super::Max350Text,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Event21 {
        fn default() -> Self {
            Self {
                event_code: Err("no value supplied for event_code".to_string()),
                event_description: Err(
                    "no value supplied for event_description".to_string(),
                ),
            }
        }
    }
    impl Event21 {
        pub fn event_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max4AlphaNumericTextFixed>,
            T::Error: ::std::fmt::Display,
        {
            self.event_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event_code: {e}")
                });
            self
        }
        pub fn event_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max350Text>,
            T::Error: ::std::fmt::Display,
        {
            self.event_description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event_description: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Event21> for super::Event21 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Event21,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_code: value.event_code?,
                event_description: value.event_description?,
            })
        }
    }
    impl ::std::convert::From<super::Event21> for Event21 {
        fn from(value: super::Event21) -> Self {
            Self {
                event_code: Ok(value.event_code),
                event_description: Ok(value.event_description),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SystemEventNotificationV02 {
        event_information: ::std::result::Result<super::Event21, ::std::string::String>,
    }
    impl ::std::default::Default for SystemEventNotificationV02 {
        fn default() -> Self {
            Self {
                event_information: Err(
                    "no value supplied for event_information".to_string(),
                ),
            }
        }
    }
    impl SystemEventNotificationV02 {
        pub fn event_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Event21>,
            T::Error: ::std::fmt::Display,
        {
            self.event_information = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for event_information: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SystemEventNotificationV02>
    for super::SystemEventNotificationV02 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SystemEventNotificationV02,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_information: value.event_information?,
            })
        }
    }
    impl ::std::convert::From<super::SystemEventNotificationV02>
    for SystemEventNotificationV02 {
        fn from(value: super::SystemEventNotificationV02) -> Self {
            Self {
                event_information: Ok(value.event_information),
            }
        }
    }
}
