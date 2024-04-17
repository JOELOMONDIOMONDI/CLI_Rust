// use std::{env, ops::Index};
// use alloc::task;
use std::{io, process::Command};

// STRUCTURE FOR THE TASK INPUT
struct Tasks {
    id: u32,
    title: String,
    description: String,
    status: TodoStatus,
    due_date: Option<String>,
}

enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

struct TodoList {
    item: Vec<Tasks>,
}

fn main() {
 
    println!("WELCOME TO THE TODO LIST APPLICATION");
    let mut current_max_id = 1;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut command: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    match command {
        1 => add_task(&mut todo_list, &mut current_max_id),
        2 => remove_task(),
        3 => list_tasks(),
        4 => save_load_tasks(),
        5 => show_status(),
        _ => panic!("Invalid function chosen"),
    }
}

// Function to add tasks to the todoList Application
fn add_task(todo_list: &mut TodoList, current_max_id: &mut u32) {
    let id = *current_max_id + 1;

    println!("Input the title");
    let mut title: String = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read title");
    let title = title.trim().to_string();

    println!("Input the description");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read the description");
    let description = description.trim().to_string();

    println!("Task Status (0 for Pending, 1 for InProgress, 2 for Completed):");
    let mut status_input = String::new();
    io::stdin()
        .read_line(&mut status_input)
        .expect("Failed to read task status");
    let status: TodoStatus = match status_input.trim().parse().expect("Invalid task status") {
        0 => TodoStatus::Pending,
        1 => TodoStatus::InProgress,
        2 => TodoStatus::Completed,
        _ => panic!("Invalid task status"),
    };

    println!("Input the due date if not leave it empty");
    let mut due_date = String::new();
    io::stdin()
        .read_line(&mut due_date)
        .expect("Failed to read due date");
    let due_date = due_date.trim().to_string();
    let due_date = if due_date.is_empty() {
        None
    } else {
        Some(due_date)
    };

    // Create the new task and add it to the vector
    let new_task = Tasks {
        id,
        title,
        description,
        status,
        due_date,
    };

    item.push(new_task);

    *current_max_id += 1; // Increment the current maximum ID
}

// Function to remove tasks in the todo with a spesific ID
fn remove_task() {}

// Function to List all the task sin the List
fn list_tasks() {}

// Function to Save sessions of pending tasks
fn save_load_tasks() {}

fn show_status() {

    //    let value=match TodoStatus {
    //         TodoStatus::Pending => println!("The status is pending."),
    //         TodoStatus::InProgress=> println!("The status is in progress."),
    //         TodoStatus::Completed => println!("The status is completed."),

    //     }
}
