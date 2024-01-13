# Execution

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dollar_id** | **String** | Execution ID. | 
**dollar_created_at** | **String** | Execution creation date in ISO 8601 format. | 
**dollar_updated_at** | **String** | Execution upate date in ISO 8601 format. | 
**dollar_permissions** | **Vec<String>** | Execution roles. | 
**function_id** | **String** | Function ID. | 
**trigger** | **String** | The trigger that caused the function to execute. Possible values can be: `http`, `schedule`, or `event`. | 
**status** | **String** | The status of the function execution. Possible values can be: `waiting`, `processing`, `completed`, or `failed`. | 
**request_method** | **String** | HTTP request method type. | 
**request_path** | **String** | HTTP request path and query. | 
**request_headers** | [**Vec<crate::models::Headers>**](headers.md) | HTTP response headers as a key-value object. This will return only whitelisted headers. All headers are returned if execution is created as synchronous. | 
**response_status_code** | **i32** | HTTP response status code. | 
**response_body** | **String** | HTTP response body. This will return empty unless execution is created as synchronous. | 
**response_headers** | [**Vec<crate::models::Headers>**](headers.md) | HTTP response headers as a key-value object. This will return only whitelisted headers. All headers are returned if execution is created as synchronous. | 
**logs** | **String** | Function logs. Includes the last 4,000 characters. This will return an empty string unless the response is returned using an API key or as part of a webhook payload. | 
**errors** | **String** | Function errors. Includes the last 4,000 characters. This will return an empty string unless the response is returned using an API key or as part of a webhook payload. | 
**duration** | **f64** | Function execution duration in seconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


