pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    ///ListUsersDevicesResponseItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "properties": {
    "info": {
      "type": "object",
      "properties": {
        "location": {
          "type": "string"
        },
        "name": {
          "type": "string"
        }
      }
    },
    "lastData": {
      "type": "object",
      "properties": {
        "baromabsin": {
          "type": "number"
        },
        "baromrelin": {
          "type": "number"
        },
        "dailyrainin": {
          "type": "number"
        },
        "date": {
          "type": "string",
          "format": "date-time"
        },
        "dateutc": {
          "type": "number"
        },
        "dewPoint": {
          "type": "number"
        },
        "feelsLike": {
          "type": "number"
        },
        "hourlyrainin": {
          "type": "number"
        },
        "humidity": {
          "type": "number"
        },
        "humidityin": {
          "type": "number"
        },
        "maxdailygust": {
          "type": "number"
        },
        "monthlyrainin": {
          "type": "number"
        },
        "tempf": {
          "type": "number"
        },
        "tempinf": {
          "type": "number"
        },
        "winddir": {
          "type": "number"
        },
        "winddir_avg10m": {
          "type": "number"
        },
        "winddir_avg2m": {
          "type": "number"
        },
        "windgustdir": {
          "type": "number"
        },
        "windgustmph": {
          "type": "number"
        },
        "windspdmph_avg10m": {
          "type": "number"
        },
        "windspdmph_avg2m": {
          "type": "number"
        },
        "windspeedmph": {
          "type": "number"
        },
        "yearlyrainin": {
          "type": "number"
        }
      }
    },
    "macAddress": {
      "type": "string"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ListUsersDevicesResponseItem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub info: Option<ListUsersDevicesResponseItemInfo>,
        #[serde(rename = "lastData", default, skip_serializing_if = "Option::is_none")]
        pub last_data: Option<ListUsersDevicesResponseItemLastData>,
        #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
        pub mac_address: Option<String>,
    }
    impl From<&ListUsersDevicesResponseItem> for ListUsersDevicesResponseItem {
        fn from(value: &ListUsersDevicesResponseItem) -> Self {
            value.clone()
        }
    }
    ///ListUsersDevicesResponseItemInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "properties": {
    "location": {
      "type": "string"
    },
    "name": {
      "type": "string"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ListUsersDevicesResponseItemInfo {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl From<&ListUsersDevicesResponseItemInfo> for ListUsersDevicesResponseItemInfo {
        fn from(value: &ListUsersDevicesResponseItemInfo) -> Self {
            value.clone()
        }
    }
    ///ListUsersDevicesResponseItemLastData
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "properties": {
    "baromabsin": {
      "type": "number"
    },
    "baromrelin": {
      "type": "number"
    },
    "dailyrainin": {
      "type": "number"
    },
    "date": {
      "type": "string",
      "format": "date-time"
    },
    "dateutc": {
      "type": "number"
    },
    "dewPoint": {
      "type": "number"
    },
    "feelsLike": {
      "type": "number"
    },
    "hourlyrainin": {
      "type": "number"
    },
    "humidity": {
      "type": "number"
    },
    "humidityin": {
      "type": "number"
    },
    "maxdailygust": {
      "type": "number"
    },
    "monthlyrainin": {
      "type": "number"
    },
    "tempf": {
      "type": "number"
    },
    "tempinf": {
      "type": "number"
    },
    "winddir": {
      "type": "number"
    },
    "winddir_avg10m": {
      "type": "number"
    },
    "winddir_avg2m": {
      "type": "number"
    },
    "windgustdir": {
      "type": "number"
    },
    "windgustmph": {
      "type": "number"
    },
    "windspdmph_avg10m": {
      "type": "number"
    },
    "windspdmph_avg2m": {
      "type": "number"
    },
    "windspeedmph": {
      "type": "number"
    },
    "yearlyrainin": {
      "type": "number"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ListUsersDevicesResponseItemLastData {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub baromabsin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub baromrelin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dailyrainin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dateutc: Option<f64>,
        #[serde(rename = "dewPoint", default, skip_serializing_if = "Option::is_none")]
        pub dew_point: Option<f64>,
        #[serde(rename = "feelsLike", default, skip_serializing_if = "Option::is_none")]
        pub feels_like: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hourlyrainin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub humidity: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub humidityin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maxdailygust: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub monthlyrainin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tempf: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tempinf: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub winddir: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub winddir_avg10m: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub winddir_avg2m: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub windgustdir: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub windgustmph: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub windspdmph_avg10m: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub windspdmph_avg2m: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub windspeedmph: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub yearlyrainin: Option<f64>,
    }
    impl From<&ListUsersDevicesResponseItemLastData>
    for ListUsersDevicesResponseItemLastData {
        fn from(value: &ListUsersDevicesResponseItemLastData) -> Self {
            value.clone()
        }
    }
    ///QueryDeviceDataResponseItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "properties": {
    "baromabsin": {
      "type": "number"
    },
    "baromrelin": {
      "type": "number"
    },
    "dailyrainin": {
      "type": "number"
    },
    "date": {
      "type": "string",
      "format": "date-time"
    },
    "dateutc": {
      "type": "number"
    },
    "dewPoint": {
      "type": "number"
    },
    "feelsLike": {
      "type": "number"
    },
    "hourlyrainin": {
      "type": "number"
    },
    "humidity": {
      "type": "number"
    },
    "humidityin": {
      "type": "number"
    },
    "maxdailygust": {
      "type": "number"
    },
    "monthlyrainin": {
      "type": "number"
    },
    "tempf": {
      "type": "number"
    },
    "tempinf": {
      "type": "number"
    },
    "winddir": {
      "type": "number"
    },
    "winddir_avg10m": {
      "type": "number"
    },
    "winddir_avg2m": {
      "type": "number"
    },
    "windgustdir": {
      "type": "number"
    },
    "windgustmph": {
      "type": "number"
    },
    "windspdmph_avg10m": {
      "type": "number"
    },
    "windspdmph_avg2m": {
      "type": "number"
    },
    "windspeedmph": {
      "type": "number"
    },
    "yearlyrainin": {
      "type": "number"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QueryDeviceDataResponseItem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub baromabsin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub baromrelin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dailyrainin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dateutc: Option<f64>,
        #[serde(rename = "dewPoint", default, skip_serializing_if = "Option::is_none")]
        pub dew_point: Option<f64>,
        #[serde(rename = "feelsLike", default, skip_serializing_if = "Option::is_none")]
        pub feels_like: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hourlyrainin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub humidity: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub humidityin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maxdailygust: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub monthlyrainin: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tempf: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tempinf: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub winddir: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub winddir_avg10m: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub winddir_avg2m: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub windgustdir: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub windgustmph: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub windspdmph_avg10m: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub windspdmph_avg2m: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub windspeedmph: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub yearlyrainin: Option<f64>,
    }
    impl From<&QueryDeviceDataResponseItem> for QueryDeviceDataResponseItem {
        fn from(value: &QueryDeviceDataResponseItem) -> Self {
            value.clone()
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Ambient Weather REST API

Access an Ambient Weather user's weather station data programmatically using our REST API

##### Authentication

Two API Keys are required for all REST API requests:

+ `applicationKey` - identifies the developer / application. To create an application key please login to your AmbientWeather.net account page (https://ambientweather.net/account)

+ `apiKey` - grants access to past/present data for a given user's devices.  A typical consumer-facing application will initially ask the user to create an `apiKey` on their AmbientWeather.net account page (https://ambientweather.net/account) and paste it into the app.  Developers for personal or in-house apps will also need to create an apiKey on their own account page.

##### Rate Limiting

API requests are capped at 1 request/second for each user's apiKey and 3 requests/second per applicationKey.  When this limit is exceeded, the API will return a 429 response code. Please be kind to our servers :)

##### Helper Libraries

+ Node.js - https://github.com/owise1/ambient-weather-api

+ PHP (Laravel) - https://github.com/Jafo232/ambient_api

+ Rust - https://github.com/JoshuaKimsey/ambient-weather-api

+ Go - https://github.com/lrosenman/ambient

+ Python - https://github.com/avryhof/ambient_api

+ Python (asyncio) - https://github.com/bachya/aioambient

+ R - https://github.com/andrewflack/ambientweatheR

+ Java - https://github.com/rsv-code/ambient-weather-java

+ Swift - https://github.com/MikeManzo/SwiftyWeatherKit

+ .NET 5 - https://github.com/ChaseDRedmon/Cirrus

##### Other Resources

+ API Wiki - https://github.com/ambient-weather/api-docs/wiki

+ This documentation's repository - https://github.com/ambient-weather/api-docs

Version: 1.0.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }
    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0.0"
    }
}
impl Client {
    /**List User's Devices

Provides a list of the user's available devices along with each device's most recent data.

Sends a `GET` request to `/v1/devices`

Arguments:
- `api_key`: API Key for user account
- `application_key`: Application Key
*/
    pub async fn list_users_devices<'a>(
        &'a self,
        api_key: &'a str,
        application_key: &'a str,
    ) -> Result<ResponseValue<Vec<types::ListUsersDevicesResponseItem>>, Error<()>> {
        let url = format!("{}/v1/devices", self.baseurl,);
        let mut query = Vec::with_capacity(2usize);
        query.push(("apiKey", api_key.to_string()));
        query.push(("applicationKey", application_key.to_string()));
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            429u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Query Device Data

Fetch data for a given device. Data is stored in 5 or 30 minute increments. A list of all possible fields is here: https://github.com/ambient-weather/api-docs/wiki/Device-Data-Specs

Sends a `GET` request to `/v1/devices/{macAddress}`

Arguments:
- `mac_address`: device Mac Address
- `api_key`: API Key for user account
- `application_key`: Application Key
- `end_date`: The most recent datetime. Results descend from there. If left blank, the most recent results will be returned.  Date format should be in milliseconds since the epoch or string representations outlined here: https://momentjs.com/docs/#/parsing/string/. Note: datetimes are stored in UTC.
- `limit`: The maximum number of results to return (max: 288)
*/
    pub async fn query_device_data<'a>(
        &'a self,
        mac_address: &'a str,
        api_key: &'a str,
        application_key: &'a str,
        end_date: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        limit: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::QueryDeviceDataResponseItem>>, Error<()>> {
        let url = format!(
            "{}/v1/devices/{}", self.baseurl, encode_path(& mac_address.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        query.push(("apiKey", api_key.to_string()));
        query.push(("applicationKey", application_key.to_string()));
        if let Some(v) = &end_date {
            query.push(("endDate", v.to_string()));
        }
        if let Some(v) = &limit {
            query.push(("limit", v.to_string()));
        }
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            429u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
pub mod prelude {
    pub use super::Client;
}
