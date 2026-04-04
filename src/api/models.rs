#![allow(dead_code)]

use serde::{Deserialize, Serialize};

fn default_priority() -> String {
    "medium".to_string()
}

// --- Auth ---

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub user: AuthUser,
}

#[derive(Debug, Deserialize)]
pub struct AuthUser {
    pub login: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthTokenRequest {
    pub github_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

// --- Todo ---

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: String,
    pub created: String,
    pub updated: String,
    pub text: String,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub completed: bool,
    #[serde(default = "default_priority")]
    pub priority: String,
    #[serde(rename = "dueDate")]
    pub due_date: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "goalIds", default)]
    pub goal_ids: Vec<String>,
    pub order: serde_json::Value,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateTodoRequest {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "goalIds", skip_serializing_if = "Vec::is_empty")]
    pub goal_ids: Vec<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateTodoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}

// --- Goal ---

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Goal {
    pub id: String,
    pub created: String,
    pub updated: String,
    pub text: String,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub completed: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    pub order: serde_json::Value,
    pub deleted: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateGoalRequest {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateGoalRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}

// --- Vision ---

#[derive(Debug, Deserialize, Serialize)]
pub struct Vision {
    pub description: String,
    #[serde(rename = "hasImage")]
    pub has_image: bool,
    #[serde(rename = "imageTimestamp")]
    pub image_timestamp: Option<String>,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateVisionRequest {
    pub description: String,
}

// --- Reorder ---

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ReorderPosition {
    Named(String), // "top" or "bottom"
    After { after: String },
}

#[derive(Debug, Serialize)]
pub struct ReorderRequest {
    pub position: ReorderPosition,
}

// --- Bulk tag ---

#[derive(Debug, Serialize)]
pub struct BulkTagItem {
    pub id: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct BulkTagRequest {
    pub updates: Vec<BulkTagItem>,
}

#[derive(Debug, Deserialize)]
pub struct BulkTagResponse<T> {
    pub success: bool,
    pub updated: u32,
    pub items: Vec<T>,
}

// --- Generic delete response ---

#[derive(Debug, Deserialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: String,
}
