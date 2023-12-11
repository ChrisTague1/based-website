use askama::Template;
use rocket::form::Form;

#[derive(FromForm)]
pub struct TodoForm<'a> {
    todo: &'a str
}

// TODO figure out how to keep the inner API's small (make these private?)
// Might not be possible
#[derive(Template)]
#[template(path = "todo.html")]
pub struct TodoTemplate<'a> {
    todo: &'a str,
    value: String
}

#[post("/todo", data = "<todo>")]
pub fn create_todo(todo: Form<TodoForm>) -> TodoTemplate {
    let num = rand::random::<usize>();
    let value = format!("{}{}", "a", num);

    let todo_html = TodoTemplate {
        value,
        todo: todo.todo
    };

    todo_html
}

#[delete("/todo")]
pub fn delete_todo() -> &'static str {
    ""
}

