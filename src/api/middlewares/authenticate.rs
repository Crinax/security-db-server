use std::{
    future::{ready, Ready},
    sync::Arc,
};

use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    HttpMessage,
};

use crate::services::auth::{AuthService, SecretsProvider};
use futures_util::future::LocalBoxFuture;

pub struct JwtAuthService<S, T>
where
    T: SecretsProvider,
{
    service: S,
    secrets_provider: Arc<T>,
}

macro_rules! need_authorization {
    ($req:ident) => {
        let res = $req.into_response(
            actix_web::HttpResponse::Unauthorized()
                .json(crate::api::errors::JsonMessage {
                    message: "need_authorization",
                })
                .map_into_boxed_body(),
        );
        return Box::pin(async move {
            Ok(res.map_body(|_, body| actix_web::body::EitherBody::right(body)))
        });
    };
}

impl<S, B, T: SecretsProvider> Service<ServiceRequest> for JwtAuthService<S, T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ServiceResponse<EitherBody<B>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get(header::AUTHORIZATION);

        if auth_header.is_none() {
            need_authorization!(req);
        }

        let auth_header = auth_header.unwrap();

        if auth_header.is_empty() {
            need_authorization!(req);
        }

        let auth_value = auth_header.to_str();

        if auth_value.is_err() {
            log::info!("Non visible ASCII characters in header value");
            need_authorization!(req);
        }

        let mut auth_value = auth_value.unwrap().split(' ');
        let auth_type = auth_value.next();
        let token = auth_value.last();

        if auth_type.is_none() || token.is_none() {
            need_authorization!(req);
        }

        let auth_type = auth_type.unwrap();
        let token = token.unwrap();

        if auth_type.to_lowercase() != "bearer" {
            need_authorization!(req);
        }

        let data = AuthService::validate_token(token, self.secrets_provider.as_ref());

        if data.is_err() {
            let res = req.into_response(
                actix_web::HttpResponse::Forbidden()
                    .json(crate::api::errors::JsonMessage {
                        message: "invalid_token",
                    })
                    .map_into_boxed_body(),
            );
            return Box::pin(async move {
                Ok(res.map_body(|_, body| actix_web::body::EitherBody::right(body)))
            });
        }

        let data = data.unwrap();

        req.extensions_mut().insert(data);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            Ok(res.map_body(|_, body| EitherBody::left(body)))
        })
    }
}

pub struct JwtAuth<T>
where
    T: SecretsProvider,
{
    secrets_provider: Arc<T>,
}

impl<T: SecretsProvider> JwtAuth<T> {
    pub fn new(secrets_provider: Arc<T>) -> Self {
        Self { secrets_provider }
    }
}

impl<S, B, T: SecretsProvider> Transform<S, ServiceRequest> for JwtAuth<T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = JwtAuthService<S, T>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthService {
            service,
            secrets_provider: self.secrets_provider.clone(),
        }))
    }
}
