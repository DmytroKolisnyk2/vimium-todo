mod user;
mod task;
mod auth;
mod utils;

use user::User;
use auth::AuthenticationManager;
use task::TaskManager;
use utils::prompt;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let auth_manager = AuthenticationManager::new();
    let task_manager = TaskManager::new(auth_manager.clone());

    loop {
        println!("1. Register");
        println!("2. Login");
        println!("3. Add Task");
        println!("4. List My Tasks");
        println!("5. List Completed Tasks");
        println!("6. Complete Task");
        println!("7. Delete Task");
        println!("8. Logout");
        println!("9. Exit");

        let choice = prompt("Enter your choice: ");

        match choice.trim() {
            "1" => {
                let username = prompt("Enter username: ");
                let email = prompt("Enter email: ");
                let password = prompt("Enter password: ");
                match User::register(&username, &email, &password).await {
                    Ok(_) => println!("User registered successfully."),
                    Err(err) => println!("Registration failed: {}", err),
                }
            },
            "2" => {
                let identifier = prompt("Enter username or email: ");
                let password = prompt("Enter password: ");
                match auth_manager.login(&identifier, &password).await {
                    Ok(token) => println!("Logged in with token: {}", token),
                    Err(err) => println!("Login failed: {}", err),
                }
            },
            "3" => {
                let name = prompt("Enter task name: ");
                let description = prompt("Enter task description: ");
                match task_manager.add_task(&name, &description).await {
                    Ok(_) => println!("Task added successfully."),
                    Err(err) => println!("Failed to add task: {}", err),
                }
            },
            "4" => {
                match task_manager.list_my_tasks().await {
                    Ok(tasks) => {
                        if tasks.is_empty() {
                            println!("No tasks found.");
                        } else {
                            for task in tasks {
                                println!("{}", task_card(&task));
                            }
                        }
                    },
                    Err(err) => println!("Failed to list tasks: {}", err),
                }
            },
            "5" => {
                match task_manager.list_completed_tasks().await {
                    Ok(tasks) => {
                        if tasks.is_empty() {
                            println!("No completed tasks found.");
                        } else {
                            for task in tasks {
                                println!("{}", task_card(&task));
                            }
                        }
                    },
                    Err(err) => println!("Failed to list completed tasks: {}", err),
                }
            },
            "6" => {
                let task_id = prompt("Enter task ID to complete: ").parse().expect("Invalid ID format.");
                match task_manager.complete_task(task_id).await {
                    Ok(_) => println!("Task completed."),
                    Err(err) => println!("Failed to complete task: {}", err),
                }
            },
            "7" => {
                let task_id = prompt("Enter task ID to delete: ").parse().expect("Invalid ID format.");
                match task_manager.delete_task(task_id).await {
                    Ok(_) => println!("Task deleted."),
                    Err(err) => println!("Failed to delete task: {}", err),
                }
            },
            "8" => {
                auth_manager.logout();
                println!("Logged out.");
            },
            "9" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn task_card(task: &task::Task) -> String {
    format!(
        "╔════════════════════════════════════════╗\n\
         ║ ID: {:<34} ║\n\
         ║ Name: {:<32} ║\n\
         ║ Description: {:<25} ║\n\
         ║ Completed: {:<27} ║\n\
         ╚════════════════════════════════════════╝",
        task.id, task.name, task.description, task.is_completed
    )
}