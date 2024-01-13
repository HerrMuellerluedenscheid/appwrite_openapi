# UsersCreateScryptUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | User ID. Choose a custom ID or generate a random ID with `ID.unique()`. Valid chars are a-z, A-Z, 0-9, period, hyphen, and underscore. Can't start with a special char. Max length is 36 chars. | [default to null]
**email** | **String** | User email. | [default to null]
**password** | **String** | User password hashed using Scrypt. | [default to null]
**password_salt** | **String** | Optional salt used to hash password. | [default to null]
**password_cpu** | **i32** | Optional CPU cost used to hash password. | 
**password_memory** | **i32** | Optional memory cost used to hash password. | 
**password_parallel** | **i32** | Optional parallelization cost used to hash password. | 
**password_length** | **i32** | Optional hash length used to hash password. | 
**name** | Option<**String**> | User name. Max length: 128 chars. | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


