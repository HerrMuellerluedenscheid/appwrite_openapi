# \AvatarsApi

All URIs are relative to *https://HOSTNAME/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**avatars_get_browser**](AvatarsApi.md#avatars_get_browser) | **GET** /avatars/browsers/{code} | Get browser icon
[**avatars_get_credit_card**](AvatarsApi.md#avatars_get_credit_card) | **GET** /avatars/credit-cards/{code} | Get credit card icon
[**avatars_get_favicon**](AvatarsApi.md#avatars_get_favicon) | **GET** /avatars/favicon | Get favicon
[**avatars_get_flag**](AvatarsApi.md#avatars_get_flag) | **GET** /avatars/flags/{code} | Get country flag
[**avatars_get_image**](AvatarsApi.md#avatars_get_image) | **GET** /avatars/image | Get image from URL
[**avatars_get_initials**](AvatarsApi.md#avatars_get_initials) | **GET** /avatars/initials | Get user initials
[**avatars_get_qr**](AvatarsApi.md#avatars_get_qr) | **GET** /avatars/qr | Get QR code



## avatars_get_browser

> std::path::PathBuf avatars_get_browser(code, width, height, quality)
Get browser icon

You can use this endpoint to show different browser icons to your users. The code argument receives the browser code as it appears in your user [GET /account/sessions](https://appwrite.io/docs/references/cloud/client-web/account#getSessions) endpoint. Use width, height and quality arguments to change the output settings.  When one dimension is specified and the other is 0, the image is scaled with preserved aspect ratio. If both dimensions are 0, the API provides an image at source quality. If dimensions are not specified, the default size of image returned is 100x100px.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Browser Code. | [required] |
**width** | Option<**i32**> | Image width. Pass an integer between 0 to 2000. Defaults to 100. |  |[default to 100]
**height** | Option<**i32**> | Image height. Pass an integer between 0 to 2000. Defaults to 100. |  |[default to 100]
**quality** | Option<**i32**> | Image quality. Pass an integer between 0 to 100. Defaults to 100. |  |[default to 100]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## avatars_get_credit_card

> std::path::PathBuf avatars_get_credit_card(code, width, height, quality)
Get credit card icon

The credit card endpoint will return you the icon of the credit card provider you need. Use width, height and quality arguments to change the output settings.  When one dimension is specified and the other is 0, the image is scaled with preserved aspect ratio. If both dimensions are 0, the API provides an image at source quality. If dimensions are not specified, the default size of image returned is 100x100px. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Credit Card Code. Possible values: amex, argencard, cabal, censosud, diners, discover, elo, hipercard, jcb, mastercard, naranja, targeta-shopping, union-china-pay, visa, mir, maestro. | [required] |
**width** | Option<**i32**> | Image width. Pass an integer between 0 to 2000. Defaults to 100. |  |[default to 100]
**height** | Option<**i32**> | Image height. Pass an integer between 0 to 2000. Defaults to 100. |  |[default to 100]
**quality** | Option<**i32**> | Image quality. Pass an integer between 0 to 100. Defaults to 100. |  |[default to 100]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## avatars_get_favicon

> std::path::PathBuf avatars_get_favicon(url)
Get favicon

Use this endpoint to fetch the favorite icon (AKA favicon) of any remote website URL. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | Website URL which you want to fetch the favicon from. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## avatars_get_flag

> std::path::PathBuf avatars_get_flag(code, width, height, quality)
Get country flag

You can use this endpoint to show different country flags icons to your users. The code argument receives the 2 letter country code. Use width, height and quality arguments to change the output settings. Country codes follow the [ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) standard.  When one dimension is specified and the other is 0, the image is scaled with preserved aspect ratio. If both dimensions are 0, the API provides an image at source quality. If dimensions are not specified, the default size of image returned is 100x100px. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Country Code. ISO Alpha-2 country code format. | [required] |
**width** | Option<**i32**> | Image width. Pass an integer between 0 to 2000. Defaults to 100. |  |[default to 100]
**height** | Option<**i32**> | Image height. Pass an integer between 0 to 2000. Defaults to 100. |  |[default to 100]
**quality** | Option<**i32**> | Image quality. Pass an integer between 0 to 100. Defaults to 100. |  |[default to 100]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## avatars_get_image

> std::path::PathBuf avatars_get_image(url, width, height)
Get image from URL

Use this endpoint to fetch a remote image URL and crop it to any image size you want. This endpoint is very useful if you need to crop and display remote images in your app or in case you want to make sure a 3rd party image is properly served using a TLS protocol.  When one dimension is specified and the other is 0, the image is scaled with preserved aspect ratio. If both dimensions are 0, the API provides an image at source quality. If dimensions are not specified, the default size of image returned is 400x400px. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | Image URL which you want to crop. | [required] |
**width** | Option<**i32**> | Resize preview image width, Pass an integer between 0 to 2000. Defaults to 400. |  |[default to 400]
**height** | Option<**i32**> | Resize preview image height, Pass an integer between 0 to 2000. Defaults to 400. |  |[default to 400]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## avatars_get_initials

> std::path::PathBuf avatars_get_initials(name, width, height, background)
Get user initials

Use this endpoint to show your user initials avatar icon on your website or app. By default, this route will try to print your logged-in user name or email initials. You can also overwrite the user name if you pass the 'name' parameter. If no name is given and no user is logged, an empty avatar will be returned.  You can use the color and background params to change the avatar colors. By default, a random theme will be selected. The random theme will persist for the user's initials when reloading the same theme will always return for the same initials.  When one dimension is specified and the other is 0, the image is scaled with preserved aspect ratio. If both dimensions are 0, the API provides an image at source quality. If dimensions are not specified, the default size of image returned is 100x100px. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Full Name. When empty, current user name or email will be used. Max length: 128 chars. |  |
**width** | Option<**i32**> | Image width. Pass an integer between 0 to 2000. Defaults to 100. |  |[default to 500]
**height** | Option<**i32**> | Image height. Pass an integer between 0 to 2000. Defaults to 100. |  |[default to 500]
**background** | Option<**String**> | Changes background color. By default a random color will be picked and stay will persistent to the given name. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## avatars_get_qr

> std::path::PathBuf avatars_get_qr(text, size, margin, download)
Get QR code

Converts a given plain text to a QR code image. You can use the query parameters to change the size and style of the resulting image. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text** | **String** | Plain text to be converted to QR code image. | [required] |
**size** | Option<**i32**> | QR code size. Pass an integer between 1 to 1000. Defaults to 400. |  |[default to 400]
**margin** | Option<**i32**> | Margin from edge. Pass an integer between 0 to 10. Defaults to 1. |  |[default to 1]
**download** | Option<**bool**> | Return resulting image with 'Content-Disposition: attachment ' headers for the browser to start downloading it. Pass 0 for no header, or 1 for otherwise. Default value is set to 0. |  |[default to false]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

