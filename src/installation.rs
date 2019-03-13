use chrono::prelude::*;

use crate::Account;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Installation {
    pub id: usize,
    pub account: Account,
    pub repository_selection: String, // or Enum?
    pub access_tokens_url: String,
    pub repositories_url: String,
    pub html_url: String,
    pub app_id: usize,
    pub target_id: usize,
    pub target_type: String, // or Enum?
    pub permissions: Permissions,
    pub events: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub single_file_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Permissions {
    pub pull_requests: String, // Enum
    pub metadata: String, // Enum
}