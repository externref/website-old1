use rocket::fs::{FileServer, NamedFile};

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, paste])
        .mount("/static", FileServer::from("static"))
}
