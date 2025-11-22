use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures::future::LocalBoxFuture;
use std::{future::{ready, Ready}, rc::Rc};
use governor::{Quota, RateLimiter as GovernorRateLimiter};
use std::num::NonZeroU32;

pub struct RateLimiter {
    limiter: Rc<GovernorRateLimiter<String, governor::state::InMemoryState, governor::clock::DefaultClock>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        let quota = Quota::per_minute(NonZeroU32::new(100).unwrap());
        Self {
            limiter: Rc::new(GovernorRateLimiter::keyed(quota)),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service: Rc::new(service),
            limiter: self.limiter.clone(),
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: Rc<S>,
    limiter: Rc<GovernorRateLimiter<String, governor::state::InMemoryState, governor::clock::DefaultClock>>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let limiter = self.limiter.clone();
        let service = self.service.clone();

        Box::pin(async move {
            let key = req.connection_info()
                .realip_remote_addr()
                .unwrap_or("unknown")
                .to_string();

            if limiter.check_key(&key).is_ok() {
                service.call(req).await
            } else {
                Err(actix_web::error::ErrorTooManyRequests("Rate limit exceeded"))
            }
        })
    }
}
