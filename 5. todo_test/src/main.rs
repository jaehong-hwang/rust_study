struct Todo {
    subject: String,
    checked: bool,
}

fn main() {
    let todos: Todo = Todo {
        subject: String::from("test"),
        checked: true,
    };

    println!("subject: {}, checked: {}", todos.subject, todos.checked);
}
