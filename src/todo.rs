use askama::Template;
use rocket::{
    form::Form,
    FromForm,
    post,
    delete
};

#[derive(FromForm)]
pub struct TodoForm<'a> {
    todo: &'a str
}

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

    TodoTemplate {
        value,
        todo: todo.todo
    }
}

#[delete("/todo")]
pub fn delete_todo() -> &'static str {
    ""
}

