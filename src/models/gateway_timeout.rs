/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GatewayTimeout : Gateway timeout model



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayTimeout {
    /// Gateway timeout message
    #[serde(rename = "error")]
    pub error: String,
    /// number of seconds the request was given
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

impl GatewayTimeout {
    /// Gateway timeout model
    pub fn new(error: String) -> GatewayTimeout {
        GatewayTimeout {
            error,
            timeout: None,
        }
    }
}

