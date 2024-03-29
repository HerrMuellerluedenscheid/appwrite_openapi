/*
 * Appwrite
 *
 * Appwrite backend as a service cuts up to 70% of the time and costs required for building a modern application. We abstract and simplify common development tasks behind a REST APIs, to help you develop your app in a fast and secure way. For full API documentation and tutorials go to [https://appwrite.io/docs](https://appwrite.io/docs)
 *
 * The version of the OpenAPI document: 1.4.9
 * Contact: team@appwrite.io
 * Generated by: https://openapi-generator.tech
 */

/// ExecutionList : Executions List



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionList {
    /// Total number of executions documents that matched your query.
    #[serde(rename = "total")]
    pub total: i32,
    /// List of executions.
    #[serde(rename = "executions")]
    pub executions: Vec<crate::models::Execution>,
}

impl ExecutionList {
    /// Executions List
    pub fn new(total: i32, executions: Vec<crate::models::Execution>) -> ExecutionList {
        ExecutionList {
            total,
            executions,
        }
    }
}


