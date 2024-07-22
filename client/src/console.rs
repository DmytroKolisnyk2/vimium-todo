use std::io::{self, Write};

pub fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn log_info(message: &str) {
    println!(
        "\n\
         ╔════════════════════════════════════════╗\n\
         ║ {:<38} ║\n\
         ╚════════════════════════════════════════╝\n",
        message
    );
}

pub fn log_error(message: &str) {
    eprintln!(
        "\n\
         ╔═══════════════════════════════════════════════════════════════════════════════╗\n\
         ║ ERROR: {:<70} ║\n\
         ╚═══════════════════════════════════════════════════════════════════════════════╝\n",
        message
    );
}


pub fn log_task(task: &crate::task::Task) {
    println!(
        "╔════════════════════════════════════════╗\n\
         ║ ID: {:<34} ║\n\
         ║ Name: {:<32} ║\n\
         ║ Description: {:<25} ║\n\
         ║ Completed: {:<27} ║\n\
         ╚════════════════════════════════════════╝",
        task.id, task.name, task.description, task.is_completed
    );
}
