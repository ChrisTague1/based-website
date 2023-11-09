#[macro_use] extern crate rocket;

use rocket::{fs::{FileServer, relative}, response::content::RawHtml, form::Form};
use rand::Rng;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("./root.html"))
}

#[derive(FromForm)]
struct TodoForm {
    todo: String
}

#[post("/todo", data = "<todo>")]
fn create_todo(todo: Form<TodoForm>) -> RawHtml<String> {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..=100);
    let value = format!("{}{}", "a", num);
    RawHtml(format!(r##"
<li id="{}" class="flex items-center justify-between p-2">
    <span class="text-gray-700">{}</span>
    <button
        class="text-red-500 hover:text-red-700"
        hx-delete="/todo"
        hx-target="#{}"
        hx-swap="outerHTML"
    >
      Delete
    </button>
</li>
    "##, value, todo.todo, value))
}

#[delete("/todo")]
fn delete_todo() -> RawHtml<&'static str> {
    RawHtml("")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, create_todo, delete_todo])
        .mount("/", FileServer::from(relative!("./static")))
}
