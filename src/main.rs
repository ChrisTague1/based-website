#[macro_use] extern crate rocket;

use rocket::{fs::{FileServer, relative}, response::content::RawHtml};

mod todo;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../templates/root.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index,
            // TODO figure out how to make this cleaner
            todo::create_todo,
            todo::delete_todo
        ])
        .mount("/", FileServer::from(relative!("./static")))
}
