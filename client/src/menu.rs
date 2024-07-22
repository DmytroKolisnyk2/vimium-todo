use crate::auth::AuthenticationManager;
use crate::task::TaskManager;
use crate::console::{prompt, log_info, log_error};

pub async fn show_menu(auth_manager: &AuthenticationManager, task_manager: &TaskManager) {
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
                match auth_manager.register(&username, &email, &password).await {
                    Ok(_) => log_info("User registered successfully."),
                    Err(err) => log_error(&format!("Registration failed: {}", err)),
                }
            },
            "2" => {
                let identifier = prompt("Enter username or email: ");
                let password = prompt("Enter password: ");
                match auth_manager.login(&identifier, &password).await {
                    Ok(_) => log_info("Logged in successfully"),
                    Err(err) => log_error(&format!("Login failed: {}", err)),
                }
            },
            "3" => {
                let name = prompt("Enter task name: ");
                let description = prompt("Enter task description: ");
                match task_manager.add_task(&name, &description).await {
                    Ok(_) => log_info("Task added successfully."),
                    Err(err) => log_error(&format!("Failed to add task: {}", err)),
                }
            },
            "4" => {
                match task_manager.list_tasks("api/tasks/my", "No tasks found.").await {
                    Ok(_) => {},
                    Err(err) => log_error(&format!("Failed to list tasks: {}", err)),
                }
            },
            "5" => {
                match task_manager.list_tasks("api/tasks/completed", "No completed tasks found.").await {
                    Ok(_) => {},
                    Err(err) => log_error(&format!("Failed to list completed tasks: {}", err)),
                }
            },
            "6" => {
                let task_id = prompt("Enter task ID to complete: ").parse().expect("Invalid ID format.");
                match task_manager.complete_task(task_id).await {
                    Ok(_) => log_info("Task completed."),
                    Err(err) => log_error(&format!("Failed to complete task: {}", err)),
                }
            },
            "7" => {
                let task_id = prompt("Enter task ID to delete: ").parse().expect("Invalid ID format.");
                match task_manager.delete_task(task_id).await {
                    Ok(_) => log_info("Task deleted."),
                    Err(err) => log_error(&format!("Failed to delete task: {}", err)),
                }
            },
            "8" => {
                auth_manager.logout();
                log_info("Logged out.");
            },
            "9" => break,
            _ => log_info("Invalid choice. Please try again."),
        }
    }
}
