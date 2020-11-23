use actix_web::{web, http, HttpRequest, HttpResponse, Result};
use chrono::{Utc};
use serde::Deserialize;
use diesel::prelude::*;

use crate::database;
use crate::playbook::model::playbook::NewPlaybook;
use crate::error::UserFacingError;
use crate::schema::playbook;
use crate::shared::input;

#[derive(Deserialize)]
pub struct NewPlaybookRequest {
    title: String
}

pub async fn handle(
    req: web::Json<NewPlaybookRequest>
) -> Result<HttpResponse> {

    let connection = database::establish_connection()
        .map_err(|_e| UserFacingError::InternalError)?;

    let new_playbook = NewPlaybook {
        title: input::clean(&req.title),
        create_time: Utc::now().timestamp(),
        update_time: Utc::now().timestamp()
    };

    diesel::insert_into(playbook::table)
        .values(&new_playbook)
        .execute(&connection)
        .map_err(|_e| UserFacingError::InternalError)?;

    Ok(
        HttpResponse::Created()
            .header(http::header::LOCATION, "/api/playbook/latest")
            .finish()
    )
}