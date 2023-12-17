use rocket::{
    get,
    launch,
    routes
};

mod todo;

use rocket::{fs::{FileServer, relative}, response::content::RawHtml};

#[get("/todo")]
fn todo_landing() -> RawHtml<&'static str> {
    RawHtml(include_str!("../templates/root.html"))
}

#[get("/")]
fn landing() -> RawHtml<&'static str> {
    RawHtml(include_str!("../templates/landing.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            landing,
            todo_landing,
            todo::create_todo,
            todo::delete_todo
        ])
        .mount("/", FileServer::from(relative!("./static")))
}

