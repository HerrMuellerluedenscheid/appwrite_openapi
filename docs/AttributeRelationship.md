# AttributeRelationship

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | Attribute Key. | 
**r#type** | **String** | Attribute type. | 
**status** | **String** | Attribute status. Possible values: `available`, `processing`, `deleting`, `stuck`, or `failed` | 
**error** | **String** | Error message. Displays error generated on failure of creating or deleting an attribute. | 
**required** | **bool** | Is attribute required? | 
**array** | Option<**bool**> | Is attribute an array? | [optional]
**related_collection** | **String** | The ID of the related collection. | 
**relation_type** | **String** | The type of the relationship. | 
**two_way** | **bool** | Is the relationship two-way? | 
**two_way_key** | **String** | The key of the two-way relationship. | 
**on_delete** | **String** | How deleting the parent document will propagate to child documents. | 
**side** | **String** | Whether this is the parent or child side of the relationship | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


