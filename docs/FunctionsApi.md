# \FunctionsApi

All URIs are relative to *https://HOSTNAME/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**functions_create**](FunctionsApi.md#functions_create) | **POST** /functions | Create function
[**functions_create_build**](FunctionsApi.md#functions_create_build) | **POST** /functions/{functionId}/deployments/{deploymentId}/builds/{buildId} | Create build
[**functions_create_deployment**](FunctionsApi.md#functions_create_deployment) | **POST** /functions/{functionId}/deployments | Create deployment
[**functions_create_execution**](FunctionsApi.md#functions_create_execution) | **POST** /functions/{functionId}/executions | Create execution
[**functions_create_variable**](FunctionsApi.md#functions_create_variable) | **POST** /functions/{functionId}/variables | Create variable
[**functions_delete**](FunctionsApi.md#functions_delete) | **DELETE** /functions/{functionId} | Delete function
[**functions_delete_deployment**](FunctionsApi.md#functions_delete_deployment) | **DELETE** /functions/{functionId}/deployments/{deploymentId} | Delete deployment
[**functions_delete_variable**](FunctionsApi.md#functions_delete_variable) | **DELETE** /functions/{functionId}/variables/{variableId} | Delete variable
[**functions_download_deployment**](FunctionsApi.md#functions_download_deployment) | **GET** /functions/{functionId}/deployments/{deploymentId}/download | Download Deployment
[**functions_get**](FunctionsApi.md#functions_get) | **GET** /functions/{functionId} | Get function
[**functions_get_deployment**](FunctionsApi.md#functions_get_deployment) | **GET** /functions/{functionId}/deployments/{deploymentId} | Get deployment
[**functions_get_execution**](FunctionsApi.md#functions_get_execution) | **GET** /functions/{functionId}/executions/{executionId} | Get execution
[**functions_get_variable**](FunctionsApi.md#functions_get_variable) | **GET** /functions/{functionId}/variables/{variableId} | Get variable
[**functions_list**](FunctionsApi.md#functions_list) | **GET** /functions | List functions
[**functions_list_deployments**](FunctionsApi.md#functions_list_deployments) | **GET** /functions/{functionId}/deployments | List deployments
[**functions_list_executions**](FunctionsApi.md#functions_list_executions) | **GET** /functions/{functionId}/executions | List executions
[**functions_list_runtimes**](FunctionsApi.md#functions_list_runtimes) | **GET** /functions/runtimes | List runtimes
[**functions_list_variables**](FunctionsApi.md#functions_list_variables) | **GET** /functions/{functionId}/variables | List variables
[**functions_update**](FunctionsApi.md#functions_update) | **PUT** /functions/{functionId} | Update function
[**functions_update_deployment**](FunctionsApi.md#functions_update_deployment) | **PATCH** /functions/{functionId}/deployments/{deploymentId} | Update function deployment
[**functions_update_variable**](FunctionsApi.md#functions_update_variable) | **PUT** /functions/{functionId}/variables/{variableId} | Update variable



## functions_create

> crate::models::Function functions_create(payload)
Create function

Create a new function. You can pass a list of [permissions](https://appwrite.io/docs/permissions) to allow different project users or team with access to execute the function using the client API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**FunctionsCreateRequest**](FunctionsCreateRequest.md)> |  |  |

### Return type

[**crate::models::Function**](function.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_create_build

> functions_create_build(function_id, deployment_id, build_id)
Create build

Create a new build for an Appwrite Function deployment. This endpoint can be used to retry a failed build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**deployment_id** | **String** | Deployment ID. | [required] |
**build_id** | **String** | Build unique ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_create_deployment

> crate::models::Deployment functions_create_deployment(function_id, code, activate, entrypoint, commands)
Create deployment

Create a new function code deployment. Use this endpoint to upload a new version of your code function. To execute your newly uploaded code, you'll need to update the function's deployment to use your new deployment UID.  This endpoint accepts a tar.gz file compressed with your code. Make sure to include any dependencies your code has within the compressed file. You can learn more about code packaging in the [Appwrite Cloud Functions tutorial](https://appwrite.io/docs/functions).  Use the \"command\" param to set the entrypoint used to execute your code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**code** | **std::path::PathBuf** | Gzip file with your code package. When used with the Appwrite CLI, pass the path to your code directory, and the CLI will automatically package your code. Use a path that is within the current directory. | [required] |
**activate** | **bool** | Automatically activate the deployment when it is finished building. | [required] |
**entrypoint** | Option<**String**> | Entrypoint File. |  |
**commands** | Option<**String**> | Build Commands. |  |

### Return type

[**crate::models::Deployment**](deployment.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_create_execution

> crate::models::Execution functions_create_execution(function_id, payload)
Create execution

Trigger a function execution. The returned object will return you the current execution status. You can ping the `Get Execution` endpoint to get updates on the current execution status. Once this endpoint is called, your function execution process will start asynchronously.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**payload** | Option<[**FunctionsCreateExecutionRequest**](FunctionsCreateExecutionRequest.md)> |  |  |

### Return type

[**crate::models::Execution**](execution.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_create_variable

> crate::models::Variable functions_create_variable(function_id, payload)
Create variable

Create a new function environment variable. These variables can be accessed in the function at runtime as environment variables.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function unique ID. | [required] |
**payload** | Option<[**FunctionsCreateVariableRequest**](FunctionsCreateVariableRequest.md)> |  |  |

### Return type

[**crate::models::Variable**](variable.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_delete

> functions_delete(function_id)
Delete function

Delete a function by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_delete_deployment

> functions_delete_deployment(function_id, deployment_id)
Delete deployment

Delete a code deployment by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**deployment_id** | **String** | Deployment ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_delete_variable

> functions_delete_variable(function_id, variable_id)
Delete variable

Delete a variable by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function unique ID. | [required] |
**variable_id** | **String** | Variable unique ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_download_deployment

> std::path::PathBuf functions_download_deployment(function_id, deployment_id)
Download Deployment

Get a Deployment's contents by its unique ID. This endpoint supports range requests for partial or streaming file download.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**deployment_id** | **String** | Deployment ID. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_get

> crate::models::Function functions_get(function_id)
Get function

Get a function by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |

### Return type

[**crate::models::Function**](function.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_get_deployment

> crate::models::Deployment functions_get_deployment(function_id, deployment_id)
Get deployment

Get a code deployment by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**deployment_id** | **String** | Deployment ID. | [required] |

### Return type

[**crate::models::Deployment**](deployment.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_get_execution

> crate::models::Execution functions_get_execution(function_id, execution_id)
Get execution

Get a function execution log by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**execution_id** | **String** | Execution ID. | [required] |

### Return type

[**crate::models::Execution**](execution.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_get_variable

> crate::models::Variable functions_get_variable(function_id, variable_id)
Get variable

Get a variable by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function unique ID. | [required] |
**variable_id** | **String** | Variable unique ID. | [required] |

### Return type

[**crate::models::Variable**](variable.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_list

> crate::models::FunctionList functions_list(queries, search)
List functions

Get a list of all the project's functions. You can use the query params to filter your results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: name, enabled, runtime, deployment, schedule, scheduleNext, schedulePrevious, timeout, entrypoint, commands, installationId |  |[default to []]
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::FunctionList**](functionList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_list_deployments

> crate::models::DeploymentList functions_list_deployments(function_id, queries, search)
List deployments

Get a list of all the project's code deployments. You can use the query params to filter your results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: size, buildId, activate, entrypoint, commands |  |[default to []]
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::DeploymentList**](deploymentList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_list_executions

> crate::models::ExecutionList functions_list_executions(function_id, queries, search)
List executions

Get a list of all the current user function execution logs. You can use the query params to filter your results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: trigger, status, responseStatusCode, duration |  |[default to []]
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::ExecutionList**](executionList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_list_runtimes

> crate::models::RuntimeList functions_list_runtimes()
List runtimes

Get a list of all runtimes that are currently active on your instance.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RuntimeList**](runtimeList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_list_variables

> crate::models::VariableList functions_list_variables(function_id)
List variables

Get a list of all variables of a specific function.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function unique ID. | [required] |

### Return type

[**crate::models::VariableList**](variableList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_update

> crate::models::Function functions_update(function_id, payload)
Update function

Update function by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**payload** | Option<[**FunctionsUpdateRequest**](FunctionsUpdateRequest.md)> |  |  |

### Return type

[**crate::models::Function**](function.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_update_deployment

> crate::models::Function functions_update_deployment(function_id, deployment_id)
Update function deployment

Update the function code deployment ID using the unique function ID. Use this endpoint to switch the code deployment that should be executed by the execution endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function ID. | [required] |
**deployment_id** | **String** | Deployment ID. | [required] |

### Return type

[**crate::models::Function**](function.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_update_variable

> crate::models::Variable functions_update_variable(function_id, variable_id, payload)
Update variable

Update variable by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **String** | Function unique ID. | [required] |
**variable_id** | **String** | Variable unique ID. | [required] |
**payload** | Option<[**FunctionsUpdateVariableRequest**](FunctionsUpdateVariableRequest.md)> |  |  |

### Return type

[**crate::models::Variable**](variable.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

