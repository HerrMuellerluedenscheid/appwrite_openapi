# Deployment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dollar_id** | **String** | Deployment ID. | 
**dollar_created_at** | **String** | Deployment creation date in ISO 8601 format. | 
**dollar_updated_at** | **String** | Deployment update date in ISO 8601 format. | 
**r#type** | **String** | Type of deployment. | 
**resource_id** | **String** | Resource ID. | 
**resource_type** | **String** | Resource type. | 
**entrypoint** | **String** | The entrypoint file to use to execute the deployment code. | 
**size** | **i32** | The code size in bytes. | 
**build_id** | **String** | The current build ID. | 
**activate** | **bool** | Whether the deployment should be automatically activated. | 
**status** | **String** | The deployment status. Possible values are \"processing\", \"building\", \"waiting\", \"ready\", and \"failed\". | 
**build_logs** | **String** | The build logs. | 
**build_time** | **i32** | The current build time in seconds. | 
**provider_repository_name** | **String** | The name of the vcs provider repository | 
**provider_repository_owner** | **String** | The name of the vcs provider repository owner | 
**provider_repository_url** | **String** | The url of the vcs provider repository | 
**provider_branch** | **String** | The branch of the vcs repository | 
**provider_commit_hash** | **String** | The commit hash of the vcs commit | 
**provider_commit_author_url** | **String** | The url of vcs commit author | 
**provider_commit_author** | **String** | The name of vcs commit author | 
**provider_commit_message** | **String** | The commit message | 
**provider_commit_url** | **String** | The url of the vcs commit | 
**provider_branch_url** | **String** | The branch of the vcs repository | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


