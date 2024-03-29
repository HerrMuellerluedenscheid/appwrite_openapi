/*
 * Appwrite
 *
 * Appwrite backend as a service cuts up to 70% of the time and costs required for building a modern application. We abstract and simplify common development tasks behind a REST APIs, to help you develop your app in a fast and secure way. For full API documentation and tutorials go to [https://appwrite.io/docs](https://appwrite.io/docs)
 *
 * The version of the OpenAPI document: 1.4.9
 * Contact: team@appwrite.io
 * Generated by: https://openapi-generator.tech
 */

/// RuntimeList : Runtimes List



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeList {
    /// Total number of runtimes documents that matched your query.
    #[serde(rename = "total")]
    pub total: i32,
    /// List of runtimes.
    #[serde(rename = "runtimes")]
    pub runtimes: Vec<crate::models::Runtime>,
}

impl RuntimeList {
    /// Runtimes List
    pub fn new(total: i32, runtimes: Vec<crate::models::Runtime>) -> RuntimeList {
        RuntimeList {
            total,
            runtimes,
        }
    }
}


