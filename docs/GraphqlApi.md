# \GraphqlApi

All URIs are relative to *https://HOSTNAME/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**graphql_mutation**](GraphqlApi.md#graphql_mutation) | **POST** /graphql/mutation | GraphQL endpoint
[**graphql_query**](GraphqlApi.md#graphql_query) | **POST** /graphql | GraphQL endpoint



## graphql_mutation

> ::std::collections::HashMap<String, serde_json::Value> graphql_mutation(payload)
GraphQL endpoint

Execute a GraphQL mutation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**GraphqlQueryRequest**](GraphqlQueryRequest.md)> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## graphql_query

> ::std::collections::HashMap<String, serde_json::Value> graphql_query(payload)
GraphQL endpoint

Execute a GraphQL mutation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**GraphqlQueryRequest**](GraphqlQueryRequest.md)> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

