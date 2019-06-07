use serde::{Serialize, ser::Serializer};
use slog::debug;
use tide::response::{self, IntoResponse};
use tide_slog::ContextExt as _;
use diesel::prelude::*;
use crate::schema::lists::dsl::*;
use uuid::Uuid;
use diesel::pg::PgConnection;

#[derive(Queryable, Serialize, Debug)]
struct List {
    #[serde(skip)]
    id: i32,
    #[serde(rename = "id", serialize_with = "uuid_to_bs58")]
    public_id: Uuid,
    title: String,
}

fn uuid_to_bs58<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(&bs58::encode(uuid.as_bytes()).into_string())
}

pub async fn mine(
    cx: tide::Context<crate::State>,
) -> tide::EndpointResult<impl IntoResponse> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let connection = PgConnection::establish(&database_url).unwrap();
    let results = lists
        .load::<List>(&connection)
        .expect("Error loading lists");
    debug!(cx.logger(), "mine";
           "path" => %cx.uri(),
           "results" => ?results,
           "headers" => ?cx.headers());
    Ok(response::json(results))
}
