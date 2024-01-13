# DatabasesCreateRelationshipAttributeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**related_collection_id** | **String** | Related Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [default to null]
**r#type** | **String** | Relation type | [default to null]
**two_way** | Option<**bool**> | Is Two Way? | [optional][default to false]
**key** | Option<**String**> | Attribute Key. | [optional][default to null]
**two_way_key** | Option<**String**> | Two Way Attribute Key. | [optional][default to null]
**on_delete** | Option<**String**> | Constraints option | [optional][default to Restrict]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


