#[cfg(test)]
mod task_tests {
    use std::env;
    use vimium_todo_client::auth::AuthenticationManager;
    use vimium_todo_client::task::TaskManager;

    async fn setup() -> TaskManager {
        let server_url =
            env::var("SERVER_URL").unwrap_or_else(|_| String::from("http://localhost:1337"));
        let auth_manager = AuthenticationManager::new(Some(&server_url));
        auth_manager.login("username", "username").await.unwrap();
        TaskManager::new(auth_manager)
    }

    #[tokio::test]
    async fn test_add_task_success() {
        let task_manager = setup().await;
        let result = task_manager
            .add_task("New Task", "Description of the new task")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_my_tasks() {
        let task_manager = setup().await;
        let result = task_manager.list_my_tasks().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_completed_tasks() {
        let task_manager = setup().await;
        let result = task_manager.list_completed_tasks().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_unlogged_add_task() {
        let task_manager = TaskManager::new(AuthenticationManager::new(None));
        let result = task_manager
            .add_task("New Task", "Description of the new task")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlogged_list_my_tasks() {
        let task_manager = TaskManager::new(AuthenticationManager::new(None));
        let result = task_manager.list_my_tasks().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlogged_list_completed_tasks() {
        let task_manager = TaskManager::new(AuthenticationManager::new(None));
        let result = task_manager.list_completed_tasks().await;
        assert!(result.is_err());
    }
}
