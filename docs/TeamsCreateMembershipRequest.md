# TeamsCreateMembershipRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> | Email of the new team member. | [optional][default to ]
**user_id** | Option<**String**> | ID of the user to be added to a team. | [optional][default to ]
**phone** | Option<**String**> | Phone number. Format this number with a leading '+' and a country code, e.g., +16175551212. | [optional][default to ]
**roles** | **Vec<String>** | Array of strings. Use this param to set the user roles in the team. A role can be any string. Learn more about [roles and permissions](https://appwrite.io/docs/permissions). Maximum of 100 roles are allowed, each 32 characters long. | 
**url** | Option<**String**> | URL to redirect the user back to your app from the invitation email.  Only URLs from hostnames in your project platform list are allowed. This requirement helps to prevent an [open redirect](https://cheatsheetseries.owasp.org/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.html) attack against your project API. | [optional][default to ]
**name** | Option<**String**> | Name of the new team member. Max length: 128 chars. | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


