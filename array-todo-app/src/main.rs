use std::io;

fn main() {
    let mut todos = vec!["Bath", "Eat"];
    println!("Welcome to our rust todo app \n");
    println!("1. Add new todo");
    print!("2. Edit a todo. \n");
    println!("3. Delete a todo. \n");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Please select an option !!");
    let option: u32 = option.trim().parse().expect("An option is needed");
    if option == 1 {
        println!("Please enter a new todo: \n");
        let mut new_todo = String::new();
        io::stdin()
            .read_line(&mut new_todo)
            .expect("Failed to get todo");
        let new_todo: String = new_todo.trim().parse().expect("Please enter a todo !!!");
        todos.push(&new_todo);
        println!("Todo added successfully");
        println!("{:?}", &todos);
    } else if option == 2 {
        println!("Please enter the todo's index: ");
        let mut todo_index = String::new();
        io::stdin()
            .read_line(&mut todo_index)
            .expect("Failed to get todo index");
        match todo_index.trim().parse::<i32>() {
            Ok(index) => {
                if index < 0 {
                    println!("Exiting the program.");
                }
                // Use the todo_index to locate and log the item
                match todos.get(index as usize) {
                    Some(todo) => {
                        println!("The todo at index {} is: {}", index, todo);
                        println!("Please enter the new todo: ");
                        let mut new_todo = String::new();
                        io::stdin()
                            .read_line(&mut new_todo)
                            .expect("Failed to get new todo");
                        let new_todo: String = new_todo
                            .trim()
                            .parse()
                            .expect("Please enter to update todo !!!");
                        todos[index as usize] = &new_todo;
                        println!("Todo updated successfully");
                        println!("{:?}", &todos);
                    }
                    None => println!("Index {} is out of bounds!", index),
                }
            }
            Err(_) => println!("That's not a valid index. Please try again."),
        }
    } else {
        println!("Please enter the todo's index: ");
        let mut todo_index = String::new();
        io::stdin()
            .read_line(&mut todo_index)
            .expect("Failed to get todo index");
        let todo_index: u32 = todo_index.trim().parse().expect("Index needed !!!");
        todos.remove(todo_index as usize);
        println!("Todo deleted successfully");
        println!("{:?}", &todos);
    }
}
