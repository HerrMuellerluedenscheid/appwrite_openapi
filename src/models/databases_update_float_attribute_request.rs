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
pub struct DatabasesUpdateFloatAttributeRequest {
    /// Is attribute required?
    #[serde(rename = "required")]
    pub required: bool,
    /// Minimum value to enforce on new documents
    #[serde(rename = "min")]
    pub min: f32,
    /// Maximum value to enforce on new documents
    #[serde(rename = "max")]
    pub max: f32,
    /// Default value for attribute when not provided. Cannot be set when attribute is required.
    #[serde(rename = "default", deserialize_with = "Option::deserialize")]
    pub default: Option<f32>,
}

impl DatabasesUpdateFloatAttributeRequest {
    pub fn new(required: bool, min: f32, max: f32, default: Option<f32>) -> DatabasesUpdateFloatAttributeRequest {
        DatabasesUpdateFloatAttributeRequest {
            required,
            min,
            max,
            default,
        }
    }
}


