/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseSchematicsSchematicIdNotFound : Schematic not found



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUniverseSchematicsSchematicIdNotFound {
    /// error message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetUniverseSchematicsSchematicIdNotFound {
    /// Schematic not found
    pub fn new() -> GetUniverseSchematicsSchematicIdNotFound {
        GetUniverseSchematicsSchematicIdNotFound {
            error: None,
        }
    }
}

