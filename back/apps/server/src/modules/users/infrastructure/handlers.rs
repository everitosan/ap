use actix_web::{web, HttpResponse, ResponseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::modules::users::application;
use crate::modules::users::infrastructure::PostgresUserRepository;
use crate::shared::{ApiResponse, AppState, AuthSession};

/// Custom deserializer that accepts both strings and integers
/// Accepts: ["1", "2", "3"] or [1, 2, 3]
fn deserialize_flexible_i32_vec<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, SeqAccess, Visitor};
    use std::fmt;

    struct FlexibleI32VecVisitor;

    impl<'de> Visitor<'de> for FlexibleI32VecVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of integers or strings")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<serde_json::Value>()? {
                let num = match value {
                    serde_json::Value::Number(n) => n
                        .as_i64()
                        .ok_or_else(|| de::Error::custom("invalid number"))? as i32,
                    serde_json::Value::String(s) => s
                        .parse::<i32>()
                        .map_err(|_| de::Error::custom(format!("invalid number string: {}", s)))?,
                    _ => return Err(de::Error::custom("expected number or string")),
                };
                vec.push(num);
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_seq(FlexibleI32VecVisitor)
}

#[derive(Serialize)]
pub struct UserResponse {
    pub username: Option<String>,
    pub topics: Vec<i32>,  // Array of topic IDs
    pub created: DateTime<Utc>,
    pub filled_address: bool,
}

/// GET /api/v1/user
pub async fn get_user(
    session: AuthSession,
    state: web::Data<AppState>,
) -> HttpResponse {
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());

    match application::get_user(session.user_id, &user_repo).await {
        Ok(user) => {
            let response = UserResponse {
                username: user.username,
                topics: user.topics,
                created: user.created,
                filled_address: user.address.is_some(),
            };
            HttpResponse::Ok().json(ApiResponse::new(response))
        }
        Err(e) => e.error_response(),
    }
}

#[derive(Deserialize)]
pub struct FillProfileRequest {
    username: String,
    #[serde(deserialize_with = "deserialize_flexible_i32_vec")]
    topics: Vec<i32>,  // Accepts both ["1", "2"] and [1, 2]
}

/// POST /api/v1/fill-profile
pub async fn fill_profile(
    session: AuthSession,
    state: web::Data<AppState>,
    body: web::Json<FillProfileRequest>,
) -> HttpResponse {
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());

    match application::update_profile(session.user_id, &body.username, &body.topics, &user_repo).await {
        Ok(()) => HttpResponse::Ok().json(ApiResponse::new("Profile updated")),
        Err(e) => e.error_response(),
    }
}

#[derive(Deserialize)]
pub struct FillAddressRequest {
    street: String,
    int_number: String,
    postal_code: String,
    state: String,
    city: String,
    colony: String,
}

/// POST /api/v1/fill-address
pub async fn fill_address(
    session: AuthSession,
    state: web::Data<AppState>,
    body: web::Json<FillAddressRequest>,
) -> HttpResponse {
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());

    let address = serde_json::json!({
        "street": body.street,
        "int_number": body.int_number,
        "postal_code": body.postal_code,
        "state": body.state,
        "city": body.city,
        "colony": body.colony,
    });

    match application::update_address(session.user_id, &address, &user_repo).await {
        Ok(()) => HttpResponse::Ok().json(ApiResponse::new("Address updated")),
        Err(e) => e.error_response(),
    }
}
