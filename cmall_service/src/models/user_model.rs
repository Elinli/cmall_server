use std::mem;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use cmall_core::{User, UserStatus};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{error::UserError, AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub dept_id: i64,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub status: UserStatus,
    pub roles: Vec<i64>,
    pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

impl AppState {
    pub async fn create_user(&self, input: &CreateUser) -> Result<User, UserError> {
        let user = self.find_user_by_email(&input.email).await?;
        if user.is_some() {
            return Err(UserError::UserAlreadyExisted(input.email.clone()));
        };
        let password_hash = format_password(&input.password)?;
        let user: User = sqlx::query_as(
            r#"
            INSERT INTO users (dept_id, username, password_hash, email, phone, status, avatar, roles) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
            RETURNING id, dept_id, email, phone, username, create_time, update_time, status, avatar, roles
        "#,
        )
        .bind(&input.dept_id)
        .bind(&input.username)
        .bind(password_hash)
        .bind(&input.email)
        .bind(&input.phone)
        .bind(&input.status)
        .bind(&input.avatar)
        .bind(&input.roles)
        .fetch_one(&self.pool).await.unwrap();
        Ok(user)
    }

    pub async fn verify_user(&self, input: &LoginUser) -> Result<Option<User>, UserError> {
        let user:Option<User> = sqlx::query_as(
            r#"
            SELECT id, username, dept_id, email, create_time, update_time, status, avatar, roles, phone FROM users WHERE email = $1
        "#,
        )
        .bind(&input.email)
        .fetch_optional(&self.pool).await?;
        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);

                let is_valid =
                    verify_password(&input.password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    info!("user found");
                    return Ok(Some(user));
                } else {
                    info!("password not match");
                    return Ok(None);
                }
            }
            None => {
                info!("user not found");
                return Ok(None);
            }
        }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let user  = sqlx::query_as("
          SELECT id, username, dept_id, email, create_time, update_time, status, avatar, roles, phone FROM users WHERE email = $1
        ")
        .bind(email)
        .fetch_optional(&self.pool).await?;
        Ok(user)
    }
    pub async fn find_user_by_id(&self, id: i64) -> Result<Option<User>, UserError> {
        let user  = sqlx::query_as("
          SELECT id, username, dept_id, email, create_time, update_time, status, avatar, roles, phone FROM users WHERE id = $1
        ")
        .bind(id)
        .fetch_optional(&self.pool).await?;
        Ok(user)
    }
}

fn format_password(password: &str) -> Result<String, UserError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, UserError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;

    let is_valid = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(is_valid)
}
// #[cfg(test)]
impl CreateUser {
    pub fn new(username: &str, email: &str, phone: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            phone: phone.to_string(),
            password: password.to_string(),
            status: UserStatus::Active,
            avatar: "default".to_string(),
            dept_id: 1,
            roles: [1].to_vec(),
        }
    }
}

#[cfg(test)]
mod test_user {
    use super::*;
    use anyhow::Result;

    // #[test]
    // fn test_format_password() {
    //     let r = match format_password("test123") {
    //         Ok(r) => r,
    //         Err(_) => "1234".to_string(),
    //     };
    //     assert_eq!(r, "$argon2id$v=19$m=19456,t=2,p=1$eD8F04XyGZgsZKPuxfVPHA$mGlSbvR5I0QqFXAzg256iXmPBSgvjSrhOAyypRKqvqY");
    // }
    #[test]
    fn test_verify_password() {
        let r = verify_password("test123", "$argon2id$v=19$m=19456,t=2,p=1$eD8F04XyGZgsZKPuxfVPHA$mGlSbvR5I0QqFXAzg256iXmPBSgvjSrhOAyypRKqvqY");
       assert_eq!(r.unwrap(), true);
    }

    #[tokio::test]
    async fn test_create_user() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let input = CreateUser::new("test", "test@example.com", "1234567890", "test123");
        assert_eq!(input.password, "test123");
        let user = state.create_user(&input).await.unwrap();

        assert_eq!(user.username, input.username);

        Ok(())
    }
}
