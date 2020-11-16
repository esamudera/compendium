mod model;

use actix_web::{HttpRequest, HttpResponse, error, Result};
use model::playbook::NewPlaybook;
use diesel::prelude::*;

use crate::database;
use crate::schema::playbook;

pub async fn handle_new_playbook(req: HttpRequest) -> Result<&'static str> {

    let connection = database::establish_connection().unwrap();

    let new_playbook = NewPlaybook {
        title: String::from("coba"),
        create_time: 1,
        update_time: 1
    };

    diesel::insert_into(playbook::table)
        .values(&new_playbook)
        .execute(&connection)
        .expect("Error saving new playbook");

    Ok("coba")
}