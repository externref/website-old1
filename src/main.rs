mod database;

use rocket::fs::{FileServer, NamedFile};
use sqlx;
use std::sync::Arc;

#[macro_use]
extern crate rocket;

struct DBInjection {
    pool: Arc<sqlx::PgPool>,
}

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open("static/templates/home.html")
        .await
        .expect("Unable to find file.")
}

#[get("/paste")]
async fn paste() -> NamedFile {
    NamedFile::open("static/templates/paste.html")
        .await
        .expect("Unable to find file.")
}

#[get("/paste?<id>")]
async fn view_paste(id: &str) -> NamedFile {
    NamedFile::open("static/templates/view_paste.html")
        .await
        .expect("Unable to find file.")
}

#[get("/create_paste?<paste_data>")]
async fn create_paste(
    paste_data: &str,
    injection: &rocket::State<DBInjection>,
) -> rocket::response::Redirect {
    let mut handler = database::DatabaseHandler {
        pool: injection.pool.clone(),
    };
    let paste_id = handler.add_paste(paste_data).await;
    rocket::response::Redirect::to(format!("./paste?id={}", paste_id))
}

#[launch]
async fn rocket() -> _ {
    let pool = Arc::new(
        sqlx::PgPool::connect(&std::env::var("PGSQL_URL").expect("PGSQL_URL key not found."))
            .await
            .expect("Unable to create database pool connection."),
    );

    let injection = DBInjection { pool: pool.clone() };

    rocket::build()
        .mount("/", routes![index, paste, view_paste])
        .mount("/paste", routes![create_paste])
        .mount("/static", FileServer::from("././static"))
        .manage(injection)
}
