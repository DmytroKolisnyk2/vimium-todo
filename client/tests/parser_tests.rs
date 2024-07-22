#[cfg(test)]
mod tests {
    use vimium_todo_client::parser::parse_error; 


    #[test]
    fn test_parse_error_valid() {
        let json_error = r#"{"error": {"message": "An error occurred"}}"#;
        assert_eq!(parse_error(json_error), "An error occurred");
    }

    #[test]
    fn test_parse_error_invalid() {
        let json_error = "Not a JSON";
        assert_eq!(parse_error(json_error), "Not a JSON");
    }
}
