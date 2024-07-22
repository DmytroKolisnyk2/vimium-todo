use crate::auth::AuthenticationManager;
use crate::task::TaskManager;
use crate::console::{prompt, log_info, log_error, prompt_integer};

pub async fn show_menu(auth_manager: &AuthenticationManager, task_manager: &TaskManager) {
    let menu_items = vec![
        "1. Register",
        "2. Login",
        "3. Add Task",
        "4. List My Tasks",
        "5. List Completed Tasks",
        "6. Complete Task",
        "7. Delete Task",
        "8. Logout",
        "9. Exit",
    ];

    loop {
        for item in &menu_items {
            println!("{}", item);
        }

        let choice = prompt("Enter your choice: ").trim().to_string();

        match choice.as_str() {
            "1" => register_user(auth_manager).await,
            "2" => user_login(auth_manager).await,
            "3" => add_task(task_manager).await,
            "4" => list_my_tasks(task_manager).await,
            "5" => list_completed_tasks(task_manager).await,
            "6" => complete_task(task_manager).await,
            "7" => delete_task(task_manager).await,
            "8" => logout(auth_manager),
            "9" => break,
            _ => log_info("Invalid choice. Please try again."),
        }
    }
}

async fn register_user(auth_manager: &AuthenticationManager) {
    let username = prompt("Enter username: ");
    let email = prompt("Enter email: ");
    let password = prompt("Enter password: ");
    match auth_manager.register(&username, &email, &password).await {
        Ok(_) => log_info("User registered successfully."),
        Err(err) => log_error(&format!("Registration failed: {}", err)),
    }
}

async fn user_login(auth_manager: &AuthenticationManager) {
    let identifier = prompt("Enter username or email: ");
    let password = prompt("Enter password: ");
    match auth_manager.login(&identifier, &password).await {
        Ok(_) => log_info("Logged in successfully"),
        Err(err) => log_error(&format!("Login failed: {}", err)),
    }
}

async fn add_task(task_manager: &TaskManager) {
    let name = prompt("Enter task name: ");
    let description = prompt("Enter task description: ");
    match task_manager.add_task(&name, &description).await {
        Ok(_) => log_info("Task added successfully."),
        Err(err) => log_error(&format!("Failed to add task: {}", err)),
    }
}

async fn list_my_tasks(task_manager: &TaskManager) {
    match task_manager.list_my_tasks().await {
        Ok(_) => {},
        Err(err) => log_error(&format!("Failed to list tasks: {}", err)),
    }
}

async fn list_completed_tasks(task_manager: &TaskManager) {
    match task_manager.list_completed_tasks().await {
        Ok(_) => {},
        Err(err) => log_error(&format!("Failed to list completed tasks: {}", err)),
    }
}

async fn complete_task(task_manager: &TaskManager) {
    let task_id = prompt_integer("Enter task ID to complete: ");
    match task_manager.complete_task(task_id).await {
        Ok(_) => log_info("Task completed."),
        Err(err) => log_error(&format!("Failed to complete task: {}", err)),
    }
}

async fn delete_task(task_manager: &TaskManager) {
    let task_id = prompt_integer("Enter task ID to delete: ");
    match task_manager.delete_task(task_id).await {
        Ok(_) => {},
        Err(err) => log_error(&format!("Failed to delete task: {}", err)),
    }
}

fn logout(auth_manager: &AuthenticationManager) {
    auth_manager.logout();
    log_info("Logged out successfully.");
}