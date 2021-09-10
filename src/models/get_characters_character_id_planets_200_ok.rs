/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdPlanets200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanets200Ok {
    /// last_update string
    #[serde(rename = "last_update")]
    pub last_update: String,
    /// num_pins integer
    #[serde(rename = "num_pins")]
    pub num_pins: i32,
    /// owner_id integer
    #[serde(rename = "owner_id")]
    pub owner_id: i32,
    /// planet_id integer
    #[serde(rename = "planet_id")]
    pub planet_id: i32,
    /// planet_type string
    #[serde(rename = "planet_type")]
    pub planet_type: PlanetType,
    /// solar_system_id integer
    #[serde(rename = "solar_system_id")]
    pub solar_system_id: i32,
    /// upgrade_level integer
    #[serde(rename = "upgrade_level")]
    pub upgrade_level: i32,
}

impl GetCharactersCharacterIdPlanets200Ok {
    /// 200 ok object
    pub fn new(last_update: String, num_pins: i32, owner_id: i32, planet_id: i32, planet_type: PlanetType, solar_system_id: i32, upgrade_level: i32) -> GetCharactersCharacterIdPlanets200Ok {
        GetCharactersCharacterIdPlanets200Ok {
            last_update,
            num_pins,
            owner_id,
            planet_id,
            planet_type,
            solar_system_id,
            upgrade_level,
        }
    }
}

/// planet_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlanetType {
    #[serde(rename = "temperate")]
    Temperate,
    #[serde(rename = "barren")]
    Barren,
    #[serde(rename = "oceanic")]
    Oceanic,
    #[serde(rename = "ice")]
    Ice,
    #[serde(rename = "gas")]
    Gas,
    #[serde(rename = "lava")]
    Lava,
    #[serde(rename = "storm")]
    Storm,
    #[serde(rename = "plasma")]
    Plasma,
}
