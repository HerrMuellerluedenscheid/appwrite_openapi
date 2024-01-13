# Collection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dollar_id** | **String** | Collection ID. | 
**dollar_created_at** | **String** | Collection creation date in ISO 8601 format. | 
**dollar_updated_at** | **String** | Collection update date in ISO 8601 format. | 
**dollar_permissions** | **Vec<String>** | Collection permissions. [Learn more about permissions](https://appwrite.io/docs/permissions). | 
**database_id** | **String** | Database ID. | 
**name** | **String** | Collection name. | 
**enabled** | **bool** | Collection enabled. Can be 'enabled' or 'disabled'. When disabled, the collection is inaccessible to users, but remains accessible to Server SDKs using API keys. | 
**document_security** | **bool** | Whether document-level permissions are enabled. [Learn more about permissions](https://appwrite.io/docs/permissions). | 
**attributes** | [**Vec<serde_json::Value>**](serde_json::Value.md) | Collection attributes. | 
**indexes** | [**Vec<crate::models::Index>**](index.md) | Collection indexes. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


