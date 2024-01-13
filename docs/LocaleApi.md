# \LocaleApi

All URIs are relative to *https://HOSTNAME/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**locale_get**](LocaleApi.md#locale_get) | **GET** /locale | Get user locale
[**locale_list_codes**](LocaleApi.md#locale_list_codes) | **GET** /locale/codes | List Locale Codes
[**locale_list_continents**](LocaleApi.md#locale_list_continents) | **GET** /locale/continents | List continents
[**locale_list_countries**](LocaleApi.md#locale_list_countries) | **GET** /locale/countries | List countries
[**locale_list_countries_eu**](LocaleApi.md#locale_list_countries_eu) | **GET** /locale/countries/eu | List EU countries
[**locale_list_countries_phones**](LocaleApi.md#locale_list_countries_phones) | **GET** /locale/countries/phones | List countries phone codes
[**locale_list_currencies**](LocaleApi.md#locale_list_currencies) | **GET** /locale/currencies | List currencies
[**locale_list_languages**](LocaleApi.md#locale_list_languages) | **GET** /locale/languages | List languages



## locale_get

> crate::models::Locale locale_get()
Get user locale

Get the current user location based on IP. Returns an object with user country code, country name, continent name, continent code, ip address and suggested currency. You can use the locale header to get the data in a supported language.  ([IP Geolocation by DB-IP](https://db-ip.com))

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Locale**](locale.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## locale_list_codes

> crate::models::LocaleCodeList locale_list_codes()
List Locale Codes

List of all locale codes in [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LocaleCodeList**](localeCodeList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## locale_list_continents

> crate::models::ContinentList locale_list_continents()
List continents

List of all continents. You can use the locale header to get the data in a supported language.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ContinentList**](continentList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## locale_list_countries

> crate::models::CountryList locale_list_countries()
List countries

List of all countries. You can use the locale header to get the data in a supported language.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CountryList**](countryList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## locale_list_countries_eu

> crate::models::CountryList locale_list_countries_eu()
List EU countries

List of all countries that are currently members of the EU. You can use the locale header to get the data in a supported language.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CountryList**](countryList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## locale_list_countries_phones

> crate::models::PhoneList locale_list_countries_phones()
List countries phone codes

List of all countries phone codes. You can use the locale header to get the data in a supported language.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PhoneList**](phoneList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## locale_list_currencies

> crate::models::CurrencyList locale_list_currencies()
List currencies

List of all currencies, including currency symbol, name, plural, and decimal digits for all major and minor currencies. You can use the locale header to get the data in a supported language.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CurrencyList**](currencyList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## locale_list_languages

> crate::models::LanguageList locale_list_languages()
List languages

List of all languages classified by ISO 639-1 including 2-letter code, name in English, and name in the respective language.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LanguageList**](languageList.md)

### Authorization

[Project](../README.md#Project), [JWT](../README.md#JWT), [Key](../README.md#Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

