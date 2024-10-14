use rand::Rng;
use std::collections::HashMap;
use std::io;

struct Todo {
    title: String,
    description: String,
    id: u32,
}
fn main() {
    let mut _todos: HashMap<u32, Todo> = HashMap::new();
    println!("Welcome to our rust todo app \n");
    println!("1. Add new todo");
    print!("2. Edit a todo \n");
    println!("3. Delete a todo");
    println!("4. Display todo list ");

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Please select an option");
    match option.trim().parse::<i32>() {
        Ok(option) => {
            if option <= 0 {
                println!("Please enter a valid option")
            } else if option == 1 {
                add_todo(&mut _todos);
            } else if option == 2 {
                edit_todo();
            } else if option == 3 {
                delete_todo();
            } else {
                display_todo(&mut _todos);
            }
        }
        Err(_) => println!("An error occured"),
    }
}

fn display_todo(_todos: &mut HashMap<u32, Todo>) {
    for (key, todo) in _todos.iter() {
        println!("Title: {}, Description: {}", todo.title, todo.description);
    }
}

fn add_todo(_todos: &mut HashMap<u32, Todo>) {
    let mut title = String::new();
    let mut description = String::new();
    println!("Enter title:");
    io::stdin()
        .read_line(&mut title)
        .expect("Please enter a todo title");
    println!("Enter description:");
    io::stdin()
        .read_line(&mut description)
        .expect("Please enter a todo description");

    let new_todo = Todo {
        title: title,
        description: description,
        id: rand::thread_rng().gen_range(1..=1000),
    };

    // insert todo to hashmap
    _todos.insert(new_todo.id.clone(), new_todo);
    println!("Todos added successfully")
}

fn edit_todo() {}

fn delete_todo() {}
