use actix_identity::Identity;
use actix_web::{
    body::BoxBody,
    dev::{forward_ready, ServiceRequest},
    Result,
};
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{Service, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

pub struct AuthGuard;

impl<S, B> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: From<BoxBody>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthGuardMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthGuardMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: From<BoxBody>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    // Ideally I want auth on everything and then opt out of it for single routes, but looks like
    // this isn't possible at the moment :(
    // I also don't feel comfortable using extractors since I feel like it's easy to forget to add
    // them to routes that I want to protect, but don't need the session. They are also IMO harder
    // to spot than using wrap inside an attribute macro
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);
        Box::pin(async move {
            // This returns 401 if it fails
            req.extract::<Identity>().await?;
            let fut = srv.call(req);
            let res = fut.await?;
            Ok(res)
        })
    }
}

// For actix_web_grants
// pub async fn extract(req: &mut ServiceRequest) -> Result<Vec<String>, actix_web::Error> {
//     // Don't return error if no identity is found. Means that someone is not logged in. If we
//     // return error here we get some werid behaviour.
//     let Ok(id) = req.extract::<Identity>().await else {
//         return Ok(vec![]);
//     };
//     let role = id
//         .get_claims()
//         .map_err(|_| {
//             let res = actix_web::error::InternalError::from_response(
//                 anyhow::anyhow!("Error"),
//                 HttpResponse::new(StatusCode::UNAUTHORIZED),
//             );
//             return actix_web::Error::from(res);
//         })?
//         .role;
//     Ok(vec![role.to_string()])
// }
