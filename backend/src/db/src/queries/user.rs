use anyhow::anyhow;
use common::user::{CreateUser, User};

use crate::FoodieDatabase;

impl FoodieDatabase {
    pub async fn create_user(&self, create_user_info: &CreateUser) -> Result<User, anyhow::Error> {
        let user = sqlx::query!(
            r#"
INSERT INTO
  users (email, name)
VALUES
  ($1, $2)
on conflict (email) do nothing
RETURNING
  id,
  name,
  email
    "#,
            create_user_info.email,
            create_user_info.name,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| anyhow!("Couldn't create user"))?;

        Ok(User {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, anyhow::Error> {
        let user = sqlx::query!(
            r#"
SELECT
  *
FROM
  users
WHERE
  email = $1
        "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }
}
