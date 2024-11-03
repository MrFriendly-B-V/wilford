use actix_web::web;
use serde::{Deserialize, Serialize};

use database::oauth2_client::{OAuth2Client, OAuth2PendingAuthorization};

use crate::routes::appdata::WDatabase;
use crate::routes::error::{WebErrorKind, WebResult};

#[derive(Deserialize)]
pub struct Query {
    authorization: String,
}

#[derive(Serialize)]
pub struct Response {
    client_name: String,
    scopes: Option<String>,
}

pub async fn authorization_info(
    database: WDatabase,
    query: web::Query<Query>,
) -> WebResult<web::Json<Response>> {
    let authorization = OAuth2PendingAuthorization::get_by_id(&database, &query.authorization)
        .await?
        .ok_or(WebErrorKind::NotFound)?;

    match &authorization {
        OAuth2PendingAuthorization::Authorized(_) => {}
        OAuth2PendingAuthorization::Unauthorized(_) => {
            return Err(WebErrorKind::Unauthorized.into())
        }
    }

    let client = OAuth2Client::get_by_client_id(&database, &authorization.client_id())
        .await?
        .ok_or(WebErrorKind::NotFound)?;

    Ok(web::Json(Response {
        client_name: client.name,
        scopes: authorization.scopes().clone(),
    }))
}
