/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsCharactersKills : Top 100 rankings of pilots by number of kills from yesterday, last week and in total



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCharactersKills {
    /// Top 100 ranking of pilots active in faction warfare by total kills. A pilot is considered \"active\" if they have participated in faction warfare in the past 14 days
    #[serde(rename = "active_total")]
    pub active_total: Vec<crate::models::GetFwLeaderboardsCharactersActiveTotalActiveTotal>,
    /// Top 100 ranking of pilots by kills in the past week
    #[serde(rename = "last_week")]
    pub last_week: Vec<crate::models::GetFwLeaderboardsCharactersLastWeekLastWeek>,
    /// Top 100 ranking of pilots by kills in the past day
    #[serde(rename = "yesterday")]
    pub yesterday: Vec<crate::models::GetFwLeaderboardsCharactersYesterdayYesterday>,
}

impl GetFwLeaderboardsCharactersKills {
    /// Top 100 rankings of pilots by number of kills from yesterday, last week and in total
    pub fn new(active_total: Vec<crate::models::GetFwLeaderboardsCharactersActiveTotalActiveTotal>, last_week: Vec<crate::models::GetFwLeaderboardsCharactersLastWeekLastWeek>, yesterday: Vec<crate::models::GetFwLeaderboardsCharactersYesterdayYesterday>) -> GetFwLeaderboardsCharactersKills {
        GetFwLeaderboardsCharactersKills {
            active_total,
            last_week,
            yesterday,
        }
    }
}

