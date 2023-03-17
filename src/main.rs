mod database;
mod utils;
use rocket::fs::{FileServer, NamedFile};
use sqlx;
#[macro_use]
extern crate rocket;

struct DBInjection {
    pool: std::sync::Arc<sqlx::PgPool>,
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
async fn view_paste(
    id: &str,
    // lang: std::option::Option<&str>,
    injection: &rocket::State<DBInjection>,
) -> (rocket::http::ContentType, String) {
    let handler = database::DatabaseHandler {
        pool: injection.pool.clone(),
    };
    let content = handler.get_paste(id).await;

    //let result = utils::highlight_code(&content, lang.unwrap_or_else(|| "python"));
    let html_data = std::fs::read_to_string("static/templates/view_paste.html")
        .expect("Unable to read file.")
        .replace("$content", &content.replace("\n", "<br>").replace(" ","&nbsp" ))
        .to_string();
    (rocket::http::ContentType::HTML, html_data)
    //rocket::response::content::RawHtml(html_data)
}

#[get("/create_paste?<paste_data>")]
async fn create_paste(
    paste_data: &str,
    injection: &rocket::State<DBInjection>,
) -> rocket::response::Redirect {
    let mut handler = database::DatabaseHandler {
        pool: injection.pool.clone(),
    };
    println!("{}",paste_data);
    let paste_id = handler.add_paste(paste_data).await;
    rocket::response::Redirect::to(format!("../paste?id={}", paste_id))
}

#[launch]
async fn rocket() -> _ {
    let pool = std::sync::Arc::new(
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
