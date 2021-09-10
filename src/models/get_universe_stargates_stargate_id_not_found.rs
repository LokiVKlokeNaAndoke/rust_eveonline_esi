/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseStargatesStargateIdNotFound : Not found



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUniverseStargatesStargateIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetUniverseStargatesStargateIdNotFound {
    /// Not found
    pub fn new() -> GetUniverseStargatesStargateIdNotFound {
        GetUniverseStargatesStargateIdNotFound {
            error: None,
        }
    }
}

