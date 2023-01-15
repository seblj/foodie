use actix_identity::Identity;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use sqlx::{Pool, Postgres};

use crate::{
    auth_middleware::AuthGuard,
    services::auth::{self, LoginInfo},
    UserClaims,
};

#[post("/login")]
pub async fn login(
    login_info: web::Json<LoginInfo>,
    req: HttpRequest,
    pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    match auth::authenticate(pool, login_info.into_inner()).await {
        Ok(user) => {
            Identity::login(
                &req.extensions(),
                serde_json::to_string(&UserClaims { id: user.id }).unwrap(),
            )
            .unwrap();
            HttpResponse::Ok().body("Authorized")
        }
        Err(_) => HttpResponse::Unauthorized().body("Not authorized"),
    }
}

#[get("/is-logged-in", wrap = "AuthGuard")]
pub async fn is_logged_in(user: Identity) -> HttpResponse {
    // let id = user.get_claims().unwrap().id;
    // match users::get_user_state(state.db(), id).await {
    //     Ok(user) => HttpResponse::Ok().json(user),
    //     Err(_) => HttpResponse::BadRequest().body("Failed to get user state"),
    // }
    HttpResponse::Ok().into()
}

#[post("/logout", wrap = "AuthGuard")]
pub async fn logout(user: Identity) -> HttpResponse {
    user.logout();
    HttpResponse::Ok().body("User logged out")
}
