use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, query_as};

use crate::db;
use crate::utils::error::{DatabaseError, ModelResult};

#[derive(FromRow)]
pub struct UserModel {
    id: i64,
    username: String,
    email: String,
    password: String,
    created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct UserCreation {
    username: String,
    password: String,
    email: String,
}

#[derive(serde::Deserialize)]
pub struct UserUpdate {
    username: Option<String>,
    email: Option<String>,
    old_password: Option<String>,
    new_password: Option<String>,
}

#[derive(Serialize)]
pub struct UserResult {
    id: i64,
    username: String,
    created_at: NaiveDateTime,
}

impl UserModel {
    pub async fn create_new(creation: UserCreation) -> ModelResult<Self> {
        let user = query_as!(
            Self,
            r#"
                INSERT INTO users (
                    username,
                    email,
                    password
                )
                VALUES (
                    $1, 
                    $2,
                    crypt($3, gen_salt('bf', 8))
                )
                RETURNING *
            "#,
            creation.username,
            creation.email,
            creation.password
        )
        .fetch_one(db!())
        .await?;

        Ok(user)
    }

    pub async fn get(id: i64) -> ModelResult<Self> {
        let user = query_as!(
            Self,
            r#"
                SELECT *
                FROM users
                WHERE
                    id = $1
            "#,
            id
        )
        .fetch_one(db!())
        .await?;
        Ok(user)
    }

    pub async fn edit(&self, update: UserUpdate) -> ModelResult<Self> {
        query_as!(
            Self,
            r#"
                UPDATE users
                SET
                    username = COALESCE($1, username),
                    email = COALESCE($2, email),
                    password = CASE
                        WHEN $3::TEXT IS NOT NULL
                            AND (password IS NULL OR crypt($4::TEXT, password) = password)
                        THEN crypt($3::TEXT, gen_salt('bf'))
                        ELSE password
                    END
                WHERE
                    id = $5
                AND 
                    ($3::TEXT IS NULL OR crypt($3::TEXT, password) = password)
                RETURNING *
            "#,
            update.username,
            update.email,
            update.old_password,
            update.new_password,
            self.id
        )
        .fetch_optional(db!())
        .await?
        .ok_or(DatabaseError::ModelNotFound("user"))
    }

    pub fn into_result(&self) -> UserResult {
        UserResult {
            id: self.id.clone(),
            username: self.username.clone(),
            created_at: self.created_at,
        }
    }
}
