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
pub struct UsersCreateShaUserRequest {
    /// User ID. Choose a custom ID or generate a random ID with `ID.unique()`. Valid chars are a-z, A-Z, 0-9, period, hyphen, and underscore. Can't start with a special char. Max length is 36 chars.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// User email.
    #[serde(rename = "email")]
    pub email: String,
    /// User password hashed using SHA.
    #[serde(rename = "password")]
    pub password: String,
    /// Optional SHA version used to hash password. Allowed values are: 'sha1', 'sha224', 'sha256', 'sha384', 'sha512/224', 'sha512/256', 'sha512', 'sha3-224', 'sha3-256', 'sha3-384', 'sha3-512'
    #[serde(rename = "passwordVersion", skip_serializing_if = "Option::is_none")]
    pub password_version: Option<PasswordVersion>,
    /// User name. Max length: 128 chars.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UsersCreateShaUserRequest {
    pub fn new(user_id: String, email: String, password: String) -> UsersCreateShaUserRequest {
        UsersCreateShaUserRequest {
            user_id,
            email,
            password,
            password_version: None,
            name: None,
        }
    }
}

/// Optional SHA version used to hash password. Allowed values are: 'sha1', 'sha224', 'sha256', 'sha384', 'sha512/224', 'sha512/256', 'sha512', 'sha3-224', 'sha3-256', 'sha3-384', 'sha3-512'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PasswordVersion {
    #[serde(rename = "sha1")]
    Sha1,
    #[serde(rename = "sha224")]
    Sha224,
    #[serde(rename = "sha256")]
    Sha256,
    #[serde(rename = "sha384")]
    Sha384,
    #[serde(rename = "sha512/224")]
    Sha512Slash224,
    #[serde(rename = "sha512/256")]
    Sha512Slash256,
    #[serde(rename = "sha512")]
    Sha512,
    #[serde(rename = "sha3-224")]
    Sha3224,
    #[serde(rename = "sha3-256")]
    Sha3256,
    #[serde(rename = "sha3-384")]
    Sha3384,
    #[serde(rename = "sha3-512")]
    Sha3512,
}

impl Default for PasswordVersion {
    fn default() -> PasswordVersion {
        Self::Sha1
    }
}
