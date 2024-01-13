# Bucket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dollar_id** | **String** | Bucket ID. | 
**dollar_created_at** | **String** | Bucket creation time in ISO 8601 format. | 
**dollar_updated_at** | **String** | Bucket update date in ISO 8601 format. | 
**dollar_permissions** | **Vec<String>** | Bucket permissions. [Learn more about permissions](https://appwrite.io/docs/permissions). | 
**file_security** | **bool** | Whether file-level security is enabled. [Learn more about permissions](https://appwrite.io/docs/permissions). | 
**name** | **String** | Bucket name. | 
**enabled** | **bool** | Bucket enabled. | 
**maximum_file_size** | **i32** | Maximum file size supported. | 
**allowed_file_extensions** | **Vec<String>** | Allowed file extensions. | 
**compression** | **String** | Compression algorithm choosen for compression. Will be one of none, [gzip](https://en.wikipedia.org/wiki/Gzip), or [zstd](https://en.wikipedia.org/wiki/Zstd). | 
**encryption** | **bool** | Bucket is encrypted. | 
**antivirus** | **bool** | Virus scanning is enabled. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


