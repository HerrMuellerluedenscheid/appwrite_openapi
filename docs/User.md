# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dollar_id** | **String** | User ID. | 
**dollar_created_at** | **String** | User creation date in ISO 8601 format. | 
**dollar_updated_at** | **String** | User update date in ISO 8601 format. | 
**name** | **String** | User name. | 
**password** | Option<**String**> | Hashed user password. | [optional]
**hash** | Option<**String**> | Password hashing algorithm. | [optional]
**hash_options** | Option<[**serde_json::Value**](.md)> | Password hashing algorithm configuration. | [optional]
**registration** | **String** | User registration date in ISO 8601 format. | 
**status** | **bool** | User status. Pass `true` for enabled and `false` for disabled. | 
**labels** | **Vec<String>** | Labels for the user. | 
**password_update** | **String** | Password update time in ISO 8601 format. | 
**email** | **String** | User email address. | 
**phone** | **String** | User phone number in E.164 format. | 
**email_verification** | **bool** | Email verification status. | 
**phone_verification** | **bool** | Phone verification status. | 
**prefs** | [**serde_json::Value**](.md) | User preferences as a key-value object | 
**accessed_at** | **String** | Most recent access date in ISO 8601 format. This attribute is only updated again after 24 hours. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


