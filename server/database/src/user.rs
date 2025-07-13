use crate::driver::Database;
use crate::impl_enum_type;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, FromRow, Result};
use std::fmt::Debug;
use thiserror::Error;
use time::OffsetDateTime;
use tracing::instrument;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub locale: Locale,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserEmail {
    pub user_id: String,
    pub registered_at: i64,
    pub address: String,
    pub verified: bool,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserEmailVerification {
    pub user_id: String,
    pub address: String,
    pub verification_code: String,
}

/// The preferred language of the user
#[derive(Debug, Clone, Copy, Encode, Decode, Deserialize, Serialize)]
pub enum Locale {
    En,
    Nl,
}

impl_enum_type!(Locale);

#[derive(Debug, Error)]
pub enum SetEmailAddressError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(
        "The email address to be set is not registered in the user_emails table or is not verified"
    )]
    NoEmail,
}

impl User {
    /// Create a new user
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn new(
        driver: &Database,
        user_id: String,
        name: String,
        email: String,
        is_admin: bool,
        locale: Locale,
        requires_email_verification: bool,
    ) -> Result<(Self, Option<UserEmailVerification>)> {
        let mut tx = driver.begin().await?;

        sqlx::query(
            "INSERT INTO users (user_id, name, email, is_admin, locale) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&user_id)
        .bind(&name)
        .bind(&email)
        .bind(is_admin)
        .bind(locale)
        .execute(&mut *tx)
        .await?;

        // We keep a record of which email addresses have been used in the past
        sqlx::query("INSERT INTO user_emails (user_id, registered_at, address, verified) VALUES (?, ?, ?, ?)")
            .bind(&user_id)
            .bind(current_time())
            .bind(&email)
            .bind(!requires_email_verification) // If it is required, the email address is not verified and it will need to be verified by the user
            .execute(&mut *tx)
            .await?;

        // If required, generate a verification code
        let verification = if requires_email_verification {
            let verification_code = gen_email_verification_code();

            sqlx::query("INSERT INTO user_email_verifications (user_id, address, verification_code) VALUES (?, ?, ?)")
                .bind(&user_id)
                .bind(&email)
                .bind(&verification_code)
                .execute(&mut *tx)
                .await?;

            Some(UserEmailVerification {
                user_id: user_id.clone(),
                address: email.clone(),
                verification_code,
            })
        } else {
            None
        };

        let user = Self {
            name,
            user_id,
            email,
            is_admin,
            locale,
        };

        tx.commit().await?;

        Ok((user, verification))
    }

    /// Get a user by their ID
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn get_by_id(driver: &Database, id: &str) -> Result<Option<Self>> {
        sqlx::query_as("SELECT * FROM users WHERE user_id = ?")
            .bind(id)
            .fetch_optional(&**driver)
            .await
    }

    /// Get a user by their email address
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn get_by_email(driver: &Database, email: &str) -> Result<Option<Self>> {
        sqlx::query_as("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&**driver)
            .await
    }

    /// Get the total number of users
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn count(driver: &Database) -> Result<i64> {
        sqlx::query_scalar("SELECT COUNT(1) FROM users")
            .fetch_one(&**driver)
            .await
    }

    /// Get a list of all users
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn list(driver: &Database) -> Result<Vec<Self>> {
        sqlx::query_as("SELECT * FROM users")
            .fetch_all(&**driver)
            .await
    }

    /// List the scopes a user is permitted to request
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn list_permitted_scopes(&self, driver: &Database) -> Result<Vec<String>> {
        sqlx::query_scalar("SELECT scope FROM user_permitted_scopes WHERE user_id = ?")
            .bind(&self.user_id)
            .fetch_all(&**driver)
            .await
    }

    /// Remove a scope that the user was permitted to request.
    /// Does not fail if the user did not have the scope specified.
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn remove_permitted_scope(&self, driver: &Database, scope: &str) -> Result<()> {
        sqlx::query("DELETE FROM user_permitted_scopes WHERE user_id = ? AND scope = ?")
            .bind(&self.user_id)
            .bind(scope)
            .execute(&**driver)
            .await?;

        Ok(())
    }

    /// Grant a scope to the user. They can now request tokens with the scope.
    ///
    /// # Errors
    ///
    /// - If the query fails
    /// - If the scope was already granted
    #[instrument(skip(driver))]
    pub async fn grant_permitted_scope(&self, driver: &Database, scope: &str) -> Result<()> {
        sqlx::query("INSERT INTO user_permitted_scopes (user_id, scope) VALUES (?, ?)")
            .bind(&self.user_id)
            .bind(scope)
            .execute(&**driver)
            .await?;

        Ok(())
    }

    /// Set if the user is an administrative user
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn set_is_admin(&mut self, driver: &Database, is_admin: bool) -> Result<()> {
        sqlx::query("UPDATE users SET is_admin = ? WHERE user_id = ?")
            .bind(is_admin)
            .bind(&self.user_id)
            .execute(&**driver)
            .await?;

        self.is_admin = is_admin;

        Ok(())
    }

    /// Set the name of the user.
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn set_name(&mut self, driver: &Database, name: &str) -> Result<()> {
        sqlx::query("UPDATE users SET name = ? WHERE user_id = ?")
            .bind(name)
            .bind(&self.user_id)
            .execute(&**driver)
            .await?;

        self.name = name.to_string();

        Ok(())
    }

    /// Set the email address for the user.
    /// This should be called only after the new email address has been verified.
    ///
    /// In general, the flow for changing the email address should look like this:
    /// 1. Call to [Self::update_email]
    /// 2. Let user verify email with provided code
    /// 3. Call to [Self::set_email_verified]
    /// 4. Call to this function
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn set_email(
        &mut self,
        driver: &Database,
        email: &str,
    ) -> std::result::Result<(), SetEmailAddressError> {
        // Check the email is in the table of user emails
        let num_addr: i64 = sqlx::query_scalar(
            "SELECT COUNT(1) FROM user_emails WHERE user_id = ? AND address = ? AND verified = ?",
        )
        .bind(&self.user_id)
        .bind(email)
        .bind(true)
        .fetch_one(&**driver)
        .await?;

        // We do not allow changing the address except through the proper flow as described above.
        if num_addr == 0 {
            return Err(SetEmailAddressError::NoEmail);
        }

        // We can now update the current email
        sqlx::query("UPDATE users SET email = ? WHERE user_id = ?")
            .bind(email)
            .bind(&self.user_id)
            .execute(&**driver)
            .await?;

        self.email = email.to_string();

        Ok(())
    }

    /// Set the locale of the user.
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn set_locale(&mut self, driver: &Database, new_locale: Locale) -> Result<()> {
        sqlx::query("UPDATE users set locale = ? WHERE user_id = ?")
            .bind(new_locale)
            .bind(&self.user_id)
            .execute(&**driver)
            .await?;

        self.locale = new_locale;

        Ok(())
    }

    /// Set the password hash for the user.
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver, password))]
    pub async fn set_password_hash<P: AsRef<str> + Debug>(
        &self,
        driver: &Database,
        password: P,
        require_change: bool,
    ) -> Result<()> {
        if self.get_password_hash(driver).await?.is_some() {
            sqlx::query("UPDATE user_credentials SET password_hash = ?, change_required = ? WHERE user_id = ?")
                .bind(password.as_ref())
                .bind(require_change)
                .bind(&self.user_id)
                .execute(&**driver)
                .await?;
        } else {
            sqlx::query("INSERT INTO user_credentials (user_id, password_hash, change_required) VALUES (?, ?, ?)")
                .bind(&self.user_id)
                .bind(password.as_ref())
                .bind(require_change)
                .execute(&**driver)
                .await?;
        }

        Ok(())
    }

    /// Get the password hash for the user
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn get_password_hash(&self, driver: &Database) -> Result<Option<String>> {
        sqlx::query_scalar("SELECT password_hash FROM user_credentials WHERE user_id = ?")
            .bind(&self.user_id)
            .fetch_optional(&**driver)
            .await
    }

    /// Check whether a password change is required for the user.
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn password_change_required(&self, driver: &Database) -> Result<Option<bool>> {
        sqlx::query_scalar("SELECT change_required FROM user_credentials WHERE user_id = ?")
            .bind(&self.user_id)
            .fetch_optional(&**driver)
            .await
    }

    /// Register an email address update.
    /// This will add the email address to the list of user emails, but not have it be verified yet.
    /// It will create a pending email verification. Once the email has been verified, it can be applied with [Self::set_email].
    ///
    /// # Errors
    ///
    /// If the query fails
    pub async fn update_email(
        &self,
        driver: &Database,
        new_address: String,
    ) -> Result<UserEmailVerification> {
        let mut tx = driver.begin().await?;

        // Insert into email table
        sqlx::query("INSERT INTO user_emails (user_id, registered_at, address, verified) VALUES (?, ?, ?, ?)")
            .bind(&self.user_id)
            .bind(current_time())
            .bind(&new_address)
            .bind(false)
            .execute(&mut *tx)
            .await?;

        // Create verification code
        let verification_code = gen_email_verification_code();
        sqlx::query("INSERT INTO user_email_verifications (user_id, address, verification_code) VALUES (?, ?, ?)")
            .bind(&self.user_id)
            .bind(&new_address)
            .bind(&verification_code)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(UserEmailVerification {
            user_id: self.user_id.clone(),
            verification_code,
            address: new_address,
        })
    }

    /// Mark an email address as verified. Does not make it the current
    /// email address of the user. For that, use [Self::set_email]
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn set_email_verified(
        &self,
        driver: &Database,
        email: &str,
        verified: bool,
    ) -> Result<()> {
        sqlx::query("UPDATE user_emails SET verified = ? WHERE user_id = ? AND address = ?")
            .bind(verified)
            .bind(&self.user_id)
            .bind(email)
            .execute(&**driver)
            .await?;

        Ok(())
    }

    /// Remove the verification code of an email address
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn remove_email_verifcation_code(
        &self,
        driver: &Database,
        email: &str,
        verification_code: &str,
    ) -> Result<()> {
        sqlx::query("DELETE FROM user_email_verifications WHERE user_id = ? AND address = ? AND verification_code = ?")
            .bind(&self.user_id)
            .bind(email)
            .bind(verification_code)
            .execute(&**driver)
            .await?;

        Ok(())
    }

    /// Fetch an unverified email address by the verification provided to the user.
    ///
    /// # Errors
    ///
    /// If the query fails
    #[instrument(skip(driver))]
    pub async fn get_address_by_verification_code(
        &self,
        verifcation_code: &str,
        driver: &Database,
    ) -> Result<Option<UserEmailVerification>> {
        sqlx::query_as(
            "SELECT * FROM user_email_verifications WHERE user_id = ? AND verification_code = ?",
        )
        .bind(&self.user_id)
        .bind(verifcation_code)
        .fetch_optional(&**driver)
        .await
    }

    pub async fn delete(driver: &Database, id: &str) -> Result<()> {
        let mut tx = driver.begin().await?;

        sqlx::query("DELETE FROM oauth2_access_tokens WHERE user_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM oauth2_authorization_codes WHERE user_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM oauth2_pending_authorizations WHERE user_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM oauth2_refresh_tokens WHERE user_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM oauth2_access_tokens WHERE user_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM user_credentials WHERE user_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM user_permitted_scopes WHERE user_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM user_email_verifications WHERE user_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM user_emails WHERE user_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        // Lastly, delete from users table
        sqlx::query("DELETE FROM users WHERE user_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    /// Check whether the current email address is verified.
    /// This is generally true, except for newly created accounts.
    ///
    /// # Errors
    ///
    /// If the query fails
    pub async fn is_email_verified(&self, database: &Database) -> Result<bool> {
        sqlx::query_scalar("SELECT verified FROM user_emails WHERE user_id = ? AND address = ?")
            .bind(&self.user_id)
            .bind(&self.email)
            .fetch_one(&**database)
            .await
    }
}

fn current_time() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

fn gen_email_verification_code() -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
