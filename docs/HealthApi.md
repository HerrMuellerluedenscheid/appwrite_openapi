# \HealthApi

All URIs are relative to *https://HOSTNAME/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**health_get**](HealthApi.md#health_get) | **GET** /health | Get HTTP
[**health_get_antivirus**](HealthApi.md#health_get_antivirus) | **GET** /health/anti-virus | Get antivirus
[**health_get_cache**](HealthApi.md#health_get_cache) | **GET** /health/cache | Get cache
[**health_get_db**](HealthApi.md#health_get_db) | **GET** /health/db | Get DB
[**health_get_pub_sub**](HealthApi.md#health_get_pub_sub) | **GET** /health/pubsub | Get pubsub
[**health_get_queue**](HealthApi.md#health_get_queue) | **GET** /health/queue | Get queue
[**health_get_queue_builds**](HealthApi.md#health_get_queue_builds) | **GET** /health/queue/builds | Get builds queue
[**health_get_queue_certificates**](HealthApi.md#health_get_queue_certificates) | **GET** /health/queue/certificates | Get certificates queue
[**health_get_queue_databases**](HealthApi.md#health_get_queue_databases) | **GET** /health/queue/databases | Get databases queue
[**health_get_queue_deletes**](HealthApi.md#health_get_queue_deletes) | **GET** /health/queue/deletes | Get deletes queue
[**health_get_queue_functions**](HealthApi.md#health_get_queue_functions) | **GET** /health/queue/functions | Get functions queue
[**health_get_queue_logs**](HealthApi.md#health_get_queue_logs) | **GET** /health/queue/logs | Get logs queue
[**health_get_queue_mails**](HealthApi.md#health_get_queue_mails) | **GET** /health/queue/mails | Get mails queue
[**health_get_queue_messaging**](HealthApi.md#health_get_queue_messaging) | **GET** /health/queue/messaging | Get messaging queue
[**health_get_queue_migrations**](HealthApi.md#health_get_queue_migrations) | **GET** /health/queue/migrations | Get migrations queue
[**health_get_queue_webhooks**](HealthApi.md#health_get_queue_webhooks) | **GET** /health/queue/webhooks | Get webhooks queue
[**health_get_storage_local**](HealthApi.md#health_get_storage_local) | **GET** /health/storage/local | Get local storage
[**health_get_time**](HealthApi.md#health_get_time) | **GET** /health/time | Get time



## health_get

> crate::models::HealthStatus health_get()
Get HTTP

Check the Appwrite HTTP server is up and responsive.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthStatus**](healthStatus.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_antivirus

> crate::models::HealthAntivirus health_get_antivirus()
Get antivirus

Check the Appwrite Antivirus server is up and connection is successful.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthAntivirus**](healthAntivirus.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_cache

> crate::models::HealthStatus health_get_cache()
Get cache

Check the Appwrite in-memory cache servers are up and connection is successful.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthStatus**](healthStatus.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_db

> crate::models::HealthStatus health_get_db()
Get DB

Check the Appwrite database servers are up and connection is successful.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthStatus**](healthStatus.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_pub_sub

> crate::models::HealthStatus health_get_pub_sub()
Get pubsub

Check the Appwrite pub-sub servers are up and connection is successful.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthStatus**](healthStatus.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue

> crate::models::HealthStatus health_get_queue()
Get queue

Check the Appwrite queue messaging servers are up and connection is successful.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthStatus**](healthStatus.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue_builds

> crate::models::HealthQueue health_get_queue_builds()
Get builds queue

Get the number of builds that are waiting to be processed in the Appwrite internal queue server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthQueue**](healthQueue.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue_certificates

> crate::models::HealthQueue health_get_queue_certificates()
Get certificates queue

Get the number of certificates that are waiting to be issued against [Letsencrypt](https://letsencrypt.org/) in the Appwrite internal queue server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthQueue**](healthQueue.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue_databases

> crate::models::HealthQueue health_get_queue_databases(name)
Get databases queue

Get the number of database changes that are waiting to be processed in the Appwrite internal queue server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Queue name for which to check the queue size |  |[default to database_db_main]

### Return type

[**crate::models::HealthQueue**](healthQueue.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue_deletes

> crate::models::HealthQueue health_get_queue_deletes()
Get deletes queue

Get the number of background destructive changes that are waiting to be processed in the Appwrite internal queue server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthQueue**](healthQueue.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue_functions

> crate::models::HealthQueue health_get_queue_functions()
Get functions queue

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthQueue**](healthQueue.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue_logs

> crate::models::HealthQueue health_get_queue_logs()
Get logs queue

Get the number of logs that are waiting to be processed in the Appwrite internal queue server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthQueue**](healthQueue.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue_mails

> crate::models::HealthQueue health_get_queue_mails()
Get mails queue

Get the number of mails that are waiting to be processed in the Appwrite internal queue server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthQueue**](healthQueue.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue_messaging

> crate::models::HealthQueue health_get_queue_messaging()
Get messaging queue

Get the number of messages that are waiting to be processed in the Appwrite internal queue server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthQueue**](healthQueue.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue_migrations

> crate::models::HealthQueue health_get_queue_migrations()
Get migrations queue

Get the number of migrations that are waiting to be processed in the Appwrite internal queue server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthQueue**](healthQueue.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_queue_webhooks

> crate::models::HealthQueue health_get_queue_webhooks()
Get webhooks queue

Get the number of webhooks that are waiting to be processed in the Appwrite internal queue server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthQueue**](healthQueue.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_storage_local

> crate::models::HealthStatus health_get_storage_local()
Get local storage

Check the Appwrite local storage device is up and connection is successful.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthStatus**](healthStatus.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_get_time

> crate::models::HealthTime health_get_time()
Get time

Check the Appwrite server time is synced with Google remote NTP server. We use this technology to smoothly handle leap seconds with no disruptive events. The [Network Time Protocol](https://en.wikipedia.org/wiki/Network_Time_Protocol) (NTP) is used by hundreds of millions of computers and devices to synchronize their clocks over the Internet. If your computer sets its own clock, it likely uses NTP.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HealthTime**](healthTime.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

