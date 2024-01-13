# \UsersApi

All URIs are relative to *https://HOSTNAME/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_create**](UsersApi.md#users_create) | **POST** /users | Create user
[**users_create_argon2_user**](UsersApi.md#users_create_argon2_user) | **POST** /users/argon2 | Create user with Argon2 password
[**users_create_bcrypt_user**](UsersApi.md#users_create_bcrypt_user) | **POST** /users/bcrypt | Create user with bcrypt password
[**users_create_md5_user**](UsersApi.md#users_create_md5_user) | **POST** /users/md5 | Create user with MD5 password
[**users_create_ph_pass_user**](UsersApi.md#users_create_ph_pass_user) | **POST** /users/phpass | Create user with PHPass password
[**users_create_scrypt_modified_user**](UsersApi.md#users_create_scrypt_modified_user) | **POST** /users/scrypt-modified | Create user with Scrypt modified password
[**users_create_scrypt_user**](UsersApi.md#users_create_scrypt_user) | **POST** /users/scrypt | Create user with Scrypt password
[**users_create_sha_user**](UsersApi.md#users_create_sha_user) | **POST** /users/sha | Create user with SHA password
[**users_delete**](UsersApi.md#users_delete) | **DELETE** /users/{userId} | Delete user
[**users_delete_identity**](UsersApi.md#users_delete_identity) | **DELETE** /users/identities/{identityId} | Delete Identity
[**users_delete_session**](UsersApi.md#users_delete_session) | **DELETE** /users/{userId}/sessions/{sessionId} | Delete user session
[**users_delete_sessions**](UsersApi.md#users_delete_sessions) | **DELETE** /users/{userId}/sessions | Delete user sessions
[**users_get**](UsersApi.md#users_get) | **GET** /users/{userId} | Get user
[**users_get_prefs**](UsersApi.md#users_get_prefs) | **GET** /users/{userId}/prefs | Get user preferences
[**users_list**](UsersApi.md#users_list) | **GET** /users | List users
[**users_list_identities**](UsersApi.md#users_list_identities) | **GET** /users/identities | List Identities
[**users_list_logs**](UsersApi.md#users_list_logs) | **GET** /users/{userId}/logs | List user logs
[**users_list_memberships**](UsersApi.md#users_list_memberships) | **GET** /users/{userId}/memberships | List user memberships
[**users_list_sessions**](UsersApi.md#users_list_sessions) | **GET** /users/{userId}/sessions | List user sessions
[**users_update_email**](UsersApi.md#users_update_email) | **PATCH** /users/{userId}/email | Update email
[**users_update_email_verification**](UsersApi.md#users_update_email_verification) | **PATCH** /users/{userId}/verification | Update email verification
[**users_update_labels**](UsersApi.md#users_update_labels) | **PUT** /users/{userId}/labels | Update user labels
[**users_update_name**](UsersApi.md#users_update_name) | **PATCH** /users/{userId}/name | Update name
[**users_update_password**](UsersApi.md#users_update_password) | **PATCH** /users/{userId}/password | Update password
[**users_update_phone**](UsersApi.md#users_update_phone) | **PATCH** /users/{userId}/phone | Update phone
[**users_update_phone_verification**](UsersApi.md#users_update_phone_verification) | **PATCH** /users/{userId}/verification/phone | Update phone verification
[**users_update_prefs**](UsersApi.md#users_update_prefs) | **PATCH** /users/{userId}/prefs | Update user preferences
[**users_update_status**](UsersApi.md#users_update_status) | **PATCH** /users/{userId}/status | Update user status



## users_create

> crate::models::User users_create(payload)
Create user

Create a new user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**UsersCreateRequest**](UsersCreateRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_create_argon2_user

> crate::models::User users_create_argon2_user(payload)
Create user with Argon2 password

Create a new user. Password provided must be hashed with the [Argon2](https://en.wikipedia.org/wiki/Argon2) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**UsersCreateArgon2UserRequest**](UsersCreateArgon2UserRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_create_bcrypt_user

> crate::models::User users_create_bcrypt_user(payload)
Create user with bcrypt password

Create a new user. Password provided must be hashed with the [Bcrypt](https://en.wikipedia.org/wiki/Bcrypt) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**UsersCreateBcryptUserRequest**](UsersCreateBcryptUserRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_create_md5_user

> crate::models::User users_create_md5_user(payload)
Create user with MD5 password

Create a new user. Password provided must be hashed with the [MD5](https://en.wikipedia.org/wiki/MD5) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**UsersCreateMd5UserRequest**](UsersCreateMd5UserRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_create_ph_pass_user

> crate::models::User users_create_ph_pass_user(payload)
Create user with PHPass password

Create a new user. Password provided must be hashed with the [PHPass](https://www.openwall.com/phpass/) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**UsersCreatePhPassUserRequest**](UsersCreatePhPassUserRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_create_scrypt_modified_user

> crate::models::User users_create_scrypt_modified_user(payload)
Create user with Scrypt modified password

Create a new user. Password provided must be hashed with the [Scrypt Modified](https://gist.github.com/Meldiron/eecf84a0225eccb5a378d45bb27462cc) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**UsersCreateScryptModifiedUserRequest**](UsersCreateScryptModifiedUserRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_create_scrypt_user

> crate::models::User users_create_scrypt_user(payload)
Create user with Scrypt password

Create a new user. Password provided must be hashed with the [Scrypt](https://github.com/Tarsnap/scrypt) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**UsersCreateScryptUserRequest**](UsersCreateScryptUserRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_create_sha_user

> crate::models::User users_create_sha_user(payload)
Create user with SHA password

Create a new user. Password provided must be hashed with the [SHA](https://en.wikipedia.org/wiki/Secure_Hash_Algorithm) algorithm. Use the [POST /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to create users with a plain text password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**UsersCreateShaUserRequest**](UsersCreateShaUserRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_delete

> users_delete(user_id)
Delete user

Delete a user by its unique ID, thereby releasing it's ID. Since ID is released and can be reused, all user-related resources like documents or storage files should be deleted before user deletion. If you want to keep ID reserved, use the [updateStatus](https://appwrite.io/docs/server/users#usersUpdateStatus) endpoint instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_delete_identity

> users_delete_identity(identity_id)
Delete Identity

Delete an identity by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **String** | Identity ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_delete_session

> users_delete_session(user_id, session_id)
Delete user session

Delete a user sessions by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**session_id** | **String** | Session ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_delete_sessions

> users_delete_sessions(user_id)
Delete user sessions

Delete all user's sessions by using the user's unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get

> crate::models::User users_get(user_id)
Get user

Get a user by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get_prefs

> ::std::collections::HashMap<String, serde_json::Value> users_get_prefs(user_id)
Get user preferences

Get the user preferences by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list

> crate::models::UserList users_list(queries, search)
List users

Get a list of all the project's users. You can use the query params to filter your results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: name, email, phone, status, passwordUpdate, registration, emailVerification, phoneVerification |  |[default to []]
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::UserList**](userList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list_identities

> crate::models::IdentityList users_list_identities(queries, search)
List Identities

Get identities for all users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queries** | Option<**String**> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: userId, provider, providerUid, providerEmail, providerAccessTokenExpiry |  |
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::IdentityList**](identityList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list_logs

> crate::models::LogList users_list_logs(user_id, queries)
List user logs

Get the user activity logs list by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Only supported methods are limit and offset |  |[default to []]

### Return type

[**crate::models::LogList**](logList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list_memberships

> crate::models::MembershipList users_list_memberships(user_id)
List user memberships

Get the user membership list by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |

### Return type

[**crate::models::MembershipList**](membershipList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list_sessions

> crate::models::SessionList users_list_sessions(user_id)
List user sessions

Get the user sessions list by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |

### Return type

[**crate::models::SessionList**](sessionList.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_email

> crate::models::User users_update_email(user_id, payload)
Update email

Update the user email by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**payload** | Option<[**UsersUpdateEmailRequest**](UsersUpdateEmailRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_email_verification

> crate::models::User users_update_email_verification(user_id, payload)
Update email verification

Update the user email verification status by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**payload** | Option<[**UsersUpdateEmailVerificationRequest**](UsersUpdateEmailVerificationRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_labels

> crate::models::User users_update_labels(user_id, payload)
Update user labels

Update the user labels by its unique ID.   Labels can be used to grant access to resources. While teams are a way for user's to share access to a resource, labels can be defined by the developer to grant access without an invitation. See the [Permissions docs](https://appwrite.io/docs/permissions) for more info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**payload** | Option<[**UsersUpdateLabelsRequest**](UsersUpdateLabelsRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_name

> crate::models::User users_update_name(user_id, payload)
Update name

Update the user name by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**payload** | Option<[**AccountUpdateNameRequest**](AccountUpdateNameRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_password

> crate::models::User users_update_password(user_id, payload)
Update password

Update the user password by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**payload** | Option<[**UsersUpdatePasswordRequest**](UsersUpdatePasswordRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_phone

> crate::models::User users_update_phone(user_id, payload)
Update phone

Update the user phone by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**payload** | Option<[**UsersUpdatePhoneRequest**](UsersUpdatePhoneRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_phone_verification

> crate::models::User users_update_phone_verification(user_id, payload)
Update phone verification

Update the user phone verification status by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**payload** | Option<[**UsersUpdatePhoneVerificationRequest**](UsersUpdatePhoneVerificationRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_prefs

> ::std::collections::HashMap<String, serde_json::Value> users_update_prefs(user_id, payload)
Update user preferences

Update the user preferences by its unique ID. The object you pass is stored as is, and replaces any previous value. The maximum allowed prefs size is 64kB and throws error if exceeded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**payload** | Option<[**AccountUpdatePrefsRequest**](AccountUpdatePrefsRequest.md)> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_status

> crate::models::User users_update_status(user_id, payload)
Update user status

Update the user status by its unique ID. Use this endpoint as an alternative to deleting a user if you want to keep user's ID reserved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID. | [required] |
**payload** | Option<[**UsersUpdateStatusRequest**](UsersUpdateStatusRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

