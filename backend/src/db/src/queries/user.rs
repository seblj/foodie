use anyhow::anyhow;
use sqlx::postgres::PgQueryResult;

use crate::{
    user::{CreateUser, User},
    FoodieDatabase,
};

impl FoodieDatabase {
    pub async fn create_user(
        &self,
        create_user_info: &CreateUser,
    ) -> Result<PgQueryResult, anyhow::Error> {
        sqlx::query!(
            r#"
INSERT INTO
  users (email, name)
VALUES
  ($1, $2)
on conflict (email) do nothing;
    "#,
            create_user_info.email,
            create_user_info.name,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| anyhow!("Couldn't create user"))
    }

    pub async fn get_user(&self, email: String) -> Result<User, anyhow::Error> {
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
