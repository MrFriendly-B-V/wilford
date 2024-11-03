use actix_web::web;
use serde::Deserialize;

use database::constant_access_tokens::ConstantAccessToken;

use crate::response_types::Empty;
use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::MANAGE_SCOPE;

#[derive(Deserialize)]
pub struct Request {
    token: String,
}

pub async fn remove(
    database: WDatabase,
    auth: Auth,
    payload: web::Json<Request>,
) -> WebResult<Empty> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebErrorKind::Forbidden.into());
    }

    let cat = ConstantAccessToken::get_by_token(&database, &payload.token)
        .await?
        .ok_or(WebErrorKind::NotFound)?;
    cat.revoke(&database).await?;

    Ok(Empty)
}
