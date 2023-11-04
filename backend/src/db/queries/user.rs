use common::user::{CreateUser, User};

use crate::db::FoodiePool;

impl FoodiePool {
    pub async fn create_user(&self, create_user_info: &CreateUser) -> Result<User, anyhow::Error> {
        let mut tx = self.begin().await?;
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
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(User {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, anyhow::Error> {
        let mut tx = self.begin().await?;
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
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(User {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }
}
