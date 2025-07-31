use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema, Debug)]
pub struct AuthResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub token: String,
}