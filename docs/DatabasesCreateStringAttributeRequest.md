# DatabasesCreateStringAttributeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | Attribute Key. | [default to null]
**size** | **i32** | Attribute size for text attributes, in number of characters. | 
**required** | **bool** | Is attribute required? | [default to false]
**default** | Option<**String**> | Default value for attribute when not provided. Cannot be set when attribute is required. | [optional][default to null]
**array** | Option<**bool**> | Is attribute an array? | [optional][default to false]
**encrypt** | Option<**bool**> | Toggle encryption for the attribute. Encryption enhances security by not storing any plain text values in the database. However, encrypted attributes cannot be queried. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


