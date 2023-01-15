use actix_cors::Cors;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use serde::{Deserialize, Serialize};

use crate::{
    api::auth::{is_logged_in, login, logout},
    db::connect,
};

mod api;
mod auth_middleware;
mod db;
mod services;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserClaims {
    pub id: i64,
}

pub trait UserIdentity {
    fn get_claims(&self) -> Result<UserClaims, anyhow::Error>;
}

impl UserIdentity for Identity {
    fn get_claims(&self) -> Result<UserClaims, anyhow::Error> {
        serde_json::from_str(&self.id()?).map_err(|_| anyhow::anyhow!("Couldn't convert to value"))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    let secret_key = Key::derive_from(std::env::var("FOODIE_SECRET_KEY")?.as_bytes());
    let port = 6000;

    let pool = connect().await?;

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4500")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(if cfg!(debug_assertions) { false } else { true })
                    .build(),
            )
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(
                web::scope("/api")
                    .wrap(Logger::new("%r status: %s, bytes: %b, time: %T").log_target("actix_log"))
                    .service(login)
                    .service(is_logged_in)
                    .service(logout),
            )
    })
    .bind(("0.0.0.0", port))?
    .run();
    println!("Server running on localhost:{}", port);
    server.await?;
    Ok(())
}
