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
pub struct FunctionsUpdateVariableRequest {
    /// Variable key. Max length: 255 chars.
    #[serde(rename = "key")]
    pub key: String,
    /// Variable value. Max length: 8192 chars.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl FunctionsUpdateVariableRequest {
    pub fn new(key: String) -> FunctionsUpdateVariableRequest {
        FunctionsUpdateVariableRequest {
            key,
            value: None,
        }
    }
}


