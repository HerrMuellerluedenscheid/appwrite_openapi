/*
 * Appwrite
 *
 * Appwrite backend as a service cuts up to 70% of the time and costs required for building a modern application. We abstract and simplify common development tasks behind a REST APIs, to help you develop your app in a fast and secure way. For full API documentation and tutorials go to [https://appwrite.io/docs](https://appwrite.io/docs)
 *
 * The version of the OpenAPI document: 1.4.9
 * Contact: team@appwrite.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasesCreateDatetimeAttributeRequest {
    /// Attribute Key.
    #[serde(rename = "key")]
    pub key: String,
    /// Is attribute required?
    #[serde(rename = "required")]
    pub required: bool,
    /// Default value for the attribute in ISO 8601 format. Cannot be set when attribute is required.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Is attribute an array?
    #[serde(rename = "array", skip_serializing_if = "Option::is_none")]
    pub array: Option<bool>,
}

impl DatabasesCreateDatetimeAttributeRequest {
    pub fn new(key: String, required: bool) -> DatabasesCreateDatetimeAttributeRequest {
        DatabasesCreateDatetimeAttributeRequest {
            key,
            required,
            default: None,
            array: None,
        }
    }
}


