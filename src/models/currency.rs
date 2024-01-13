/*
 * Appwrite
 *
 * Appwrite backend as a service cuts up to 70% of the time and costs required for building a modern application. We abstract and simplify common development tasks behind a REST APIs, to help you develop your app in a fast and secure way. For full API documentation and tutorials go to [https://appwrite.io/docs](https://appwrite.io/docs)
 *
 * The version of the OpenAPI document: 1.4.9
 * Contact: team@appwrite.io
 * Generated by: https://openapi-generator.tech
 */

/// Currency : Currency



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    /// Currency symbol.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Currency name.
    #[serde(rename = "name")]
    pub name: String,
    /// Currency native symbol.
    #[serde(rename = "symbolNative")]
    pub symbol_native: String,
    /// Number of decimal digits.
    #[serde(rename = "decimalDigits")]
    pub decimal_digits: i32,
    /// Currency digit rounding.
    #[serde(rename = "rounding")]
    pub rounding: f64,
    /// Currency code in [ISO 4217-1](http://en.wikipedia.org/wiki/ISO_4217) three-character format.
    #[serde(rename = "code")]
    pub code: String,
    /// Currency plural name
    #[serde(rename = "namePlural")]
    pub name_plural: String,
}

impl Currency {
    /// Currency
    pub fn new(symbol: String, name: String, symbol_native: String, decimal_digits: i32, rounding: f64, code: String, name_plural: String) -> Currency {
        Currency {
            symbol,
            name,
            symbol_native,
            decimal_digits,
            rounding,
            code,
            name_plural,
        }
    }
}


