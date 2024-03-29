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
pub struct FunctionsCreateExecutionRequest {
    /// HTTP body of execution. Default value is empty string.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Execute code in the background. Default value is false.
    #[serde(rename = "async", skip_serializing_if = "Option::is_none")]
    pub r#async: Option<bool>,
    /// HTTP path of execution. Path can include query params. Default value is /
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// HTTP method of execution. Default value is GET.
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
    /// HTTP headers of execution. Defaults to empty.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}

impl FunctionsCreateExecutionRequest {
    pub fn new() -> FunctionsCreateExecutionRequest {
        FunctionsCreateExecutionRequest {
            body: None,
            r#async: None,
            path: None,
            method: None,
            headers: None,
        }
    }
}

/// HTTP method of execution. Default value is GET.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "OPTIONS")]
    Options,
}

impl Default for Method {
    fn default() -> Method {
        Self::Get
    }
}

