mod recipe;

use common::user::UserLogin;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde::Serialize;
use std::{fmt::Display, future::IntoFuture};

use backend::{app::App, entities::users};
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
    async fn new(pool: DatabaseConnection) -> Result<Self, anyhow::Error> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind to port");
        let address = format!("http://{}", listener.local_addr()?);
        let app = App::new(pool.clone())?;

        users::Entity::insert(users::ActiveModel {
            email: Set(TEST_EMAIL.into()),
            name: Set(TEST_NAME.into()),
            ..Default::default()
        })
        .exec(&app.app_state.db)
        .await?;

        let client = reqwest::Client::builder().cookie_store(true).build()?;

        let server = axum::serve(listener, app.router.into_make_service());
        tokio::spawn(server.into_future());

        Ok(Self {
            address,
            client,
            pool,
        })
    }

    pub async fn post<T, U>(&self, url: U, body: &T) -> Result<Response, anyhow::Error>
    where
        U: IntoUrl + Display,
        T: Serialize + ?Sized,
    {
        let response = self
            .client
            .request(reqwest::Method::POST, format!("{}/{}", self.address, url))
            .json(body)
            .send()
            .await?;

        assert_eq!(
            response.status(),
            reqwest::StatusCode::UNAUTHORIZED,
            "{} is not protected",
            url
        );

        self.client
            .request(reqwest::Method::POST, format!("{}/login", self.address))
            .json(&UserLogin {
                email: TEST_EMAIL.to_string(),
                password: TEST_PASSWORD.to_string(),
            })
            .send()
            .await?;

        let res = self
            .client
            .request(reqwest::Method::POST, format!("{}/{}", self.address, url))
            .json(body)
            .send()
            .await?;

        self.client
            .request(reqwest::Method::POST, format!("{}/logout", self.address))
            .send()
            .await?;

        Ok(res)
    }

    pub fn put<U: IntoUrl + Display>(&self, url: U) -> RequestBuilder {
        self.client
            .request(reqwest::Method::PUT, format!("{}/{}", self.address, url))
    }

    pub fn delete<U: IntoUrl + Display>(&self, url: U) -> RequestBuilder {
        self.client
            .request(reqwest::Method::DELETE, format!("{}/{}", self.address, url))
    }

    pub fn get<U: IntoUrl + Display>(&self, url: U) -> RequestBuilder {
        self.client
            .request(reqwest::Method::GET, format!("{}/{}", self.address, url))
    }
}
