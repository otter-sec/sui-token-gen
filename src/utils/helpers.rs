use colored::*;

// Returing filtered alphanumeric characters string
pub fn sanitize_name(name: &String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

//Success log
pub fn log_success_message(message: &str){
    let success  = "SUCCESS: ".green();
    println!("{} {}", success, message);
}

//Error log
pub fn log_error_message(message: &str){
    let error  = "ERROR: ".red();
    println!("{} {}", error, message);
}
