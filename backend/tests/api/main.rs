mod recipe;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use backend::db::{FoodieDatabase, FoodiePool};
use common::user::CreateUser;
use serde::Serialize;
use sqlx::PgPool;
use std::{fmt::Display, net::TcpListener};
use uuid::Uuid;

use backend::app::{App, AuthContext};
use hyper::{Method, StatusCode};
use reqwest::{IntoUrl, RequestBuilder, Response};

struct TestApp {
    pub client: reqwest::Client,
    pub address: String,
    email: String,
    pool: FoodiePool,
    current_user: String,
}

const TEST_EMAIL: &str = "foo@foo.com";
const TEST_NAME: &str = "foo";

async fn test_login(
    mut auth: AuthContext,
    State(db): State<FoodieDatabase>,
    Json(email): Json<String>,
) -> impl IntoResponse {
    let user = db.get(None).get_user_by_email(&email).await.unwrap();
    auth.login(&user).await.expect("Couldn't log user in");
}

async fn test_logout(mut auth: AuthContext) -> impl IntoResponse {
    auth.logout().await;
}

// TODO: Move this to another place to make the implementation details hidden from the tests.
// Now the tests are able to access non public fields and methods which I do not like, as they are
// in the same module

// Maybe I want different `post_unauth` or something similar for the endpoints that should be
// open. I am not sure if this is really needed, but think about it.
impl TestApp {
    async fn new(pool: PgPool) -> Result<Self, anyhow::Error> {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
        let address = format!("http://{}", listener.local_addr()?);
        let app = App::new(pool.clone())?;

        let current_user = uuid::Uuid::new_v4().to_string();
        sqlx::query(&format!("CREATE ROLE \"{}\"", current_user))
            .execute(&pool)
            .await?;

        sqlx::query(&format!(
            "GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA PUBLIC TO \"{}\"",
            current_user
        ))
        .execute(&pool)
        .await?;

        sqlx::query(&format!("SET ROLE \"{}\"", current_user))
            .execute(&pool)
            .await?;

        app.app_state
            .clone()
            .db
            .get(None)
            .create_user(&CreateUser {
                email: TEST_EMAIL.into(),
                name: TEST_NAME.into(),
            })
            .await?;

        let router = app.router.merge(
            Router::new()
                .route("/test-login", post(test_login))
                .route("/test-logout", post(test_logout))
                .with_state(app.app_state)
                .layer(app.auth_layer)
                .layer(app.session_layer),
        );

        let server = axum::Server::from_tcp(listener)?.serve(router.into_make_service());
        tokio::spawn(server);
        let client = reqwest::Client::builder().cookie_store(true).build()?;

        Ok(Self {
            address,
            client,
            email: TEST_EMAIL.to_string(),
            pool: FoodiePool::new(pool, None),
            current_user,
        })
    }

    // TODO: Get rid of this method
    pub async fn set_user(&mut self, email: &str) -> Result<(), anyhow::Error> {
        let user = self
            .pool
            .create_user(&CreateUser {
                email: email.into(),
                name: "foo".into(),
            })
            .await?;

        self.pool.user_id = Some(user.id);
        self.email = email.to_string();
        Ok(())
    }

    // TODO: Maybe do a thing where I can set the current user on app, and it creates and logges
    // into the user
    pub async fn post<T, U>(&self, url: U, body: &T) -> Result<Response, anyhow::Error>
    where
        U: IntoUrl + Display,
        T: Serialize + ?Sized,
    {
        let response = self
            .client
            .request(Method::POST, format!("{}/{}", self.address, url))
            .json(body)
            .send()
            .await?;

        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "{} is not protected",
            url
        );

        self.client
            .request(Method::POST, format!("{}/test-login", self.address))
            .json(&self.email)
            .send()
            .await?;

        let res = self
            .client
            .request(Method::POST, format!("{}/{}", self.address, url))
            .json(body)
            .send()
            .await?;

        self.client
            .request(Method::POST, format!("{}/test-logout", self.address))
            .send()
            .await?;

        Ok(res)
    }

    pub fn put<U: IntoUrl + Display>(&self, url: U) -> RequestBuilder {
        self.client
            .request(Method::PUT, format!("{}/{}", self.address, url))
    }

    pub fn delete<U: IntoUrl + Display>(&self, url: U) -> RequestBuilder {
        self.client
            .request(Method::DELETE, format!("{}/{}", self.address, url))
    }

    pub fn get<U: IntoUrl + Display>(&self, url: U) -> RequestBuilder {
        self.client
            .request(Method::GET, format!("{}/{}", self.address, url))
    }

    // TODO: Maybe I should have all these things another place? Maybe in a db-query thing that
    // only return the database model based on the id of the record

    // Think about if I also want an insert query, to insert the model naively for a test or
    // something for setup of different dependencies. For example, for a recipe, I need ingredients
    // to already be setup. Think about how I would want to do that
    pub async fn get_recipe(&self, id: Uuid) -> Option<backend::db::models::Recipe> {
        sqlx::query_as!(
            backend::db::models::Recipe,
            r#"
SELECT
  *
FROM
  recipes
WHERE
  id = $1
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .ok()
    }

    pub async fn get_ingredient(&self, id: Uuid) -> Option<backend::db::models::Ingredient> {
        sqlx::query_as!(
            backend::db::models::Ingredient,
            r#"
SELECT
  *
FROM
  ingredients
WHERE
  id = $1
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .ok()
    }
}

// TODO: Does not work for some reason
impl Drop for TestApp {
    fn drop(&mut self) {
        use tokio::runtime::Handle;

        let handle = Handle::current();
        let user = self.current_user.clone();
        std::thread::spawn(move || {
            handle.block_on(async move {
                let pool = sqlx::PgPool::connect(&dotenv::var("DATABASE_URL").unwrap())
                    .await
                    .unwrap();

                sqlx::query(&format!("DROP OWNED BY \"{}\"", user))
                    .execute(&pool)
                    .await
                    .unwrap();

                sqlx::query(&format!("DROP ROLE \"{}\"", user))
                    .execute(&pool)
                    .await
                    .unwrap();
            });
        });
    }
}
