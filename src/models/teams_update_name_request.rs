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
pub struct TeamsUpdateNameRequest {
    /// New team name. Max length: 128 chars.
    #[serde(rename = "name")]
    pub name: String,
}

impl TeamsUpdateNameRequest {
    pub fn new(name: String) -> TeamsUpdateNameRequest {
        TeamsUpdateNameRequest {
            name,
        }
    }
}


