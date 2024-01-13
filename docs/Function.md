# Function

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dollar_id** | **String** | Function ID. | 
**dollar_created_at** | **String** | Function creation date in ISO 8601 format. | 
**dollar_updated_at** | **String** | Function update date in ISO 8601 format. | 
**execute** | **Vec<String>** | Execution permissions. | 
**name** | **String** | Function name. | 
**enabled** | **bool** | Function enabled. | 
**live** | **bool** | Is the function deployed with the latest configuration? This is set to false if you've changed an environment variables, entrypoint, commands, or other settings that needs redeploy to be applied. When the value is false, redeploy the function to update it with the latest configuration. | 
**logging** | **bool** | Whether executions will be logged. When set to false, executions will not be logged, but will reduce resource used by your Appwrite project. | 
**runtime** | **String** | Function execution runtime. | 
**deployment** | **String** | Function's active deployment ID. | 
**vars** | [**Vec<crate::models::Variable>**](variable.md) | Function variables. | 
**events** | **Vec<String>** | Function trigger events. | 
**schedule** | **String** | Function execution schedult in CRON format. | 
**timeout** | **i32** | Function execution timeout in seconds. | 
**entrypoint** | **String** | The entrypoint file used to execute the deployment. | 
**commands** | **String** | The build command used to build the deployment. | 
**version** | **String** | Version of Open Runtimes used for the function. | 
**installation_id** | **String** | Function VCS (Version Control System) installation id. | 
**provider_repository_id** | **String** | VCS (Version Control System) Repository ID | 
**provider_branch** | **String** | VCS (Version Control System) branch name | 
**provider_root_directory** | **String** | Path to function in VCS (Version Control System) repository | 
**provider_silent_mode** | **bool** | Is VCS (Version Control System) connection is in silent mode? When in silence mode, no comments will be posted on the repository pull or merge requests | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


