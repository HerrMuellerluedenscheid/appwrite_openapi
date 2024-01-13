/*
 * Appwrite
 *
 * Appwrite backend as a service cuts up to 70% of the time and costs required for building a modern application. We abstract and simplify common development tasks behind a REST APIs, to help you develop your app in a fast and secure way. For full API documentation and tutorials go to [https://appwrite.io/docs](https://appwrite.io/docs)
 *
 * The version of the OpenAPI document: 1.4.9
 * Contact: team@appwrite.io
 * Generated by: https://openapi-generator.tech
 */

/// Index : Index



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Index {
    /// Index Key.
    #[serde(rename = "key")]
    pub key: String,
    /// Index type.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Index status. Possible values: `available`, `processing`, `deleting`, `stuck`, or `failed`
    #[serde(rename = "status")]
    pub status: String,
    /// Error message. Displays error generated on failure of creating or deleting an index.
    #[serde(rename = "error")]
    pub error: String,
    /// Index attributes.
    #[serde(rename = "attributes")]
    pub attributes: Vec<String>,
    /// Index orders.
    #[serde(rename = "orders", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Option<Vec<String>>>,
}

impl Index {
    /// Index
    pub fn new(key: String, r#type: String, status: String, error: String, attributes: Vec<String>) -> Index {
        Index {
            key,
            r#type,
            status,
            error,
            attributes,
            orders: None,
        }
    }
}

