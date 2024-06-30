use crate::driver::Database;
use sqlx::{FromRow, Result};
use tracing::instrument;

#[derive(Debug, FromRow)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

impl User {
    #[instrument]
    pub async fn new(
        driver: &Database,
        user_id: String,
        name: String,
        email: String,
        is_admin: bool,
    ) -> Result<Self> {
        sqlx::query("INSERT INTO users (user_id, name, email, is_admin) VALUES (?, ?, ?, ?)")
            .bind(&user_id)
            .bind(&name)
            .bind(&email)
            .bind(is_admin)
            .execute(&**driver)
            .await?;

        Ok(Self {
            name,
            user_id,
            email,
            is_admin,
        })
    }

    #[instrument]
    pub async fn get_by_id(driver: &Database, id: &str) -> Result<Option<Self>> {
        Ok(sqlx::query_as("SELECT * FROM users WHERE user_id = ?")
            .bind(id)
            .fetch_optional(&**driver)
            .await?)
    }

    #[instrument]
    pub async fn list(driver: &Database) -> Result<Vec<Self>> {
        Ok(sqlx::query_as("SELECT * FROM users")
            .fetch_all(&**driver)
            .await?)
    }

    #[instrument]
    pub async fn list_permitted_scopes(&self, driver: &Database) -> Result<Vec<String>> {
        Ok(
            sqlx::query_scalar("SELECT scope FROM user_permitted_scopes WHERE user_id = ?")
                .bind(&self.user_id)
                .fetch_all(&**driver)
                .await?,
        )
    }

    #[instrument]
    pub async fn remove_permitted_scope(&self, driver: &Database, scope: &str) -> Result<()> {
        sqlx::query("DELETE FROM user_permitted_scopes WHERE user_id = ? AND scope = ?")
            .bind(&self.user_id)
            .bind(scope)
            .execute(&**driver)
            .await?;

        Ok(())
    }

    #[instrument]
    pub async fn grant_permitted_scope(&self, driver: &Database, scope: &str) -> Result<()> {
        sqlx::query("INSERT INTO user_permitted_scopes (user_id, scope) VALUES (?, ?)")
            .bind(&self.user_id)
            .bind(scope)
            .execute(&**driver)
            .await?;

        Ok(())
    }
}
