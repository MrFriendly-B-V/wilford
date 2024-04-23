use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::response_types::Empty;
use crate::routes::error::{WebError, WebResult};
use crate::routes::v1::MANAGE_SCOPE;
use actix_web::web;
use database::oauth2_client::OAuth2Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    client_id: String,
}

pub async fn remove(
    database: WDatabase,
    auth: Auth,
    payload: web::Json<Request>,
) -> WebResult<Empty> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebError::Forbidden);
    }

    let client = OAuth2Client::get_by_client_id(&database, &payload.client_id)
        .await?
        .ok_or(WebError::NotFound)?;
    client.delete(&database).await?;

    Ok(Empty)
}
