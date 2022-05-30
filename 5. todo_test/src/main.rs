use std::io;

struct Todo {
    subject: String,
    checked: bool,
}

fn main() {
    let mut todos: Vec<Todo> = vec![];

    loop {
        println!("Choose menu, 1. add, 2. list, 3. toggle, 4. exit");
        // mut는 가변 여부
        // let guess = String::new(); 일 경우 가변이 안되는 상수
        let mut choose = String::new();

        io::stdin()
            .read_line(&mut choose)
            .expect("Failed to read line");

        let choose: u32 = match choose.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You selected: {}", choose);

        match choose {
            1 => {}
            2 => {
                println!("Current Todo List: ");
                for todo in todos {
                    println!("subject: {}, checked: {}", todo.subject, todo.checked);
                }
            }
            3 => {
                println!("Toggle Number: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: usize = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if todos.len() > id {
                    todos[id] = Todo {
                        subject: todos[id].subject,
                        checked: !todos[id].checked,
                    };
                } else {
                    println!("error")
                }
            }
            4 => {
                println!("bye 😊");
                break;
            }
            _ => {
                println!("Please input 1~3");
            }
        }
    }
}
