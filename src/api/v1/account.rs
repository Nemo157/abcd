use chrono::{DateTime, Utc};
use serde::Serialize;
use slog::debug;
use tide::response::{self, IntoResponse};
use tide_slog::ContextExt as _;

#[derive(Serialize, Debug)]
struct Account {
    id: String,
    username: String,
    acct: String,
    display_name: String,
    locked: bool,
    created_at: DateTime<Utc>,
    followers_count: u64,
    following_count: u64,
    statuses_count: u64,
    note: String,
    url: String,
    avatar: String,
    avatar_static: String,
    header: String,
    header_static: String,
    emojis: Vec<()>,
    moved: Option<()>,
    fields: Option<()>,
    bot: Option<()>,
}

pub async fn verify_credentials(
    cx: tide::Context<crate::State>,
) -> tide::EndpointResult<impl IntoResponse> {
    let response = Account {
        id: "5".into(),
        username: "username".into(),
        acct: "acct".into(),
        display_name: "display_name".into(),
        locked: false,
        created_at: Utc::now(),
        followers_count: 0,
        following_count: 0,
        statuses_count: 0,
        note: "".into(),
        url: "".into(),
        avatar: "".into(),
        avatar_static: "".into(),
        header: "".into(),
        header_static: "".into(),
        emojis: vec![],
        moved: None,
        fields: None,
        bot: None,
    };
    debug!(cx.logger(), "verify credentials";
           "path" => %cx.uri(),
           "response" => ?response,
           "headers" => ?cx.headers());
    Ok(response::json(response))
}
