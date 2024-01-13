# UsersCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | User ID. Choose a custom ID or generate a random ID with `ID.unique()`. Valid chars are a-z, A-Z, 0-9, period, hyphen, and underscore. Can't start with a special char. Max length is 36 chars. | [default to null]
**email** | Option<**String**> | User email. | [optional][default to null]
**phone** | Option<**String**> | Phone number. Format this number with a leading '+' and a country code, e.g., +16175551212. | [optional][default to null]
**password** | Option<**String**> | Plain text user password. Must be at least 8 chars. | [optional][default to ]
**name** | Option<**String**> | User name. Max length: 128 chars. | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


