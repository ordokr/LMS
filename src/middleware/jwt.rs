// src/middleware/jwt.rs

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    Error,
    HttpResponse,
};
use futures_util::future::{LocalBoxFuture, ready, Ready};
use crate::auth::jwt::validate_token;

pub struct JwtMiddleware;

impl<S, B> Service<ServiceRequest, State = S, Body = B> for JwtMiddleware
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(S);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str[7..].trim();

                    if validate_token(token).is_ok() {
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        });
                    }
                }
            }
        }

        Box::pin(ready(Ok(req.into_response(
            HttpResponse::Unauthorized().finish()
        ))))
    }
}