# FunctionsCreateExecutionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | Option<**String**> | HTTP body of execution. Default value is empty string. | [optional][default to ]
**r#async** | Option<**bool**> | Execute code in the background. Default value is false. | [optional][default to false]
**path** | Option<**String**> | HTTP path of execution. Path can include query params. Default value is / | [optional][default to /]
**method** | Option<**String**> | HTTP method of execution. Default value is GET. | [optional][default to Post]
**headers** | Option<[**serde_json::Value**](.md)> | HTTP headers of execution. Defaults to empty. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


