/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdStandings200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdStandings200Ok {
    /// from_id integer
    #[serde(rename = "from_id")]
    pub from_id: i32,
    /// from_type string
    #[serde(rename = "from_type")]
    pub from_type: FromType,
    /// standing number
    #[serde(rename = "standing")]
    pub standing: f32,
}

impl GetCorporationsCorporationIdStandings200Ok {
    /// 200 ok object
    pub fn new(from_id: i32, from_type: FromType, standing: f32) -> GetCorporationsCorporationIdStandings200Ok {
        GetCorporationsCorporationIdStandings200Ok {
            from_id,
            from_type,
            standing,
        }
    }
}

/// from_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FromType {
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "npc_corp")]
    NpcCorp,
    #[serde(rename = "faction")]
    Faction,
}
