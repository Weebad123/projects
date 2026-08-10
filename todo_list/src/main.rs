use std::io::{self, Write};
use std::fs::{self, File};


use serde::{Serialize, Deserialize};

// Define Task struct

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoTasks {
    all_tasks: Vec<Task>
}
impl TodoTasks {
    fn new(size: usize) -> Self {
        Self {
            all_tasks: /*Vec::with_capacity(size)*/Vec::new()
        }
    }
}

/*
1. create a task and add it to the Todo
*/
fn main() {
    println!("Simple ToDo List App");

    //let tasks = load_tasks("file1.json");
    loop {
        println!("\nTo-Do List Menu");
        println!("1. Add a Task");
        println!("2. View Task");
        println!("3. Mark task as completed");
        println!("4. Delete Task");
        println!("5. Update Task Description");
        println!("6. Save Task");

        let user_choice = get_input("Enter Your Action");
        //let parsed_user_choice = parsed_id(id)
        //let tasks = load_tasks("some_shit.json");
        let mut my_task = TodoTasks::new(2);
        match user_choice.trim() {
            "1" => {
                println!("Adding task");
                my_task = add_task(/*&get_input("Enter filename")*/);
            },
            "6" => save_tasks(my_task, "some_shit.json"),
            //"2" => view_tasks(&tasks),
            _ => {
                println!("Incorrect choice");
                break;
            }
        }
    }
}


fn get_input(inp: &str) -> String {
    println!("{}: ", inp);
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to get user input");
    
    user_input
}

fn parsed_id(id: String) -> Option<usize> {
    let parsed_id = match id.trim().parse::<usize>() {
        Ok(id) => Some(id),
        Err(_) => {
            println!("Invalid task Id. Task Id must be a positive integer");
            return None;
        }
    };
    parsed_id
}
fn load_tasks(file_name_containing_task: &str) -> /*Vec<Task>*/TodoTasks {

    match fs::read_to_string(file_name_containing_task) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| TodoTasks::new(0)),
        Err(_) => /*Vec::new()*/TodoTasks::new(0),
    }
}

fn save_tasks(tasks: TodoTasks, filename: &str) {
    //let all_tasks = &tasks.all_tasks;
    let json = serde_json::to_string_pretty(&tasks.all_tasks).expect("Failed to serialize tasks");
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write tasks to file");
}

fn add_task(/*filename: &str*/) -> TodoTasks {
    let mut all_todo = TodoTasks::new(10);
    // Get input for description, automatic id generation
    let description = get_input("Enter task description");
    let id = all_todo.all_tasks.len() + 1;
    let completed = false;

    all_todo.all_tasks.push( Task {
        id,
        description,
        completed
    });

    //save_tasks(all_todo, &get_input("Enter filename here"));
    all_todo
}

fn view_tasks(tasks: &TodoTasks) {
    if tasks.all_tasks.is_empty() {
        println!("No Task Found!");
    } else {
        for each_task in tasks.all_tasks.iter() {
            let status = if each_task.completed {"Completed"} else {"Not Complted"};
            println!(
                "{} - {}: {}",
                each_task.id, each_task.description, status
            );
        }
    }
}

fn update_task_description(all_task: TodoTasks) {
    let task_id = get_input("Please enter the task Id you wish to update");
    let description = get_input("Enter the new description!");
    let mut tasks = all_task.all_tasks;
   
    let parsed_task_id = parsed_id(task_id);
    

    if let Some(task) = tasks.get_mut(parsed_task_id.unwrap()) {
        task.description = description;
    } else {
        println!("Non-existent task id");
    }
}

fn mark_task_as_completed(tasks: TodoTasks) {
    
    // get id from user
    let task_id = get_input("Enter the task Id you wish to mark as completed!");
    let parsed_task_id = parsed_id(task_id);
    let mut all_tasks = tasks.all_tasks;
    if let Some(task) = all_tasks.get_mut(parsed_task_id.unwrap()) {
        task.completed = true;
    }

}

fn delete_task(tasks: TodoTasks) {
    let task_id = get_input("Enter task id you wish to delete!");
    let parsed_id = parsed_id(task_id);
    let mut all_tasks = tasks.all_tasks;
    let removed_task = all_tasks.remove(parsed_id.unwrap());
    println!("Removed Task: {:?}", removed_task);
}