# AttributeInteger

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | Attribute Key. | 
**r#type** | **String** | Attribute type. | 
**status** | **String** | Attribute status. Possible values: `available`, `processing`, `deleting`, `stuck`, or `failed` | 
**error** | **String** | Error message. Displays error generated on failure of creating or deleting an attribute. | 
**required** | **bool** | Is attribute required? | 
**array** | Option<**bool**> | Is attribute an array? | [optional]
**min** | Option<**i32**> | Minimum value to enforce for new documents. | [optional]
**max** | Option<**i32**> | Maximum value to enforce for new documents. | [optional]
**default** | Option<**i32**> | Default value for attribute when not provided. Cannot be set when attribute is required. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


