# \DatabasesApi

All URIs are relative to *https://HOSTNAME/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**databases_create**](DatabasesApi.md#databases_create) | **POST** /databases | Create database
[**databases_create_boolean_attribute**](DatabasesApi.md#databases_create_boolean_attribute) | **POST** /databases/{databaseId}/collections/{collectionId}/attributes/boolean | Create boolean attribute
[**databases_create_collection**](DatabasesApi.md#databases_create_collection) | **POST** /databases/{databaseId}/collections | Create collection
[**databases_create_datetime_attribute**](DatabasesApi.md#databases_create_datetime_attribute) | **POST** /databases/{databaseId}/collections/{collectionId}/attributes/datetime | Create datetime attribute
[**databases_create_document**](DatabasesApi.md#databases_create_document) | **POST** /databases/{databaseId}/collections/{collectionId}/documents | Create document
[**databases_create_email_attribute**](DatabasesApi.md#databases_create_email_attribute) | **POST** /databases/{databaseId}/collections/{collectionId}/attributes/email | Create email attribute
[**databases_create_enum_attribute**](DatabasesApi.md#databases_create_enum_attribute) | **POST** /databases/{databaseId}/collections/{collectionId}/attributes/enum | Create enum attribute
[**databases_create_float_attribute**](DatabasesApi.md#databases_create_float_attribute) | **POST** /databases/{databaseId}/collections/{collectionId}/attributes/float | Create float attribute
[**databases_create_index**](DatabasesApi.md#databases_create_index) | **POST** /databases/{databaseId}/collections/{collectionId}/indexes | Create index
[**databases_create_integer_attribute**](DatabasesApi.md#databases_create_integer_attribute) | **POST** /databases/{databaseId}/collections/{collectionId}/attributes/integer | Create integer attribute
[**databases_create_ip_attribute**](DatabasesApi.md#databases_create_ip_attribute) | **POST** /databases/{databaseId}/collections/{collectionId}/attributes/ip | Create IP address attribute
[**databases_create_relationship_attribute**](DatabasesApi.md#databases_create_relationship_attribute) | **POST** /databases/{databaseId}/collections/{collectionId}/attributes/relationship | Create relationship attribute
[**databases_create_string_attribute**](DatabasesApi.md#databases_create_string_attribute) | **POST** /databases/{databaseId}/collections/{collectionId}/attributes/string | Create string attribute
[**databases_create_url_attribute**](DatabasesApi.md#databases_create_url_attribute) | **POST** /databases/{databaseId}/collections/{collectionId}/attributes/url | Create URL attribute
[**databases_delete**](DatabasesApi.md#databases_delete) | **DELETE** /databases/{databaseId} | Delete database
[**databases_delete_attribute**](DatabasesApi.md#databases_delete_attribute) | **DELETE** /databases/{databaseId}/collections/{collectionId}/attributes/{key} | Delete attribute
[**databases_delete_collection**](DatabasesApi.md#databases_delete_collection) | **DELETE** /databases/{databaseId}/collections/{collectionId} | Delete collection
[**databases_delete_document**](DatabasesApi.md#databases_delete_document) | **DELETE** /databases/{databaseId}/collections/{collectionId}/documents/{documentId} | Delete document
[**databases_delete_index**](DatabasesApi.md#databases_delete_index) | **DELETE** /databases/{databaseId}/collections/{collectionId}/indexes/{key} | Delete index
[**databases_get**](DatabasesApi.md#databases_get) | **GET** /databases/{databaseId} | Get database
[**databases_get_attribute**](DatabasesApi.md#databases_get_attribute) | **GET** /databases/{databaseId}/collections/{collectionId}/attributes/{key} | Get attribute
[**databases_get_collection**](DatabasesApi.md#databases_get_collection) | **GET** /databases/{databaseId}/collections/{collectionId} | Get collection
[**databases_get_document**](DatabasesApi.md#databases_get_document) | **GET** /databases/{databaseId}/collections/{collectionId}/documents/{documentId} | Get document
[**databases_get_index**](DatabasesApi.md#databases_get_index) | **GET** /databases/{databaseId}/collections/{collectionId}/indexes/{key} | Get index
[**databases_list**](DatabasesApi.md#databases_list) | **GET** /databases | List databases
[**databases_list_attributes**](DatabasesApi.md#databases_list_attributes) | **GET** /databases/{databaseId}/collections/{collectionId}/attributes | List attributes
[**databases_list_collections**](DatabasesApi.md#databases_list_collections) | **GET** /databases/{databaseId}/collections | List collections
[**databases_list_documents**](DatabasesApi.md#databases_list_documents) | **GET** /databases/{databaseId}/collections/{collectionId}/documents | List documents
[**databases_list_indexes**](DatabasesApi.md#databases_list_indexes) | **GET** /databases/{databaseId}/collections/{collectionId}/indexes | List indexes
[**databases_update**](DatabasesApi.md#databases_update) | **PUT** /databases/{databaseId} | Update database
[**databases_update_boolean_attribute**](DatabasesApi.md#databases_update_boolean_attribute) | **PATCH** /databases/{databaseId}/collections/{collectionId}/attributes/boolean/{key} | Update boolean attribute
[**databases_update_collection**](DatabasesApi.md#databases_update_collection) | **PUT** /databases/{databaseId}/collections/{collectionId} | Update collection
[**databases_update_datetime_attribute**](DatabasesApi.md#databases_update_datetime_attribute) | **PATCH** /databases/{databaseId}/collections/{collectionId}/attributes/datetime/{key} | Update dateTime attribute
[**databases_update_document**](DatabasesApi.md#databases_update_document) | **PATCH** /databases/{databaseId}/collections/{collectionId}/documents/{documentId} | Update document
[**databases_update_email_attribute**](DatabasesApi.md#databases_update_email_attribute) | **PATCH** /databases/{databaseId}/collections/{collectionId}/attributes/email/{key} | Update email attribute
[**databases_update_enum_attribute**](DatabasesApi.md#databases_update_enum_attribute) | **PATCH** /databases/{databaseId}/collections/{collectionId}/attributes/enum/{key} | Update enum attribute
[**databases_update_float_attribute**](DatabasesApi.md#databases_update_float_attribute) | **PATCH** /databases/{databaseId}/collections/{collectionId}/attributes/float/{key} | Update float attribute
[**databases_update_integer_attribute**](DatabasesApi.md#databases_update_integer_attribute) | **PATCH** /databases/{databaseId}/collections/{collectionId}/attributes/integer/{key} | Update integer attribute
[**databases_update_ip_attribute**](DatabasesApi.md#databases_update_ip_attribute) | **PATCH** /databases/{databaseId}/collections/{collectionId}/attributes/ip/{key} | Update IP address attribute
[**databases_update_relationship_attribute**](DatabasesApi.md#databases_update_relationship_attribute) | **PATCH** /databases/{databaseId}/collections/{collectionId}/attributes/{key}/relationship | Update relationship attribute
[**databases_update_string_attribute**](DatabasesApi.md#databases_update_string_attribute) | **PATCH** /databases/{databaseId}/collections/{collectionId}/attributes/string/{key} | Update string attribute
[**databases_update_url_attribute**](DatabasesApi.md#databases_update_url_attribute) | **PATCH** /databases/{databaseId}/collections/{collectionId}/attributes/url/{key} | Update URL attribute



## databases_create

> crate::models::Database databases_create(payload)
Create database

Create a new Database. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**DatabasesCreateRequest**](DatabasesCreateRequest.md)> |  |  |

### Return type

[**crate::models::Database**](database.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_boolean_attribute

> crate::models::AttributeBoolean databases_create_boolean_attribute(database_id, collection_id, payload)
Create boolean attribute

Create a boolean attribute. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateBooleanAttributeRequest**](DatabasesCreateBooleanAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeBoolean**](attributeBoolean.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_collection

> crate::models::Collection databases_create_collection(database_id, payload)
Create collection

Create a new Collection. Before using this route, you should create a new database resource using either a [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection) API or directly from your database console.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**payload** | Option<[**DatabasesCreateCollectionRequest**](DatabasesCreateCollectionRequest.md)> |  |  |

### Return type

[**crate::models::Collection**](collection.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_datetime_attribute

> crate::models::AttributeDatetime databases_create_datetime_attribute(database_id, collection_id, payload)
Create datetime attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateDatetimeAttributeRequest**](DatabasesCreateDatetimeAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeDatetime**](attributeDatetime.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_document

> crate::models::Document databases_create_document(database_id, collection_id, payload)
Create document

Create a new Document. Before using this route, you should create a new collection resource using either a [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection) API or directly from your database console.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). Make sure to define attributes before creating documents. | [required] |
**payload** | Option<[**DatabasesCreateDocumentRequest**](DatabasesCreateDocumentRequest.md)> |  |  |

### Return type

[**crate::models::Document**](document.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_email_attribute

> crate::models::AttributeEmail databases_create_email_attribute(database_id, collection_id, payload)
Create email attribute

Create an email attribute. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateEmailAttributeRequest**](DatabasesCreateEmailAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeEmail**](attributeEmail.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_enum_attribute

> crate::models::AttributeEnum databases_create_enum_attribute(database_id, collection_id, payload)
Create enum attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateEnumAttributeRequest**](DatabasesCreateEnumAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeEnum**](attributeEnum.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_float_attribute

> crate::models::AttributeFloat databases_create_float_attribute(database_id, collection_id, payload)
Create float attribute

Create a float attribute. Optionally, minimum and maximum values can be provided. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateFloatAttributeRequest**](DatabasesCreateFloatAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeFloat**](attributeFloat.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_index

> crate::models::Index databases_create_index(database_id, collection_id, payload)
Create index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateIndexRequest**](DatabasesCreateIndexRequest.md)> |  |  |

### Return type

[**crate::models::Index**](index.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_integer_attribute

> crate::models::AttributeInteger databases_create_integer_attribute(database_id, collection_id, payload)
Create integer attribute

Create an integer attribute. Optionally, minimum and maximum values can be provided. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateIntegerAttributeRequest**](DatabasesCreateIntegerAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeInteger**](attributeInteger.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_ip_attribute

> crate::models::AttributeIp databases_create_ip_attribute(database_id, collection_id, payload)
Create IP address attribute

Create IP address attribute. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateEmailAttributeRequest**](DatabasesCreateEmailAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeIp**](attributeIp.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_relationship_attribute

> crate::models::AttributeRelationship databases_create_relationship_attribute(database_id, collection_id, payload)
Create relationship attribute

Create relationship attribute. [Learn more about relationship attributes](https://appwrite.io/docs/databases-relationships#relationship-attributes). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateRelationshipAttributeRequest**](DatabasesCreateRelationshipAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeRelationship**](attributeRelationship.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_string_attribute

> crate::models::AttributeString databases_create_string_attribute(database_id, collection_id, payload)
Create string attribute

Create a string attribute. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateStringAttributeRequest**](DatabasesCreateStringAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeString**](attributeString.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_create_url_attribute

> crate::models::AttributeUrl databases_create_url_attribute(database_id, collection_id, payload)
Create URL attribute

Create a URL attribute. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**payload** | Option<[**DatabasesCreateEmailAttributeRequest**](DatabasesCreateEmailAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeUrl**](attributeUrl.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_delete

> databases_delete(database_id)
Delete database

Delete a database by its unique ID. Only API keys with with databases.write scope can delete a database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_delete_attribute

> databases_delete_attribute(database_id, collection_id, key)
Delete attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_delete_collection

> databases_delete_collection(database_id, collection_id)
Delete collection

Delete a collection by its unique ID. Only users with write permissions have access to delete this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_delete_document

> databases_delete_document(database_id, collection_id, document_id)
Delete document

Delete a document by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**document_id** | **String** | Document ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_delete_index

> databases_delete_index(database_id, collection_id, key)
Delete index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Index Key. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get

> crate::models::Database databases_get(database_id)
Get database

Get a database by its unique ID. This endpoint response returns a JSON object with the database metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |

### Return type

[**crate::models::Database**](database.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_attribute

> serde_json::Value databases_get_attribute(database_id, collection_id, key)
Get attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_collection

> crate::models::Collection databases_get_collection(database_id, collection_id)
Get collection

Get a collection by its unique ID. This endpoint response returns a JSON object with the collection metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. | [required] |

### Return type

[**crate::models::Collection**](collection.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_document

> crate::models::Document databases_get_document(database_id, collection_id, document_id, queries)
Get document

Get a document by its unique ID. This endpoint response returns a JSON object with the document data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**document_id** | **String** | Document ID. | [required] |
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/databases#querying-documents). Only method allowed is select. |  |[default to []]

### Return type

[**crate::models::Document**](document.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_get_index

> crate::models::Index databases_get_index(database_id, collection_id, key)
Get index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Index Key. | [required] |

### Return type

[**crate::models::Index**](index.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list

> crate::models::DatabaseList databases_list(queries, search)
List databases

Get a list of all databases from the current Appwrite project. You can use the search parameter to filter your results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: name |  |[default to []]
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::DatabaseList**](databaseList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_attributes

> crate::models::AttributeList databases_list_attributes(database_id, collection_id, queries)
List attributes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: key, type, size, required, array, status, error |  |[default to []]

### Return type

[**crate::models::AttributeList**](attributeList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_collections

> crate::models::CollectionList databases_list_collections(database_id, queries, search)
List collections

Get a list of all collections that belong to the provided databaseId. You can use the search parameter to filter your results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: name, enabled, documentSecurity |  |[default to []]
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::CollectionList**](collectionList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_documents

> crate::models::DocumentList databases_list_documents(database_id, collection_id, queries)
List documents

Get a list of all the user's documents in a given collection. You can use the query params to filter your results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. |  |[default to []]

### Return type

[**crate::models::DocumentList**](documentList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_list_indexes

> crate::models::IndexList databases_list_indexes(database_id, collection_id, queries)
List indexes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: key, type, status, attributes, error |  |[default to []]

### Return type

[**crate::models::IndexList**](indexList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update

> crate::models::Database databases_update(database_id, payload)
Update database

Update a database by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**payload** | Option<[**DatabasesUpdateRequest**](DatabasesUpdateRequest.md)> |  |  |

### Return type

[**crate::models::Database**](database.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_boolean_attribute

> crate::models::AttributeBoolean databases_update_boolean_attribute(database_id, collection_id, key, payload)
Update boolean attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |
**payload** | Option<[**DatabasesUpdateBooleanAttributeRequest**](DatabasesUpdateBooleanAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeBoolean**](attributeBoolean.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_collection

> crate::models::Collection databases_update_collection(database_id, collection_id, payload)
Update collection

Update a collection by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. | [required] |
**payload** | Option<[**DatabasesUpdateCollectionRequest**](DatabasesUpdateCollectionRequest.md)> |  |  |

### Return type

[**crate::models::Collection**](collection.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_datetime_attribute

> crate::models::AttributeDatetime databases_update_datetime_attribute(database_id, collection_id, key, payload)
Update dateTime attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |
**payload** | Option<[**DatabasesUpdateDatetimeAttributeRequest**](DatabasesUpdateDatetimeAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeDatetime**](attributeDatetime.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_document

> crate::models::Document databases_update_document(database_id, collection_id, document_id, payload)
Update document

Update a document by its unique ID. Using the patch method you can pass only specific fields that will get updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. | [required] |
**document_id** | **String** | Document ID. | [required] |
**payload** | Option<[**DatabasesUpdateDocumentRequest**](DatabasesUpdateDocumentRequest.md)> |  |  |

### Return type

[**crate::models::Document**](document.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_email_attribute

> crate::models::AttributeEmail databases_update_email_attribute(database_id, collection_id, key, payload)
Update email attribute

Update an email attribute. Changing the `default` value will not update already existing documents. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |
**payload** | Option<[**DatabasesUpdateDatetimeAttributeRequest**](DatabasesUpdateDatetimeAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeEmail**](attributeEmail.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_enum_attribute

> crate::models::AttributeEnum databases_update_enum_attribute(database_id, collection_id, key, payload)
Update enum attribute

Update an enum attribute. Changing the `default` value will not update already existing documents. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |
**payload** | Option<[**DatabasesUpdateEnumAttributeRequest**](DatabasesUpdateEnumAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeEnum**](attributeEnum.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_float_attribute

> crate::models::AttributeFloat databases_update_float_attribute(database_id, collection_id, key, payload)
Update float attribute

Update a float attribute. Changing the `default` value will not update already existing documents. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |
**payload** | Option<[**DatabasesUpdateFloatAttributeRequest**](DatabasesUpdateFloatAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeFloat**](attributeFloat.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_integer_attribute

> crate::models::AttributeInteger databases_update_integer_attribute(database_id, collection_id, key, payload)
Update integer attribute

Update an integer attribute. Changing the `default` value will not update already existing documents. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |
**payload** | Option<[**DatabasesUpdateIntegerAttributeRequest**](DatabasesUpdateIntegerAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeInteger**](attributeInteger.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_ip_attribute

> crate::models::AttributeIp databases_update_ip_attribute(database_id, collection_id, key, payload)
Update IP address attribute

Update an ip attribute. Changing the `default` value will not update already existing documents. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |
**payload** | Option<[**DatabasesUpdateDatetimeAttributeRequest**](DatabasesUpdateDatetimeAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeIp**](attributeIp.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_relationship_attribute

> crate::models::AttributeRelationship databases_update_relationship_attribute(database_id, collection_id, key, payload)
Update relationship attribute

Update relationship attribute. [Learn more about relationship attributes](https://appwrite.io/docs/databases-relationships#relationship-attributes). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |
**payload** | Option<[**DatabasesUpdateRelationshipAttributeRequest**](DatabasesUpdateRelationshipAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeRelationship**](attributeRelationship.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_string_attribute

> crate::models::AttributeString databases_update_string_attribute(database_id, collection_id, key, payload)
Update string attribute

Update a string attribute. Changing the `default` value will not update already existing documents. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |
**payload** | Option<[**DatabasesUpdateDatetimeAttributeRequest**](DatabasesUpdateDatetimeAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeString**](attributeString.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## databases_update_url_attribute

> crate::models::AttributeUrl databases_update_url_attribute(database_id, collection_id, key, payload)
Update URL attribute

Update an url attribute. Changing the `default` value will not update already existing documents. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **String** | Database ID. | [required] |
**collection_id** | **String** | Collection ID. You can create a new collection using the Database service [server integration](https://appwrite.io/docs/server/databases#databasesCreateCollection). | [required] |
**key** | **String** | Attribute Key. | [required] |
**payload** | Option<[**DatabasesUpdateDatetimeAttributeRequest**](DatabasesUpdateDatetimeAttributeRequest.md)> |  |  |

### Return type

[**crate::models::AttributeUrl**](attributeUrl.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

