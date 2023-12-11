#[macro_use] extern crate rocket;

use rocket::{fs::{FileServer, relative}, response::content::RawHtml, form::Form};

mod todo;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../templates/root.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, create_todo, delete_todo])
        .mount("/", FileServer::from(relative!("./static")))
}
