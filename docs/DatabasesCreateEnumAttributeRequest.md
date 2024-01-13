# DatabasesCreateEnumAttributeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | Attribute Key. | [default to null]
**elements** | **Vec<String>** | Array of elements in enumerated type. Uses length of longest element to determine size. Maximum of 100 elements are allowed, each 4096 characters long. | 
**required** | **bool** | Is attribute required? | [default to false]
**default** | Option<**String**> | Default value for attribute when not provided. Cannot be set when attribute is required. | [optional][default to null]
**array** | Option<**bool**> | Is attribute an array? | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


