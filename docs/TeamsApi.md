# \TeamsApi

All URIs are relative to *https://HOSTNAME/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**teams_create**](TeamsApi.md#teams_create) | **POST** /teams | Create team
[**teams_create_membership**](TeamsApi.md#teams_create_membership) | **POST** /teams/{teamId}/memberships | Create team membership
[**teams_delete**](TeamsApi.md#teams_delete) | **DELETE** /teams/{teamId} | Delete team
[**teams_delete_membership**](TeamsApi.md#teams_delete_membership) | **DELETE** /teams/{teamId}/memberships/{membershipId} | Delete team membership
[**teams_get**](TeamsApi.md#teams_get) | **GET** /teams/{teamId} | Get team
[**teams_get_membership**](TeamsApi.md#teams_get_membership) | **GET** /teams/{teamId}/memberships/{membershipId} | Get team membership
[**teams_get_prefs**](TeamsApi.md#teams_get_prefs) | **GET** /teams/{teamId}/prefs | Get team preferences
[**teams_list**](TeamsApi.md#teams_list) | **GET** /teams | List teams
[**teams_list_memberships**](TeamsApi.md#teams_list_memberships) | **GET** /teams/{teamId}/memberships | List team memberships
[**teams_update_membership**](TeamsApi.md#teams_update_membership) | **PATCH** /teams/{teamId}/memberships/{membershipId} | Update membership
[**teams_update_membership_status**](TeamsApi.md#teams_update_membership_status) | **PATCH** /teams/{teamId}/memberships/{membershipId}/status | Update team membership status
[**teams_update_name**](TeamsApi.md#teams_update_name) | **PUT** /teams/{teamId} | Update name
[**teams_update_prefs**](TeamsApi.md#teams_update_prefs) | **PUT** /teams/{teamId}/prefs | Update preferences



## teams_create

> crate::models::Team teams_create(payload)
Create team

Create a new team. The user who creates the team will automatically be assigned as the owner of the team. Only the users with the owner role can invite new members, add new owners and delete or update the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | Option<[**TeamsCreateRequest**](TeamsCreateRequest.md)> |  |  |

### Return type

[**crate::models::Team**](team.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_create_membership

> crate::models::Membership teams_create_membership(team_id, payload)
Create team membership

Invite a new member to join your team. Provide an ID for existing users, or invite unregistered users using an email or phone number. If initiated from a Client SDK, Appwrite will send an email or sms with a link to join the team to the invited user, and an account will be created for them if one doesn't exist. If initiated from a Server SDK, the new member will be added automatically to the team.  You only need to provide one of a user ID, email, or phone number. Appwrite will prioritize accepting the user ID > email > phone number if you provide more than one of these parameters.  Use the `url` parameter to redirect the user from the invitation email to your app. After the user is redirected, use the [Update Team Membership Status](https://appwrite.io/docs/references/cloud/client-web/teams#updateMembershipStatus) endpoint to allow the user to accept the invitation to the team.   Please note that to avoid a [Redirect Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md) Appwrite will accept the only redirect URLs under the domains you have added as a platform on the Appwrite Console. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |
**payload** | Option<[**TeamsCreateMembershipRequest**](TeamsCreateMembershipRequest.md)> |  |  |

### Return type

[**crate::models::Membership**](membership.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_delete

> teams_delete(team_id)
Delete team

Delete a team using its ID. Only team members with the owner role can delete the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_delete_membership

> teams_delete_membership(team_id, membership_id)
Delete team membership

This endpoint allows a user to leave a team or for a team owner to delete the membership of any other team member. You can also use this endpoint to delete a user membership even if it is not accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |
**membership_id** | **String** | Membership ID. | [required] |

### Return type

 (empty response body)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_get

> crate::models::Team teams_get(team_id)
Get team

Get a team by its ID. All team members have read access for this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |

### Return type

[**crate::models::Team**](team.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_get_membership

> crate::models::Membership teams_get_membership(team_id, membership_id)
Get team membership

Get a team member by the membership unique id. All team members have read access for this resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |
**membership_id** | **String** | Membership ID. | [required] |

### Return type

[**crate::models::Membership**](membership.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_get_prefs

> ::std::collections::HashMap<String, serde_json::Value> teams_get_prefs(team_id)
Get team preferences

Get the team's shared preferences by its unique ID. If a preference doesn't need to be shared by all team members, prefer storing them in [user preferences](https://appwrite.io/docs/references/cloud/client-web/account#getPrefs).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_list

> crate::models::TeamList teams_list(queries, search)
List teams

Get a list of all the teams in which the current user is a member. You can use the parameters to filter your results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: name, total |  |[default to []]
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::TeamList**](teamList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_list_memberships

> crate::models::MembershipList teams_list_memberships(team_id, queries, search)
List team memberships

Use this endpoint to list a team's members using the team's ID. All team members have read access to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |
**queries** | Option<[**Vec<String>**](String.md)> | Array of query strings generated using the Query class provided by the SDK. [Learn more about queries](https://appwrite.io/docs/queries). Maximum of 100 queries are allowed, each 4096 characters long. You may filter on the following attributes: userId, teamId, invited, joined, confirm |  |[default to []]
**search** | Option<**String**> | Search term to filter your list results. Max length: 256 chars. |  |

### Return type

[**crate::models::MembershipList**](membershipList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_update_membership

> crate::models::Membership teams_update_membership(team_id, membership_id, payload)
Update membership

Modify the roles of a team member. Only team members with the owner role have access to this endpoint. Learn more about [roles and permissions](https://appwrite.io/docs/permissions). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |
**membership_id** | **String** | Membership ID. | [required] |
**payload** | Option<[**TeamsUpdateMembershipRequest**](TeamsUpdateMembershipRequest.md)> |  |  |

### Return type

[**crate::models::Membership**](membership.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_update_membership_status

> crate::models::Membership teams_update_membership_status(team_id, membership_id, payload)
Update team membership status

Use this endpoint to allow a user to accept an invitation to join a team after being redirected back to your app from the invitation email received by the user.  If the request is successful, a session for the user is automatically created. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |
**membership_id** | **String** | Membership ID. | [required] |
**payload** | Option<[**TeamsUpdateMembershipStatusRequest**](TeamsUpdateMembershipStatusRequest.md)> |  |  |

### Return type

[**crate::models::Membership**](membership.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_update_name

> crate::models::Team teams_update_name(team_id, payload)
Update name

Update the team's name by its unique ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |
**payload** | Option<[**TeamsUpdateNameRequest**](TeamsUpdateNameRequest.md)> |  |  |

### Return type

[**crate::models::Team**](team.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_update_prefs

> ::std::collections::HashMap<String, serde_json::Value> teams_update_prefs(team_id, payload)
Update preferences

Update the team's preferences by its unique ID. The object you pass is stored as is and replaces any previous value. The maximum allowed prefs size is 64kB and throws an error if exceeded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team ID. | [required] |
**payload** | Option<[**AccountUpdatePrefsRequest**](AccountUpdatePrefsRequest.md)> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

