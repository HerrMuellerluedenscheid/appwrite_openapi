# \AccountApi

All URIs are relative to *https://HOSTNAME/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_create_phone_verification**](AccountApi.md#account_create_phone_verification) | **POST** /account/verification/phone | Create phone verification
[**account_create_recovery**](AccountApi.md#account_create_recovery) | **POST** /account/recovery | Create password recovery
[**account_create_verification**](AccountApi.md#account_create_verification) | **POST** /account/verification | Create email verification
[**account_delete_identity**](AccountApi.md#account_delete_identity) | **DELETE** /account/identities/{identityId} | Delete Identity
[**account_delete_session**](AccountApi.md#account_delete_session) | **DELETE** /account/sessions/{sessionId} | Delete session
[**account_delete_sessions**](AccountApi.md#account_delete_sessions) | **DELETE** /account/sessions | Delete sessions
[**account_get**](AccountApi.md#account_get) | **GET** /account | Get account
[**account_get_prefs**](AccountApi.md#account_get_prefs) | **GET** /account/prefs | Get account preferences
[**account_get_session**](AccountApi.md#account_get_session) | **GET** /account/sessions/{sessionId} | Get session
[**account_list_identities**](AccountApi.md#account_list_identities) | **GET** /account/identities | List Identities
[**account_list_logs**](AccountApi.md#account_list_logs) | **GET** /account/logs | List logs
[**account_list_sessions**](AccountApi.md#account_list_sessions) | **GET** /account/sessions | List sessions
[**account_update_email**](AccountApi.md#account_update_email) | **PATCH** /account/email | Update email
[**account_update_name**](AccountApi.md#account_update_name) | **PATCH** /account/name | Update name
[**account_update_password**](AccountApi.md#account_update_password) | **PATCH** /account/password | Update password
[**account_update_phone**](AccountApi.md#account_update_phone) | **PATCH** /account/phone | Update phone
[**account_update_phone_verification**](AccountApi.md#account_update_phone_verification) | **PUT** /account/verification/phone | Create phone verification (confirmation)
[**account_update_prefs**](AccountApi.md#account_update_prefs) | **PATCH** /account/prefs | Update preferences
[**account_update_recovery**](AccountApi.md#account_update_recovery) | **PUT** /account/recovery | Create password recovery (confirmation)
[**account_update_session**](AccountApi.md#account_update_session) | **PATCH** /account/sessions/{sessionId} | Update OAuth session (refresh tokens)
[**account_update_status**](AccountApi.md#account_update_status) | **PATCH** /account/status | Update status
[**account_update_verification**](AccountApi.md#account_update_verification) | **PUT** /account/verification | Create email verification (confirmation)



## account_create_phone_verification

> crate::models::Token account_create_phone_verification()
Create phone verification

Use this endpoint to send a verification SMS to the currently logged in user. This endpoint is meant for use after updating a user's phone number using the [accountUpdatePhone](https://appwrite.io/docs/references/cloud/client-web/account#updatePhone) endpoint. Learn more about how to [complete the verification process](https://appwrite.io/docs/references/cloud/client-web/account#updatePhoneVerification). The verification code sent to the user's phone number is valid for 15 minutes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Token**](token.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_create_recovery

> crate::models::Token account_create_recovery(payload)
Create password recovery

Sends the user an email with a temporary secret key for password reset. When the user clicks the confirmation link he is redirected back to your app password reset URL with the secret key and email address values attached to the URL query string. Use the query string params to submit a request to the [PUT /account/recovery](https://appwrite.io/docs/references/cloud/client-web/account#updateRecovery) endpoint to complete the process. The verification link sent to the user's email address is valid for 1 hour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**AccountCreateRecoveryRequest**](AccountCreateRecoveryRequest.md)> |  |  |

### Return type

[**crate::models::Token**](token.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_create_verification

> crate::models::Token account_create_verification(payload)
Create email verification

Use this endpoint to send a verification message to your user email address to confirm they are the valid owners of that address. Both the **userId** and **secret** arguments will be passed as query parameters to the URL you have provided to be attached to the verification email. The provided URL should redirect the user back to your app and allow you to complete the verification process by verifying both the **userId** and **secret** parameters. Learn more about how to [complete the verification process](https://appwrite.io/docs/references/cloud/client-web/account#updateVerification). The verification link sent to the user's email address is valid for 7 days.  Please note that in order to avoid a [Redirect Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md), the only valid redirect URLs are the ones from domains you have set when adding your platforms in the console interface. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**AccountCreateVerificationRequest**](AccountCreateVerificationRequest.md)> |  |  |

### Return type

[**crate::models::Token**](token.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_delete_identity

> account_delete_identity(identity_id)
Delete Identity

Delete an identity by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **String** | Identity ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_delete_session

> account_delete_session(session_id)
Delete session

Logout the user. Use 'current' as the session ID to logout on this device, use a session ID to logout on another device. If you're looking to logout the user on all devices, use [Delete Sessions](https://appwrite.io/docs/references/cloud/client-web/account#deleteSessions) instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | Session ID. Use the string 'current' to delete the current device session. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_delete_sessions

> account_delete_sessions()
Delete sessions

Delete all sessions from the user account and remove any sessions cookies from the end client.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_get

> crate::models::User account_get()
Get account

Get the currently logged in user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_get_prefs

> ::std::collections::HashMap<String, serde_json::Value> account_get_prefs()
Get account preferences

Get the preferences as a key-value object for the currently logged in user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_get_session

> crate::models::Session account_get_session(session_id)
Get session

Use this endpoint to get a logged in user's session using a Session ID. Inputting 'current' will return the current session being used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | Session ID. Use the string 'current' to get the current device session. | [required] |

### Return type

[**crate::models::Session**](session.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_list_identities

> crate::models::IdentityList account_list_identities(queries)
List Identities

Get the list of identities for the currently logged in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queries** | Option<**String**> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: userId, provider, providerUid, providerEmail, providerAccessTokenExpiry |  |

### Return type

[**crate::models::IdentityList**](identityList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_list_logs

> crate::models::LogList account_list_logs(queries)
List logs

Get the list of latest security activity logs for the currently logged in user. Each log returns user IP address, location and date and time of log.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Only supported methods are limit and offset |  |[default to []]

### Return type

[**crate::models::LogList**](logList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_list_sessions

> crate::models::SessionList account_list_sessions()
List sessions

Get the list of active sessions across different devices for the currently logged in user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SessionList**](sessionList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_update_email

> crate::models::User account_update_email(payload)
Update email

Update currently logged in user account email address. After changing user address, the user confirmation status will get reset. A new confirmation email is not sent automatically however you can use the send confirmation email endpoint again to send the confirmation email. For security measures, user password is required to complete this request. This endpoint can also be used to convert an anonymous account to a normal one, by passing an email address and a new password. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**AccountUpdateEmailRequest**](AccountUpdateEmailRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_update_name

> crate::models::User account_update_name(payload)
Update name

Update currently logged in user account name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**AccountUpdateNameRequest**](AccountUpdateNameRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_update_password

> crate::models::User account_update_password(payload)
Update password

Update currently logged in user password. For validation, user is required to pass in the new password, and the old password. For users created with OAuth, Team Invites and Magic URL, oldPassword is optional.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**AccountUpdatePasswordRequest**](AccountUpdatePasswordRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_update_phone

> crate::models::User account_update_phone(payload)
Update phone

Update the currently logged in user's phone number. After updating the phone number, the phone verification status will be reset. A confirmation SMS is not sent automatically, however you can use the [POST /account/verification/phone](https://appwrite.io/docs/references/cloud/client-web/account#createPhoneVerification) endpoint to send a confirmation SMS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**AccountUpdatePhoneRequest**](AccountUpdatePhoneRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_update_phone_verification

> crate::models::Token account_update_phone_verification(payload)
Create phone verification (confirmation)

Use this endpoint to complete the user phone verification process. Use the **userId** and **secret** that were sent to your user's phone number to verify the user email ownership. If confirmed this route will return a 200 status code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**AccountUpdateVerificationRequest**](AccountUpdateVerificationRequest.md)> |  |  |

### Return type

[**crate::models::Token**](token.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_update_prefs

> crate::models::User account_update_prefs(payload)
Update preferences

Update currently logged in user account preferences. The object you pass is stored as is, and replaces any previous value. The maximum allowed prefs size is 64kB and throws error if exceeded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**AccountUpdatePrefsRequest**](AccountUpdatePrefsRequest.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_update_recovery

> crate::models::Token account_update_recovery(payload)
Create password recovery (confirmation)

Use this endpoint to complete the user account password reset. Both the **userId** and **secret** arguments will be passed as query parameters to the redirect URL you have provided when sending your request to the [POST /account/recovery](https://appwrite.io/docs/references/cloud/client-web/account#createRecovery) endpoint.  Please note that in order to avoid a [Redirect Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md) the only valid redirect URLs are the ones from domains you have set when adding your platforms in the console interface.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**AccountUpdateRecoveryRequest**](AccountUpdateRecoveryRequest.md)> |  |  |

### Return type

[**crate::models::Token**](token.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_update_session

> crate::models::Session account_update_session(session_id)
Update OAuth session (refresh tokens)

Access tokens have limited lifespan and expire to mitigate security risks. If session was created using an OAuth provider, this route can be used to \"refresh\" the access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | Session ID. Use the string 'current' to update the current device session. | [required] |

### Return type

[**crate::models::Session**](session.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_update_status

> crate::models::User account_update_status()
Update status

Block the currently logged in user account. Behind the scene, the user record is not deleted but permanently blocked from any access. To completely delete a user, use the Users API instead.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::User**](user.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_update_verification

> crate::models::Token account_update_verification(payload)
Create email verification (confirmation)

Use this endpoint to complete the user email verification process. Use both the **userId** and **secret** parameters that were attached to your app URL to verify the user email ownership. If confirmed this route will return a 200 status code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**AccountUpdateVerificationRequest**](AccountUpdateVerificationRequest.md)> |  |  |

### Return type

[**crate::models::Token**](token.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

