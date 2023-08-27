use axum::Json;
use serde_json::Value;

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use tracing::{error, info};

use crate::error::AppError;
use crate::models::user::{User, UserAndRank, UserSignup};

#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
}

pub async fn new_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

impl Store {
    pub fn with_pool(pool: PgPool) -> Self {
        Self { conn_pool: pool }
    }

    pub async fn test_database(&self) -> Result<(), sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.conn_pool)
            .await?;

        info!("{}", &row.0);

        assert_eq!(row.0, 150);
        Ok(())
    }

    pub async fn get_user(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
                SELECT email, password FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.conn_pool)
        .await
        .map_err(|f| {
            error!("Error!: {}", f.to_string());
            AppError::Any(anyhow::anyhow!(f))
        })?;

        info!("Query: {:?}", user);

        Ok(user)
    }

    pub async fn create_user(&self, user: UserSignup) -> Result<Json<Value>, AppError> {
        // TODO: Encrypt/bcrypt user passwords
        let result = sqlx::query("INSERT INTO users(email, password) values ($1, $2)")
            .bind(&user.email)
            .bind(&user.password)
            .execute(&self.conn_pool)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if result.rows_affected() < 1 {
            Err(AppError::InternalServerError)
        } else {
            Ok(Json(
                serde_json::json!({"message": "User created successfully!"}),
            ))
        }
    }

    /// Gets the top 100 users in the database by ranking. If there are less than 100 users, it will get how many it can.
    /// # Returns:
    /// Result<[Vec]<[UserAndRank]>, [AppError]>
    pub async fn get_top_num_users(&self, num_users: i32) -> Result<Vec<UserAndRank>, AppError> {
        let rows = sqlx::query(
            r#"
                SELECT * FROM users WHERE rank > 0 AND rank <= $1
            "#)
            .bind(num_users)
            .fetch_all(&self.conn_pool)
            .await?;


        let mut user_vec: Vec<UserAndRank> = Vec::new();

        for row in rows {
            let user = UserAndRank {
                email: row.get("email"),
                rank: row.get("rank"),
            };
            user_vec.push(user);
        }
        Ok(user_vec)
    }

}
