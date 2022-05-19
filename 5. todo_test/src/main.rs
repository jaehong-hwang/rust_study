struct Todo {
    subject: String,
    checked: bool,
}

fn main() {
    let todos: &[&Todo] = &[
        &Todo {
            subject: String::from("test"),
            checked: true,
        },
        &Todo {
            subject: String::from("test2"),
            checked: false,
        },
    ];

    for todo in todos {
        println!("subject: {}, checked: {}", todo.subject, todo.checked);
    }
}
