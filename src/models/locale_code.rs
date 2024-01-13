/*
 * Appwrite
 *
 * Appwrite backend as a service cuts up to 70% of the time and costs required for building a modern application. We abstract and simplify common development tasks behind a REST APIs, to help you develop your app in a fast and secure way. For full API documentation and tutorials go to [https://appwrite.io/docs](https://appwrite.io/docs)
 *
 * The version of the OpenAPI document: 1.4.9
 * Contact: team@appwrite.io
 * Generated by: https://openapi-generator.tech
 */

/// LocaleCode : LocaleCode



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocaleCode {
    /// Locale codes in [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes)
    #[serde(rename = "code")]
    pub code: String,
    /// Locale name
    #[serde(rename = "name")]
    pub name: String,
}

impl LocaleCode {
    /// LocaleCode
    pub fn new(code: String, name: String) -> LocaleCode {
        LocaleCode {
            code,
            name,
        }
    }
}


