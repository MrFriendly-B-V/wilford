use crate::driver::Database;
use crate::generate_string;
use sqlx::FromRow;
use sqlx::Result;
use tracing::instrument;

#[derive(Debug, Clone, FromRow)]
pub struct ConstantAccessToken {
    pub name: String,
    pub token: String,
}

impl ConstantAccessToken {
    fn generate_token() -> String {
        generate_string(32)
    }

    #[instrument]
    pub async fn new(driver: &Database, name: String) -> Result<Self> {
        let token = Self::generate_token();

        sqlx::query("INSERT INTO constant_access_tokens (name, token) VALUES (?, ?)")
            .bind(&name)
            .bind(&token)
            .execute(&**driver)
            .await?;

        Ok(Self { name, token })
    }

    #[instrument]
    pub async fn list(driver: &Database) -> Result<Vec<Self>> {
        Ok(sqlx::query_as("SELECT * FROM constant_access_tokens")
            .fetch_all(&**driver)
            .await?)
    }

    #[instrument]
    pub async fn get_by_token(driver: &Database, token: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as("SELECT * FROM constant_access_tokens WHERE token = ?")
                .bind(token)
                .fetch_optional(&**driver)
                .await?,
        )
    }

    #[instrument]
    pub async fn revoke(self, driver: &Database) -> Result<()> {
        sqlx::query("DELETE FROM constant_access_tokens WHERE token = ?")
            .bind(self.token)
            .execute(&**driver)
            .await?;
        Ok(())
    }
}
