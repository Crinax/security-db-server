use std::future::{Ready, ready};

use actix_web::{dev::{Transform, ServiceRequest, Service, ServiceResponse}, HttpMessage};

pub struct JwtAuthService<S> {
    service: S,
    enabled: bool,
}

pub struct Message(pub String);

impl<S, B> Service<ServiceRequest> for JwtAuthService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Error = actix_web::Error;
    type Future = S::Future;
    type Response = ServiceResponse<B>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self.enabled {
            req.extensions_mut()
                .insert(Message("Some message".to_owned()));
        }

        self.service.call(req)
    }
}

pub struct JwtAuth {
    enabled: bool,
}

impl JwtAuth {}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = JwtAuthService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthService { service, enabled: self.enabled }))
    }
}
