#[macro_use] extern crate rocket;

mod todo;

use rocket::{fs::{FileServer, relative}, response::content::RawHtml};

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../templates/root.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index,
            todo::create_todo,
            todo::delete_todo
        ])
        .mount("/", FileServer::from(relative!("./static")))
}

