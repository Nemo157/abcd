use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use http::{header::HeaderValue, status::StatusCode, Response};
use http_service::Body;
use mime::Mime;
use slog::error;
use tide::{
    middleware::{Middleware, Next},
    response::IntoResponse,
    Context,
};
use tide_slog::ContextExt;

pub struct With<T, M> {
    response: T,
    modifier: M,
}

pub struct Redirect {
    location: HeaderValue,
}

pub fn redirect(location: &str) -> Result<Redirect, http::header::InvalidHeaderValue> {
    let location = HeaderValue::from_str(location)?;
    Ok(Redirect { location })
}

pub trait ResponseModifier: Send {
    fn modify(self, response: &mut Response<Body>);
}

impl ResponseModifier for Redirect {
    fn modify(self, response: &mut Response<Body>) {
        StatusCode::TEMPORARY_REDIRECT.modify(response);
        response.headers_mut().insert("Location", self.location);
    }
}

impl ResponseModifier for Mime {
    fn modify(self, response: &mut Response<Body>) {
        response.headers_mut().insert(
            "Content-Type",
            HeaderValue::from_str(self.as_ref()).unwrap(),
        );
    }
}

impl ResponseModifier for StatusCode {
    fn modify(self, response: &mut Response<Body>) {
        *response.status_mut() = self;
    }
}

impl<T: IntoResponse, M: ResponseModifier> IntoResponse for With<T, M> {
    fn into_response(self) -> Response<Body> {
        let mut response = self.response.into_response();
        self.modifier.modify(&mut response);
        response
    }
}

pub trait IntoResponseExt: IntoResponse + Sized {
    fn with<M: ResponseModifier>(self, modifier: M) -> With<Self, M> {
        With {
            response: self,
            modifier,
        }
    }
}

impl<T: IntoResponse + Sized> IntoResponseExt for T {}
