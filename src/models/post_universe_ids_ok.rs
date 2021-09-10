/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostUniverseIdsOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostUniverseIdsOk {
    /// agents array
    #[serde(rename = "agents", skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<crate::models::PostUniverseIdsAgent>>,
    /// alliances array
    #[serde(rename = "alliances", skip_serializing_if = "Option::is_none")]
    pub alliances: Option<Vec<crate::models::PostUniverseIdsAlliance>>,
    /// characters array
    #[serde(rename = "characters", skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<crate::models::PostUniverseIdsCharacter>>,
    /// constellations array
    #[serde(rename = "constellations", skip_serializing_if = "Option::is_none")]
    pub constellations: Option<Vec<crate::models::PostUniverseIdsConstellation>>,
    /// corporations array
    #[serde(rename = "corporations", skip_serializing_if = "Option::is_none")]
    pub corporations: Option<Vec<crate::models::PostUniverseIdsCorporation>>,
    /// factions array
    #[serde(rename = "factions", skip_serializing_if = "Option::is_none")]
    pub factions: Option<Vec<crate::models::PostUniverseIdsFaction>>,
    /// inventory_types array
    #[serde(rename = "inventory_types", skip_serializing_if = "Option::is_none")]
    pub inventory_types: Option<Vec<crate::models::PostUniverseIdsInventoryType>>,
    /// regions array
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<crate::models::PostUniverseIdsRegion>>,
    /// stations array
    #[serde(rename = "stations", skip_serializing_if = "Option::is_none")]
    pub stations: Option<Vec<crate::models::PostUniverseIdsStation>>,
    /// systems array
    #[serde(rename = "systems", skip_serializing_if = "Option::is_none")]
    pub systems: Option<Vec<crate::models::PostUniverseIdsSystem>>,
}

impl PostUniverseIdsOk {
    /// 200 ok object
    pub fn new() -> PostUniverseIdsOk {
        PostUniverseIdsOk {
            agents: None,
            alliances: None,
            characters: None,
            constellations: None,
            corporations: None,
            factions: None,
            inventory_types: None,
            regions: None,
            stations: None,
            systems: None,
        }
    }
}

