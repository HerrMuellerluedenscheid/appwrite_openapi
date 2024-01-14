**Auto-generated** client for Appwrite
======================================

This is an **auto-generated** client for Appwrite. It is generated using [OpenAPI Generator](https://openapi-generator.tech) with the following command:

```bash
openapi-generator generate -g rust -i https://raw.githubusercontent.com/appwrite/appwrite/main/app/config/specs/swagger2-latest-server.json -o . --skip-validate-spec
```

Check to generated [README.md](./README.md) for more information about the generated client.

Installation
------------

To use it in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
appwrite_openapi = "1.4.9"
```

Example Usage
-------------

This example will create a user, a database, a collection and an attribute when executing the tests:

```rust
use appwrite_openapi::{
    apis::{
        configuration::Configuration,
        databases_api::{
            databases_create, databases_create_collection, databases_create_string_attribute,
        },
        users_api::users_create,
    },
    models::DatabasesCreateStringAttributeRequest,
};
use reqwest::header;

const DATABASE_NAME: &str = "example";
const DATABASE_ID: &str = "5f9c5c6c5d5f7";
const COLLECTION_ID: &str = "5f9c5c6c5d5f8";
const API_KEY: &str = "PASTE YOUR API KEY HERE"; // should be read from environment variables instead.
const PROJECT_ID: &str = "PASTE YOUR PROJECT ID HERE"; // should be read from environment variables instead.

/// 
struct Appwrite {
    configuration: Configuration,
}

impl Appwrite {
    pub fn new() -> Self {

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Appwrite-key", API_KEY.to_string().parse().unwrap());
        headers.insert(
            "X-Appwrite-Project",
            header::HeaderValue::from_static(PROJECT_ID),
        );
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        let mut configuration = Configuration::new();
        configuration.base_path = "https://cloud.appwrite.io/v1".to_string();
        configuration.client = client;

        Appwrite { configuration }
    }
}

/// Create an example user. This will fail when the user already exists.
async fn create_user(appwrite: &Appwrite) {
    let user_create_request = appwrite_openapi::models::UsersCreateRequest {
        email: Some("example.user@gmail.com".to_string()),
        user_id: "23409usdfj0283u4".to_string(),
        phone: None,
        password: Some("dlijijsdifj0fdlj1230pisdf2384090pijsidfj".to_string()),
        name: Some("SomeRandomUser".to_string()),
    };

    let result = users_create(&appwrite.configuration, Some(user_create_request)).await;
    println!("{:?}", result.unwrap());
}

/// Create an example database. This will fail when the database already exists.
async fn create_database(appwrite: &Appwrite) {
    let database_create_request = appwrite_openapi::models::DatabasesCreateRequest {
        name: DATABASE_NAME.to_owned(),
        database_id: DATABASE_ID.to_owned(),
        enabled: Some(true),
    };

    let result = databases_create(&appwrite.configuration, Some(database_create_request)).await;
    println!("{:?}", result.unwrap());
}

/// Create an example collection. This will fail when the collection already exists.
async fn create_collection(appwrite: &Appwrite) {
    let request = appwrite_openapi::models::DatabasesCreateCollectionRequest {
        collection_id: COLLECTION_ID.to_owned(),
        name: "kitas".to_string(),
        permissions: None,
        document_security: None,
        enabled: Some(true),
    };

    let result =
        databases_create_collection(&appwrite.configuration, DATABASE_ID, Some(request)).await;
    println!("{:?}", result.unwrap());
}

/// Create an example attribute. This will fail when the attribute already exists.
async fn create_attributes(appwrite: &Appwrite) {
    let request = DatabasesCreateStringAttributeRequest {
        key: "name".to_string(),
        required: true,
        size: 10,
        default: None,
        array: None,
        encrypt: None,
    };

    let result = databases_create_string_attribute(
        &appwrite.configuration,
        DATABASE_ID,
        COLLECTION_ID,
        Some(request),
    )
    .await;
    println!("{:?}", result.unwrap());
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_example() {
        let appwrite = Appwrite::new();
        create_user(&appwrite).await;
        create_database(&appwrite).await;
        create_collection(&appwrite).await;
        create_attributes(&appwrite).await;
    }
}

```

