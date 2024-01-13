# \StorageApi

All URIs are relative to *https://HOSTNAME/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**storage_create_bucket**](StorageApi.md#storage_create_bucket) | **POST** /storage/buckets | Create bucket
[**storage_create_file**](StorageApi.md#storage_create_file) | **POST** /storage/buckets/{bucketId}/files | Create file
[**storage_delete_bucket**](StorageApi.md#storage_delete_bucket) | **DELETE** /storage/buckets/{bucketId} | Delete bucket
[**storage_delete_file**](StorageApi.md#storage_delete_file) | **DELETE** /storage/buckets/{bucketId}/files/{fileId} | Delete File
[**storage_get_bucket**](StorageApi.md#storage_get_bucket) | **GET** /storage/buckets/{bucketId} | Get bucket
[**storage_get_file**](StorageApi.md#storage_get_file) | **GET** /storage/buckets/{bucketId}/files/{fileId} | Get file
[**storage_get_file_download**](StorageApi.md#storage_get_file_download) | **GET** /storage/buckets/{bucketId}/files/{fileId}/download | Get file for download
[**storage_get_file_preview**](StorageApi.md#storage_get_file_preview) | **GET** /storage/buckets/{bucketId}/files/{fileId}/preview | Get file preview
[**storage_get_file_view**](StorageApi.md#storage_get_file_view) | **GET** /storage/buckets/{bucketId}/files/{fileId}/view | Get file for view
[**storage_list_buckets**](StorageApi.md#storage_list_buckets) | **GET** /storage/buckets | List buckets
[**storage_list_files**](StorageApi.md#storage_list_files) | **GET** /storage/buckets/{bucketId}/files | List files
[**storage_update_bucket**](StorageApi.md#storage_update_bucket) | **PUT** /storage/buckets/{bucketId} | Update bucket
[**storage_update_file**](StorageApi.md#storage_update_file) | **PUT** /storage/buckets/{bucketId}/files/{fileId} | Update file



## storage_create_bucket

> crate::models::Bucket storage_create_bucket(payload)
Create bucket

Create a new storage bucket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**StorageCreateBucketRequest**](StorageCreateBucketRequest.md)> |  |  |

### Return type

[**crate::models::Bucket**](bucket.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_create_file

> std::path::PathBuf storage_create_file(bucket_id, file_id, file, permissions)
Create file

Create a new file. Before using this route, you should create a new bucket resource using either a [server integration](https://appwrite.io/docs/server/storage#storageCreateBucket) API or directly from your Appwrite console.  Larger files should be uploaded using multiple requests with the [content-range](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Range) header to send a partial request with a maximum supported chunk of `5MB`. The `content-range` header values should always be in bytes.  When the first request is sent, the server will return the **File** object, and the subsequent part request must include the file's **id** in `x-appwrite-id` header to allow the server to know that the partial upload is for the existing file and not for a new one.  If you're creating a new file using one of the Appwrite SDKs, all the chunking logic will be managed by the SDK internally. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Storage bucket unique ID. You can create a new storage bucket using the Storage service [server integration](https://appwrite.io/docs/server/storage#createBucket). | [required] |
**file_id** | **String** | File ID. Choose a custom ID or generate a random ID with `ID.unique()`. Valid chars are a-z, A-Z, 0-9, period, hyphen, and underscore. Can't start with a special char. Max length is 36 chars. | [required] |
**file** | **std::path::PathBuf** | Binary file. Appwrite SDKs provide helpers to handle file input. [Learn about file input](https://appwrite.io/docs/storage#file-input). | [required] |
**permissions** | Option<[**Vec<String>**](String.md)> | An array of permission strings. By default, only the current user is granted all permissions. [Learn more about permissions](https://appwrite.io/docs/permissions). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_delete_bucket

> storage_delete_bucket(bucket_id)
Delete bucket

Delete a storage bucket by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Bucket unique ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_delete_file

> storage_delete_file(bucket_id, file_id)
Delete File

Delete a file by its unique ID. Only users with write permissions have access to delete this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Storage bucket unique ID. You can create a new storage bucket using the Storage service [server integration](https://appwrite.io/docs/server/storage#createBucket). | [required] |
**file_id** | **String** | File ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_get_bucket

> crate::models::Bucket storage_get_bucket(bucket_id)
Get bucket

Get a storage bucket by its unique ID. This endpoint response returns a JSON object with the storage bucket metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Bucket unique ID. | [required] |

### Return type

[**crate::models::Bucket**](bucket.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_get_file

> std::path::PathBuf storage_get_file(bucket_id, file_id)
Get file

Get a file by its unique ID. This endpoint response returns a JSON object with the file metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Storage bucket unique ID. You can create a new storage bucket using the Storage service [server integration](https://appwrite.io/docs/server/storage#createBucket). | [required] |
**file_id** | **String** | File ID. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_get_file_download

> std::path::PathBuf storage_get_file_download(bucket_id, file_id)
Get file for download

Get a file content by its unique ID. The endpoint response return with a 'Content-Disposition: attachment' header that tells the browser to start downloading the file to user downloads directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Storage bucket ID. You can create a new storage bucket using the Storage service [server integration](https://appwrite.io/docs/server/storage#createBucket). | [required] |
**file_id** | **String** | File ID. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_get_file_preview

> std::path::PathBuf storage_get_file_preview(bucket_id, file_id, width, height, gravity, quality, border_width, border_color, border_radius, opacity, rotation, background, output)
Get file preview

Get a file preview image. Currently, this method supports preview for image files (jpg, png, and gif), other supported formats, like pdf, docs, slides, and spreadsheets, will return the file icon image. You can also pass query string arguments for cutting and resizing your preview image. Preview is supported only for image files smaller than 10MB.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Storage bucket unique ID. You can create a new storage bucket using the Storage service [server integration](https://appwrite.io/docs/server/storage#createBucket). | [required] |
**file_id** | **String** | File ID | [required] |
**width** | Option<**i32**> | Resize preview image width, Pass an integer between 0 to 4000. |  |[default to 0]
**height** | Option<**i32**> | Resize preview image height, Pass an integer between 0 to 4000. |  |[default to 0]
**gravity** | Option<**String**> | Image crop gravity. Can be one of center,top-left,top,top-right,left,right,bottom-left,bottom,bottom-right |  |[default to center]
**quality** | Option<**i32**> | Preview image quality. Pass an integer between 0 to 100. Defaults to 100. |  |[default to 100]
**border_width** | Option<**i32**> | Preview image border in pixels. Pass an integer between 0 to 100. Defaults to 0. |  |[default to 0]
**border_color** | Option<**String**> | Preview image border color. Use a valid HEX color, no # is needed for prefix. |  |
**border_radius** | Option<**i32**> | Preview image border radius in pixels. Pass an integer between 0 to 4000. |  |[default to 0]
**opacity** | Option<**f32**> | Preview image opacity. Only works with images having an alpha channel (like png). Pass a number between 0 to 1. |  |[default to 1.0]
**rotation** | Option<**i32**> | Preview image rotation in degrees. Pass an integer between -360 and 360. |  |[default to 0]
**background** | Option<**String**> | Preview image background color. Only works with transparent images (png). Use a valid HEX color, no # is needed for prefix. |  |
**output** | Option<**String**> | Output format type (jpeg, jpg, png, gif and webp). |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_get_file_view

> std::path::PathBuf storage_get_file_view(bucket_id, file_id)
Get file for view

Get a file content by its unique ID. This endpoint is similar to the download method but returns with no  'Content-Disposition: attachment' header.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Storage bucket unique ID. You can create a new storage bucket using the Storage service [server integration](https://appwrite.io/docs/server/storage#createBucket). | [required] |
**file_id** | **String** | File ID. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_list_buckets

> crate::models::BucketList storage_list_buckets(queries, search)
List buckets

Get a list of all the storage buckets. You can use the query params to filter your results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: enabled, name, fileSecurity, maximumFileSize, encryption, antivirus |  |[default to []]
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::BucketList**](bucketList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_list_files

> crate::models::FileList storage_list_files(bucket_id, queries, search)
List files

Get a list of all the user files. You can use the query params to filter your results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Storage bucket unique ID. You can create a new storage bucket using the Storage service [server integration](https://appwrite.io/docs/server/storage#createBucket). | [required] |
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: name, signature, mimeType, sizeOriginal, chunksTotal, chunksUploaded |  |[default to []]
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::FileList**](fileList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_update_bucket

> crate::models::Bucket storage_update_bucket(bucket_id, payload)
Update bucket

Update a storage bucket by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Bucket unique ID. | [required] |
**payload** | Option<[**StorageUpdateBucketRequest**](StorageUpdateBucketRequest.md)> |  |  |

### Return type

[**crate::models::Bucket**](bucket.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_update_file

> std::path::PathBuf storage_update_file(bucket_id, file_id, payload)
Update file

Update a file by its unique ID. Only users with write permissions have access to update this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_id** | **String** | Storage bucket unique ID. You can create a new storage bucket using the Storage service [server integration](https://appwrite.io/docs/server/storage#createBucket). | [required] |
**file_id** | **String** | File unique ID. | [required] |
**payload** | Option<[**StorageUpdateFileRequest**](StorageUpdateFileRequest.md)> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

