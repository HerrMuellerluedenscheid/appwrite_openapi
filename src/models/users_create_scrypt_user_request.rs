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
pub struct UsersCreateScryptUserRequest {
    /// User ID. Choose a custom ID or generate a random ID with `ID.unique()`. Valid chars are a-z, A-Z, 0-9, period, hyphen, and underscore. Can't start with a special char. Max length is 36 chars.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// User email.
    #[serde(rename = "email")]
    pub email: String,
    /// User password hashed using Scrypt.
    #[serde(rename = "password")]
    pub password: String,
    /// Optional salt used to hash password.
    #[serde(rename = "passwordSalt")]
    pub password_salt: String,
    /// Optional CPU cost used to hash password.
    #[serde(rename = "passwordCpu")]
    pub password_cpu: i32,
    /// Optional memory cost used to hash password.
    #[serde(rename = "passwordMemory")]
    pub password_memory: i32,
    /// Optional parallelization cost used to hash password.
    #[serde(rename = "passwordParallel")]
    pub password_parallel: i32,
    /// Optional hash length used to hash password.
    #[serde(rename = "passwordLength")]
    pub password_length: i32,
    /// User name. Max length: 128 chars.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UsersCreateScryptUserRequest {
    pub fn new(user_id: String, email: String, password: String, password_salt: String, password_cpu: i32, password_memory: i32, password_parallel: i32, password_length: i32) -> UsersCreateScryptUserRequest {
        UsersCreateScryptUserRequest {
            user_id,
            email,
            password,
            password_salt,
            password_cpu,
            password_memory,
            password_parallel,
            password_length,
            name: None,
        }
    }
}


