#[macro_use] extern crate rocket;

use rocket::{fs::{FileServer, relative}, response::content::RawHtml, form::Form};

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
    println!("{:#?}", todo.todo);
    RawHtml(format!(r#"
<li class="flex items-center justify-between p-2">
    <span class="text-gray-700">{}</span>
    <button class="text-red-500 hover:text-red-700">
      Delete
    </button>
</li>
    "#, todo.todo))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, create_todo])
        .mount("/", FileServer::from(relative!("./static")))
}
