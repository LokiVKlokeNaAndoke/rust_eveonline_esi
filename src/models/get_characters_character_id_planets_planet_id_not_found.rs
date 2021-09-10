/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdPlanetsPlanetIdNotFound : Colony not found



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdNotFound {
    /// error message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetCharactersCharacterIdPlanetsPlanetIdNotFound {
    /// Colony not found
    pub fn new() -> GetCharactersCharacterIdPlanetsPlanetIdNotFound {
        GetCharactersCharacterIdPlanetsPlanetIdNotFound {
            error: None,
        }
    }
}

