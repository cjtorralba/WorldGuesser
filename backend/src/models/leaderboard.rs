use std::sync::Arc;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_derive::{Deserialize, Serialize};
use crate::db::Store;
use crate::error::AppError;
use crate::models::user::{LeaderBoardRow, User, UserRankInfo};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LeaderBoard {
    user_list: Vec<LeaderBoardRow>,
}

impl IntoResponse for LeaderBoard {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl LeaderBoard {
    pub fn new(user_list: Vec<LeaderBoardRow>) -> Self {
        Self {
           user_list,
        }
    }


    /// Fetches top num amount of users from the database
    pub async fn populate_top_num_users(num_users: i32, database: Store) -> Result<Self, AppError> {
        let user_list = database.get_top_num_users(num_users).await?;
        Ok(
            Self {
                user_list
            }
        )
    }
}