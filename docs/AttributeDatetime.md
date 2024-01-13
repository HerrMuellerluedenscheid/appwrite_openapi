# AttributeDatetime

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | Attribute Key. | 
**r#type** | **String** | Attribute type. | 
**status** | **String** | Attribute status. Possible values: `available`, `processing`, `deleting`, `stuck`, or `failed` | 
**error** | **String** | Error message. Displays error generated on failure of creating or deleting an attribute. | 
**required** | **bool** | Is attribute required? | 
**array** | Option<**bool**> | Is attribute an array? | [optional]
**format** | **String** | ISO 8601 format. | 
**default** | Option<**String**> | Default value for attribute when not provided. Only null is optional | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


