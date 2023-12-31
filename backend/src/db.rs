use axum::Json;
use serde_json::Value;

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use tracing::{error, info};
use crate::AppResult;

use crate::error::AppError;
use crate::models::location::Location;
use crate::models::user::{LoggedInUser, User, UserRankInfo, UserForClaims, UserSignup, LeaderBoardRow};

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

    pub async fn get_user(&self, email: &str) -> Result<Option<UserForClaims>, AppError> {


        // FETCHING FROM DB
        let user = sqlx::query_as::<_, UserForClaims>(
            r#"
                SELECT id, email, password FROM user_creds WHERE email = $1
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

        // INTSERTING INTO DB
        let result = sqlx::query("INSERT INTO user_creds(email, password) values ($1, $2)")
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
    pub async fn get_top_num_users(&self, num_users: i32) -> Result<Vec<LeaderBoardRow>, AppError> {
        let mut rows = sqlx::query_as::<_, LeaderBoardRow>(
            r#"
                SELECT id, rank, total_score, num_guesses FROM user_ranks WHERE rank > 0 AND rank <= $1
            "#)
            .bind(num_users)
            .fetch_all(&self.conn_pool)
            .await?;

        rows.sort_by(|a, b| a.rank.cmp(&b.rank));

        info!("Top users: {:?}", rows);

        Ok(rows)
    }





/// Updates and reorganizes the user_ranks table
/// # Returns
/// * [Result]<(), [AppError]>
///
/// #
pub async fn update_score(&self, score: f32, id: i32) -> AppResult<()> {

    info!("Updating score table");
    // Updating the table and organizing it
    let res = sqlx::query(
        r#"
            UPDATE user_ranks SET total_score = total_score + $1, num_guesses = num_guesses + 1 WHERE id = $2
            "#
    )
        .bind(score)
        .bind(id)
        .execute(&self.conn_pool)
        .await?;
    //TODO: determine if we need to comment or uncomment this as the rows affect by this query could be more than one
    let rows_affected = res.rows_affected();
    if rows_affected < 1 || rows_affected > 1 { // Should only ever affect one row, no more no less
        return Err(AppError::InternalServerError);
    }

    sqlx::query(
        r#"
            UPDATE user_ranks AS t SET rank = r.ranking from (
                SELECT id, DENSE_RANK() OVER (ORDER BY total_score DESC) AS ranking FROM user_ranks
            )
            AS r WHERE t.id = r.id
            "#
    )
        .execute(&self.conn_pool)
        .await?;

    Ok(())
}

/*
    /// TODO: somehow make this more efficient? Maybe use the current user rank and only look at things above it since it will never go below?
    pub async fn get_rank_from_score(&self, score: i32) -> Result<i32, AppError> {
        if rank <= 0 {
            return Ok(0);
        }

        // Getting the current user information from the rank
        // This includes the UID, rank, num_guesses, and score
        let res = sqlx::query_as!(UserAndScore,
            r#"
                SELECT * FROM user_ranks WHERE rank = $1 AND rank != 0
            "#,
            &rank
        )
            .fetch_all(&self.conn_pool)
            .await
            .map_err(|_| AppError::InternalServerError)?;



        Ok(1)

 */
/*
        let mut user_rank_vec: Vec<UserAndScore> = Vec::new();

        let filtered_rows = res
            .iter()
            .filter(|row| {
               row.get("rank") != 0
            })
            .collect();

    }
 */
/*
    pub async fn get_user_ranks(&self, user: LoggedInUser) -> Result<UserAndRank, AppError> {
        let res = sqlx::query(
            r#"
                    SELECT * FROM user_ranks WHERE id = $1
               "#
        )
            .bind(user.token.id)
            .fetch_optional(&self.conn_pool)
            .await
            .map_err(|_| AppError::UserDoesNotExist)?;

        if let Ok(row) = res {
            let user_rank = UserAndRank {
                email: row.get("email"),
                rank: row.get("rank"),
            };

            return Ok(user_rank);
        } else {
            return Err(AppError::UserDoesNotExist);
        }

    }

 */
/*
    pub async fn update_user_rank(&self, user: LoggedInUser) -> Result<(), AppError> {
        let user_rank = self.get_user_ranks(user).await?;

        let res = sqlx::query(
            r#"
                UPDATE user_ranks(id, rank, total_score, num_guesses) WITH VALUES($1, $2, $3, $4)
            "#
        )
            .bind(&user.token.id)
            .bind(&user_rank.rank)
            .bind(&user_rank.)


        Ok(())
    }
 */

}


#[cfg(test)]
mod tests {
    use super::*;

    async fn setup() -> Result<PgPool, sqlx::Error> {

        todo!()
    }


}
