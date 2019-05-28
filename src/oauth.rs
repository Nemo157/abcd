use serde::Deserialize;
use slog::error;
use tide::{error::ResultExt as _, response::IntoResponse};
use tide_slog::ContextExt as _;

#[derive(Deserialize, Debug)]
struct AuthorizeData {
    response_type: String,
    client_id: String,
    redirect_uri: Option<String>,
    scope: Option<String>,
    state: Option<String>,
}

use http_service::Body;
use oxide_auth::frontends::simple::request::{
    Body as OAuthBody, Request as OAuthRequest, Response as OAuthResponse, Status as OAuthStatus,
};
use std::{collections::HashMap, io};

async fn transform_request(
    cx: &mut tide::Context<crate::State>,
) -> Result<OAuthRequest, Box<dyn std::error::Error + Send + Sync>> {
    let query = cx
        .request()
        .uri()
        .query()
        .map(serde_urlencoded::from_str)
        .transpose()?
        .unwrap_or_else(HashMap::new);
    let mut urlbody: HashMap<String, String> =
        serde_urlencoded::from_bytes(&cx.body_bytes().await?)?;
    let auth = cx
        .request()
        .headers()
        .get("Authorization")
        .map(|v| v.to_str().map(|v| v.to_owned()))
        .transpose()?;
    // Workaround https://github.com/HeroicKatora/oxide-auth/issues/29
    urlbody
        .entry("redirect_uri".into())
        .and_modify(|uri| uri.push_str("/"));
    // Workaround https://github.com/HeroicKatora/oxide-auth/issues/28
    let auth = auth.or_else(|| {
        urlbody.remove("client_id").and_then(|id| {
            urlbody
                .remove("client_secret")
                .map(|secret| format!("Basic {}", base64::encode(&format!("{}:{}", id, secret))))
        })
    });
    Ok(OAuthRequest {
        query,
        urlbody,
        auth,
    })
}

fn transform_response(response: OAuthResponse) -> tide::Response {
    let mut builder = http::Response::builder();
    builder.status(match response.status {
        OAuthStatus::Ok => http::status::StatusCode::OK,
        OAuthStatus::Redirect => http::status::StatusCode::TEMPORARY_REDIRECT,
        OAuthStatus::BadRequest => http::status::StatusCode::BAD_REQUEST,
        OAuthStatus::Unauthorized => http::status::StatusCode::UNAUTHORIZED,
    });
    if let Some(location) = response.location {
        builder.header("Location", location.to_string());
    }
    if let Some(www_authenticate) = response.www_authenticate {
        builder.header("WWW-Authenticate", www_authenticate);
    }
    match response.body {
        Some(OAuthBody::Text(text)) => {
            builder.header("Content-Type", "text/plain");
            builder.body(text.into()).unwrap()
        }
        Some(OAuthBody::Json(json)) => {
            builder.header("Content-Type", "application/json");
            builder.body(json.into()).unwrap()
        }
        None => {
            builder.header("Content-Type", "text/plain");
            builder.body(Body::empty()).unwrap()
        }
    }
}

fn solicitor(
    _: &mut OAuthRequest,
    _: &oxide_auth::endpoint::PreGrant,
) -> oxide_auth::endpoint::OwnerConsent<OAuthResponse> {
    oxide_auth::endpoint::OwnerConsent::Authorized("bobby".to_owned())
}

pub async fn authorize(
    mut cx: tide::Context<crate::State>,
) -> tide::EndpointResult<impl IntoResponse> {
    let request = transform_request(&mut cx).await.unwrap();
    let response = {
        let crate::State {
            registrar,
            authorizer,
            ..
        } = cx.state();
        let registrar = registrar.lock().unwrap();
        let mut authorizer = authorizer.lock().unwrap();
        let mut solicitor = oxide_auth::frontends::simple::endpoint::FnSolicitor(solicitor);
        let mut flow = oxide_auth::frontends::simple::endpoint::authorization_flow(
            &*registrar,
            &mut authorizer,
            &mut solicitor,
        );
        flow.execute(request.clone())
            .map_err(|error| {
                error!(cx.logger(), ""; "error" => ?error);
                io::Error::new(io::ErrorKind::Other, "")
            })
            .client_err()?
    };
    Ok(transform_response(response))
}

pub async fn token(mut cx: tide::Context<crate::State>) -> tide::EndpointResult<impl IntoResponse> {
    let request = transform_request(&mut cx).await.unwrap();
    let response = {
        let crate::State {
            registrar,
            authorizer,
            issuer,
            ..
        } = cx.state();
        let registrar = registrar.lock().unwrap();
        let mut authorizer = authorizer.lock().unwrap();
        let mut issuer = issuer.lock().unwrap();
        let mut flow = oxide_auth::frontends::simple::endpoint::access_token_flow(
            &*registrar,
            &mut authorizer,
            &mut issuer,
        );
        flow.execute(request.clone())
            .map_err(|error| {
                error!(cx.logger(), ""; "error" => ?error);
                io::Error::new(io::ErrorKind::Other, "")
            })
            .client_err()?
    };
    Ok(transform_response(response))
}
