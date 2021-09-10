/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostUniverseIdsCorporation : corporation object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostUniverseIdsCorporation {
    /// id integer
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PostUniverseIdsCorporation {
    /// corporation object
    pub fn new() -> PostUniverseIdsCorporation {
        PostUniverseIdsCorporation {
            id: None,
            name: None,
        }
    }
}

