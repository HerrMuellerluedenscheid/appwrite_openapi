# FunctionsUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Function name. Max length: 128 chars. | [default to null]
**runtime** | Option<**String**> | Execution runtime. | [optional][default to ]
**execute** | Option<**Vec<String>**> | An array of role strings with execution permissions. By default no user is granted with any execute permissions. [learn more about roles](https://appwrite.io/docs/permissions#permission-roles). Maximum of 100 roles are allowed, each 64 characters long. | [optional]
**events** | Option<**Vec<String>**> | Events list. Maximum of 100 events are allowed. | [optional]
**schedule** | Option<**String**> | Schedule CRON syntax. | [optional][default to ]
**timeout** | Option<**i32**> | Maximum execution time in seconds. | [optional]
**enabled** | Option<**bool**> | Is function enabled? When set to 'disabled', users cannot access the function but Server SDKs with and API key can still access the function. No data is lost when this is toggled. | [optional][default to true]
**logging** | Option<**bool**> | Whether executions will be logged. When set to false, executions will not be logged, but will reduce resource used by your Appwrite project. | [optional][default to true]
**entrypoint** | Option<**String**> | Entrypoint File. This path is relative to the \"providerRootDirectory\". | [optional][default to ]
**commands** | Option<**String**> | Build Commands. | [optional][default to ]
**installation_id** | Option<**String**> | Appwrite Installation ID for VCS (Version Controle System) deployment. | [optional][default to ]
**provider_repository_id** | Option<**String**> | Repository ID of the repo linked to the function | [optional][default to ]
**provider_branch** | Option<**String**> | Production branch for the repo linked to the function | [optional][default to ]
**provider_silent_mode** | Option<**bool**> | Is the VCS (Version Control System) connection in silent mode for the repo linked to the function? In silent mode, comments will not be made on commits and pull requests. | [optional][default to false]
**provider_root_directory** | Option<**String**> | Path to function code in the linked repo. | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


