#[cfg(test)]
mod tests {
    use std::env;
    use tokio::runtime::Runtime;
    use vimium_todo_client::auth::AuthenticationManager;
    

    #[test]
    fn test_login_success() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let server_url =
                env::var("SERVER_URL").unwrap_or_else(|_| String::from("http://localhost:1337"));
            let auth_manager = AuthenticationManager::new(Some(&server_url));

            let result = auth_manager.login("username", "username").await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), auth_manager.get_token().unwrap());
        });
    }

    #[test]
    fn test_login_failure() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let server_url =
                env::var("SERVER_URL").unwrap_or_else(|_| String::from("http://localhost:1337"));
            let auth_manager = AuthenticationManager::new(Some(&server_url));

            let result = auth_manager.login("username", "wrong_password").await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Invalid identifier or password");
        });
    }

    #[test]
    fn test_logout() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let server_url =
                env::var("SERVER_URL").unwrap_or_else(|_| String::from("http://localhost:1337"));
            let auth_manager = AuthenticationManager::new(Some(&server_url));

            auth_manager.login("username", "username").await.unwrap();
            auth_manager.logout();
            assert!(auth_manager.get_token().is_none());
        });
    }
}
