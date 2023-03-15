mod database;
use rocket::fs::{FileServer, NamedFile};
use sqlx;

struct DBInjection {
    database: database::DatabaseHandler,
}

#[macro_use]
extern crate rocket;
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

#[get("/create_paste?<paste_data>")]
async fn create_paste(paste_data: &str) {
    println!("{}", paste_data)
}

#[launch]
async fn rocket() -> _ {
    let pool = sqlx::PgPool::connect(&std::env::var("PGSQL_URL").expect("PGSQL_URL key not found."))
        .await
        .expect("Unable to create database pool connection.");
    let db_handler: database::DatabaseHandler =
        database::DatabaseHandler { pool: pool }.setup().await;
    let injection: DBInjection = DBInjection {
        database: db_handler,
    };

    rocket::build()
        .mount("/", routes![index, paste])
        .mount("/paste", routes![create_paste])
        .mount("/static", FileServer::from("././static"))
        .manage(injection)
}
