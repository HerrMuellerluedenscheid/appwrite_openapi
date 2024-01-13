# StorageCreateBucketRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bucket_id** | **String** | Unique Id. Choose a custom ID or generate a random ID with `ID.unique()`. Valid chars are a-z, A-Z, 0-9, period, hyphen, and underscore. Can't start with a special char. Max length is 36 chars. | [default to null]
**name** | **String** | Bucket name | [default to null]
**permissions** | Option<**Vec<String>**> | An array of permission strings. By default, no user is granted with any permissions. [Learn more about permissions](https://appwrite.io/docs/permissions). | [optional]
**file_security** | Option<**bool**> | Enables configuring permissions for individual file. A user needs one of file or bucket level permissions to access a file. [Learn more about permissions](https://appwrite.io/docs/permissions). | [optional][default to false]
**enabled** | Option<**bool**> | Is bucket enabled? When set to 'disabled', users cannot access the files in this bucket but Server SDKs with and API key can still access the bucket. No files are lost when this is toggled. | [optional][default to true]
**maximum_file_size** | Option<**i32**> | Maximum file size allowed in bytes. Maximum allowed value is 30MB. | [optional]
**allowed_file_extensions** | Option<**Vec<String>**> | Allowed file extensions. Maximum of 100 extensions are allowed, each 64 characters long. | [optional]
**compression** | Option<**String**> | Compression algorithm choosen for compression. Can be one of none,  [gzip](https://en.wikipedia.org/wiki/Gzip), or [zstd](https://en.wikipedia.org/wiki/Zstd), For file size above 20MB compression is skipped even if it's enabled | [optional][default to None]
**encryption** | Option<**bool**> | Is encryption enabled? For file size above 20MB encryption is skipped even if it's enabled | [optional][default to true]
**antivirus** | Option<**bool**> | Is virus scanning enabled? For file size above 20MB AntiVirus scanning is skipped even if it's enabled | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


