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
///Message reference of relevance to the present message.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Message reference of relevance to the present message.",
///  "type": "object",
///  "required": [
///    "reference"
///  ],
///  "properties": {
///    "reference": {
///      "description": "Business reference of the present message assigned by the party issuing the message. This reference must be unique amongst all messages of the same name sent by the same party.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageReference {
    ///Business reference of the present message assigned by the party issuing the message. This reference must be unique amongst all messages of the same name sent by the same party.
    pub reference: Max35Text,
}
impl MessageReference {
    pub fn builder() -> builder::MessageReference {
        Default::default()
    }
}
/**Scope
The MessageReject message is sent by a central system to notify the rejection of a previously received message.
Usage
The message provides specific information about the rejection reason.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Scope\r\nThe MessageReject message is sent by a central system to notify the rejection of a previously received message.\r\nUsage\r\nThe message provides specific information about the rejection reason.",
///  "type": "object",
///  "required": [
///    "reason",
///    "related_reference"
///  ],
///  "properties": {
///    "reason": {
///      "description": "General information about the reason of the message rejection.",
///      "$ref": "#/definitions/RejectionReason2__1"
///    },
///    "related_reference": {
///      "description": "Refers to the identification of the message previously received and for which the rejection is notified.",
///      "$ref": "#/definitions/MessageReference"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MessageRejectV01 {
    ///General information about the reason of the message rejection.
    pub reason: RejectionReason21,
    ///Refers to the identification of the message previously received and for which the rejection is notified.
    pub related_reference: MessageReference,
}
impl MessageRejectV01 {
    pub fn builder() -> builder::MessageRejectV01 {
        Default::default()
    }
}
///General information about the reason of the rejection.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "General information about the reason of the rejection.",
///  "type": "object",
///  "required": [
///    "rejecting_party_reason"
///  ],
///  "properties": {
///    "reason_description": {
///      "description": "Detailed description of the rejection reason.",
///      "$ref": "#/definitions/Max350Text"
///    },
///    "rejecting_party_reason": {
///      "description": "Reason of the rejection provided by the rejecting party.",
///      "$ref": "#/definitions/Max35Text"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RejectionReason21 {
    ///Detailed description of the rejection reason.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason_description: ::std::option::Option<Max350Text>,
    ///Reason of the rejection provided by the rejecting party.
    pub rejecting_party_reason: Max35Text,
}
impl RejectionReason21 {
    pub fn builder() -> builder::RejectionReason21 {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct MessageReference {
        reference: ::std::result::Result<super::Max35Text, ::std::string::String>,
    }
    impl ::std::default::Default for MessageReference {
        fn default() -> Self {
            Self {
                reference: Err("no value supplied for reference".to_string()),
            }
        }
    }
    impl MessageReference {
        pub fn reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.reference = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for reference: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<MessageReference> for super::MessageReference {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageReference,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reference: value.reference?,
            })
        }
    }
    impl ::std::convert::From<super::MessageReference> for MessageReference {
        fn from(value: super::MessageReference) -> Self {
            Self {
                reference: Ok(value.reference),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageRejectV01 {
        reason: ::std::result::Result<super::RejectionReason21, ::std::string::String>,
        related_reference: ::std::result::Result<
            super::MessageReference,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageRejectV01 {
        fn default() -> Self {
            Self {
                reason: Err("no value supplied for reason".to_string()),
                related_reference: Err(
                    "no value supplied for related_reference".to_string(),
                ),
            }
        }
    }
    impl MessageRejectV01 {
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RejectionReason21>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn related_reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageReference>,
            T::Error: ::std::fmt::Display,
        {
            self.related_reference = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for related_reference: {e}")
                });
            self
        }
    }
    impl ::std::convert::TryFrom<MessageRejectV01> for super::MessageRejectV01 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageRejectV01,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reason: value.reason?,
                related_reference: value.related_reference?,
            })
        }
    }
    impl ::std::convert::From<super::MessageRejectV01> for MessageRejectV01 {
        fn from(value: super::MessageRejectV01) -> Self {
            Self {
                reason: Ok(value.reason),
                related_reference: Ok(value.related_reference),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RejectionReason21 {
        reason_description: ::std::result::Result<
            ::std::option::Option<super::Max350Text>,
            ::std::string::String,
        >,
        rejecting_party_reason: ::std::result::Result<
            super::Max35Text,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RejectionReason21 {
        fn default() -> Self {
            Self {
                reason_description: Ok(Default::default()),
                rejecting_party_reason: Err(
                    "no value supplied for rejecting_party_reason".to_string(),
                ),
            }
        }
    }
    impl RejectionReason21 {
        pub fn reason_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Max350Text>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason_description = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for reason_description: {e}"
                    )
                });
            self
        }
        pub fn rejecting_party_reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Max35Text>,
            T::Error: ::std::fmt::Display,
        {
            self.rejecting_party_reason = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for rejecting_party_reason: {e}"
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RejectionReason21> for super::RejectionReason21 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RejectionReason21,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reason_description: value.reason_description?,
                rejecting_party_reason: value.rejecting_party_reason?,
            })
        }
    }
    impl ::std::convert::From<super::RejectionReason21> for RejectionReason21 {
        fn from(value: super::RejectionReason21) -> Self {
            Self {
                reason_description: Ok(value.reason_description),
                rejecting_party_reason: Ok(value.rejecting_party_reason),
            }
        }
    }
}
