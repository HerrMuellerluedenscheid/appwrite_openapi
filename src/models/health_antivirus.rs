/*
 * Appwrite
 *
 * Appwrite backend as a service cuts up to 70% of the time and costs required for building a modern application. We abstract and simplify common development tasks behind a REST APIs, to help you develop your app in a fast and secure way. For full API documentation and tutorials go to [https://appwrite.io/docs](https://appwrite.io/docs)
 *
 * The version of the OpenAPI document: 1.4.9
 * Contact: team@appwrite.io
 * Generated by: https://openapi-generator.tech
 */

/// HealthAntivirus : Health Antivirus



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAntivirus {
    /// Antivirus version.
    #[serde(rename = "version")]
    pub version: String,
    /// Antivirus status. Possible values can are: `disabled`, `offline`, `online`
    #[serde(rename = "status")]
    pub status: String,
}

impl HealthAntivirus {
    /// Health Antivirus
    pub fn new(version: String, status: String) -> HealthAntivirus {
        HealthAntivirus {
            version,
            status,
        }
    }
}


