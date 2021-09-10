/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdMail200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdMail200Ok {
    /// From whom the mail was sent
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    /// is_read boolean
    #[serde(rename = "is_read", skip_serializing_if = "Option::is_none")]
    pub is_read: Option<bool>,
    /// labels array
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<i32>>,
    /// mail_id integer
    #[serde(rename = "mail_id", skip_serializing_if = "Option::is_none")]
    pub mail_id: Option<i32>,
    /// Recipients of the mail
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<crate::models::GetCharactersCharacterIdMailRecipient>>,
    /// Mail subject
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// When the mail was sent
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl GetCharactersCharacterIdMail200Ok {
    /// 200 ok object
    pub fn new() -> GetCharactersCharacterIdMail200Ok {
        GetCharactersCharacterIdMail200Ok {
            from: None,
            is_read: None,
            labels: None,
            mail_id: None,
            recipients: None,
            subject: None,
            timestamp: None,
        }
    }
}

