/*
 * Appwrite
 *
 * Appwrite backend as a service cuts up to 70% of the time and costs required for building a modern application. We abstract and simplify common development tasks behind a REST APIs, to help you develop your app in a fast and secure way. For full API documentation and tutorials go to [https://appwrite.io/docs](https://appwrite.io/docs)
 *
 * The version of the OpenAPI document: 1.4.9
 * Contact: team@appwrite.io
 * Generated by: https://openapi-generator.tech
 */

/// AttributeDatetime : AttributeDatetime



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeDatetime {
    /// Attribute Key.
    #[serde(rename = "key")]
    pub key: String,
    /// Attribute type.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Attribute status. Possible values: `available`, `processing`, `deleting`, `stuck`, or `failed`
    #[serde(rename = "status")]
    pub status: String,
    /// Error message. Displays error generated on failure of creating or deleting an attribute.
    #[serde(rename = "error")]
    pub error: String,
    /// Is attribute required?
    #[serde(rename = "required")]
    pub required: bool,
    /// Is attribute an array?
    #[serde(rename = "array", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub array: Option<Option<bool>>,
    /// ISO 8601 format.
    #[serde(rename = "format")]
    pub format: String,
    /// Default value for attribute when not provided. Only null is optional
    #[serde(rename = "default", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default: Option<Option<String>>,
}

impl AttributeDatetime {
    /// AttributeDatetime
    pub fn new(key: String, r#type: String, status: String, error: String, required: bool, format: String) -> AttributeDatetime {
        AttributeDatetime {
            key,
            r#type,
            status,
            error,
            required,
            array: None,
            format,
            default: None,
        }
    }
}


