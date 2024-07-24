mod task;
mod auth;
mod console;
mod parser;
mod menu;

use auth::AuthenticationManager;
use task::TaskManager;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let auth_manager = AuthenticationManager::new(None);
    let task_manager = TaskManager::new(auth_manager.clone());

    menu::show_menu(&auth_manager, &task_manager).await;
}
