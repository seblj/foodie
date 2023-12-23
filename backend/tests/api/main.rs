mod recipe;

use common::user::UserLogin;
use sea_orm::{ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set, SqlxPostgresConnector};
use sea_orm_migration::MigratorTrait;
use serde::Serialize;
use sqlx::PgPool;
use std::{fmt::Display, future::IntoFuture};

use backend::{api::auth::compute_hash, app::App, entities::users};
use reqwest::{IntoUrl, RequestBuilder, Response};

struct TestApp {
    pub client: reqwest::Client,
    pub address: String,
    pool: DatabaseConnection,
}

const TEST_EMAIL: &str = "foo@foo.com";
const TEST_NAME: &str = "foo";
const TEST_PASSWORD: &str = "test123";

// TODO: Move this to another place to make the implementation details hidden from the tests.
// Now the tests are able to access non public fields and methods which I do not like, as they are
// in the same module

// Maybe I want different `post_unauth` or something similar for the endpoints that should be
// open. I am not sure if this is really needed, but think about it.
impl TestApp {
    async fn new(pool: PgPool) -> Result<Self, anyhow::Error> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind to port");

        let address = format!("http://{}", listener.local_addr()?);
        let connection = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);

        migration::Migrator::up(&connection, None).await?;

        let app = App::new(connection.clone())?;

        // TODO: This is slow because of the hashing algorithm I think.
        // It logges in before each request
        users::Entity::insert(users::ActiveModel {
            id: NotSet,
            email: Set(TEST_EMAIL.to_string()),
            password: Set(Some(compute_hash(TEST_PASSWORD.as_bytes()))),
            name: Set(TEST_NAME.to_string()),
        })
        .exec(&app.app_state.db)
        .await?;

        let client = reqwest::Client::builder().cookie_store(true).build()?;

        let server = axum::serve(listener, app.router.into_make_service());
        tokio::spawn(server.into_future());

        Ok(Self {
            address,
            client,
            pool: connection,
        })
    }

    pub async fn post<T, U>(&self, url: U, body: &T) -> Result<Response, anyhow::Error>
    where
        U: IntoUrl + Display,
        T: Serialize + ?Sized,
    {
        let url = format!("{}/{}", self.address, url);
        let req = self.client.request(reqwest::Method::POST, &url).json(body);

        self.assert_logged_in(req.try_clone().unwrap(), &url).await;

        self.login().await;

        let res = req.send().await?;

        self.logout().await;

        Ok(res)
    }

    pub async fn put<T, U>(&self, url: U, body: &T) -> Result<Response, anyhow::Error>
    where
        U: IntoUrl + Display,
        T: Serialize + ?Sized,
    {
        let url = format!("{}/{}", self.address, url);
        let req = self.client.request(reqwest::Method::PUT, &url).json(body);

        self.assert_logged_in(req.try_clone().unwrap(), &url).await;

        self.login().await;

        let res = req.send().await?;

        self.logout().await;

        Ok(res)
    }

    pub async fn delete<U: IntoUrl + Display>(&self, url: U) -> Result<Response, anyhow::Error> {
        let url = format!("{}/{}", self.address, url);
        let req = self.client.request(reqwest::Method::DELETE, &url);

        self.assert_logged_in(req.try_clone().unwrap(), &url).await;

        self.login().await;

        let res = req.send().await?;

        self.logout().await;

        Ok(res)
    }

    pub async fn get<U: IntoUrl + Display>(&self, url: U) -> Result<Response, anyhow::Error> {
        let url = format!("{}/{}", self.address, url);
        let req = self.client.request(reqwest::Method::GET, &url);

        self.assert_logged_in(req.try_clone().unwrap(), &url).await;

        self.login().await;

        let res = req.send().await?;

        self.logout().await;

        Ok(res)
    }

    async fn login(&self) {
        self.client
            .request(reqwest::Method::POST, format!("{}/api/login", self.address))
            .json(&UserLogin {
                email: TEST_EMAIL.to_string(),
                password: TEST_PASSWORD.to_string(),
            })
            .send()
            .await
            .unwrap();
    }

    async fn logout(&self) {
        self.client
            .request(
                reqwest::Method::POST,
                format!("{}/api/logout", self.address),
            )
            .send()
            .await
            .unwrap();
    }

    async fn assert_logged_in(&self, req: RequestBuilder, url: &str) {
        let response = req.send().await.unwrap();

        assert_eq!(
            response.status(),
            reqwest::StatusCode::UNAUTHORIZED,
            "{} is not protected",
            url
        );
    }
}
