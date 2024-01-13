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
pub struct StorageUpdateBucketRequest {
    /// Bucket name
    #[serde(rename = "name")]
    pub name: String,
    /// An array of permission strings. By default, the current permissions are inherited. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// Enables configuring permissions for individual file. A user needs one of file or bucket level permissions to access a file. [Learn more about permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "fileSecurity", skip_serializing_if = "Option::is_none")]
    pub file_security: Option<bool>,
    /// Is bucket enabled? When set to 'disabled', users cannot access the files in this bucket but Server SDKs with and API key can still access the bucket. No files are lost when this is toggled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Maximum file size allowed in bytes. Maximum allowed value is 30MB.
    #[serde(rename = "maximumFileSize", skip_serializing_if = "Option::is_none")]
    pub maximum_file_size: Option<i32>,
    /// Allowed file extensions. Maximum of 100 extensions are allowed, each 64 characters long.
    #[serde(rename = "allowedFileExtensions", skip_serializing_if = "Option::is_none")]
    pub allowed_file_extensions: Option<Vec<String>>,
    /// Compression algorithm choosen for compression. Can be one of none, [gzip](https://en.wikipedia.org/wiki/Gzip), or [zstd](https://en.wikipedia.org/wiki/Zstd), For file size above 20MB compression is skipped even if it's enabled
    #[serde(rename = "compression", skip_serializing_if = "Option::is_none")]
    pub compression: Option<Compression>,
    /// Is encryption enabled? For file size above 20MB encryption is skipped even if it's enabled
    #[serde(rename = "encryption", skip_serializing_if = "Option::is_none")]
    pub encryption: Option<bool>,
    /// Is virus scanning enabled? For file size above 20MB AntiVirus scanning is skipped even if it's enabled
    #[serde(rename = "antivirus", skip_serializing_if = "Option::is_none")]
    pub antivirus: Option<bool>,
}

impl StorageUpdateBucketRequest {
    pub fn new(name: String) -> StorageUpdateBucketRequest {
        StorageUpdateBucketRequest {
            name,
            permissions: None,
            file_security: None,
            enabled: None,
            maximum_file_size: None,
            allowed_file_extensions: None,
            compression: None,
            encryption: None,
            antivirus: None,
        }
    }
}

/// Compression algorithm choosen for compression. Can be one of none, [gzip](https://en.wikipedia.org/wiki/Gzip), or [zstd](https://en.wikipedia.org/wiki/Zstd), For file size above 20MB compression is skipped even if it's enabled
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Compression {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "zstd")]
    Zstd,
}

impl Default for Compression {
    fn default() -> Compression {
        Self::None
    }
}
