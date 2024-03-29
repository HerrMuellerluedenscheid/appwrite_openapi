/*
 * Appwrite
 *
 * Appwrite backend as a service cuts up to 70% of the time and costs required for building a modern application. We abstract and simplify common development tasks behind a REST APIs, to help you develop your app in a fast and secure way. For full API documentation and tutorials go to [https://appwrite.io/docs](https://appwrite.io/docs)
 *
 * The version of the OpenAPI document: 1.4.9
 * Contact: team@appwrite.io
 * Generated by: https://openapi-generator.tech
 */

/// DeploymentList : Deployments List



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentList {
    /// Total number of deployments documents that matched your query.
    #[serde(rename = "total")]
    pub total: i32,
    /// List of deployments.
    #[serde(rename = "deployments")]
    pub deployments: Vec<crate::models::Deployment>,
}

impl DeploymentList {
    /// Deployments List
    pub fn new(total: i32, deployments: Vec<crate::models::Deployment>) -> DeploymentList {
        DeploymentList {
            total,
            deployments,
        }
    }
}


