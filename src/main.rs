mod database;

use rocket::fs::{FileServer, NamedFile};
use sqlx;
use std::sync::Arc;
use pyo3::prelude::*;
use pyo3::types::PyDict;
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
async fn view_paste(
    id: &str,
    injection: &rocket::State<DBInjection>
) -> rocket::response::content::RawHtml<String> {
    let handler = database::DatabaseHandler {
        pool: injection.pool.clone(),
    };
    let content = handler.get_paste(id).await;
    let html_data = std::fs::read_to_string("static/templates/view_paste.html")
        .expect("Unable to read file.")
        .replace("$content", &content);

    let result: String=   Python::with_gil(|py| {
            let code_str = r#"
from __future__ import annotations

from pygments import highlight
from pygments.lexers import guess_lexer
from pygments.formatters import HtmlFormatter

def highlight_html(code: str)-> str:
    lexer = guess_lexer(code)
    print(lexer)
    formatter = HtmlFormatter(style='colorful')
    return highlight(code, lexer, formatter)
    return pygments.highlight(content, lexer, formatter)
"#;
    
            py.run(code_str, None, None).expect("unable to parse code");
    
            let locals = PyDict::new(py);
            locals.set_item("code", html_data).expect("couldnt add html content.");
    
            let result = py.eval("highlight_html(code)", None, Some(&locals)).expect("Unable to run py code.");
        result.extract::<String>().expect("invalid data type").to_string()
        
        });

    rocket::response::content::RawHtml(result)
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
    rocket::response::Redirect::to(format!("../paste?id={}", paste_id))
}

#[launch]
async fn rocket() -> _ {
    let pool = Arc::new(
        sqlx::PgPool::connect(&std::env::var("PGSQL_URL").expect("PGSQL_URL key not found."))
            .await
            .expect("Unable to create database pool connection."),
    );

    let injection = DBInjection { pool: pool.clone() };
    pyo3::prepare_freethreaded_python();
    rocket::build()
        .mount("/", routes![index, paste, view_paste])
        .mount("/paste", routes![create_paste])
        .mount("/static", FileServer::from("././static"))
        .manage(injection)
}
