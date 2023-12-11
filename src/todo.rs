use askama::Template;

#[derive(FromForm)]
struct TodoForm<'a> {
    todo: &'a str
}

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplate<'a> {
    todo: &'a str,
    value: String
}

#[post("/todo", data = "<todo>")]
fn create_todo(todo: Form<TodoForm>) -> TodoTemplate {
    let num = rand::random::<usize>();
    let value = format!("{}{}", "a", num);

    let todo_html = TodoTemplate {
        value,
        todo: todo.todo
    };

    todo_html
}

#[delete("/todo")]
fn delete_todo() -> &'static str {
    ""
}

