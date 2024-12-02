use crate::utils::helpers::validate_input;

// Verify command
pub async fn verify_token(path_or_url: &str) {
    println!("Verifying token at: {}", path_or_url);

    let (status, error_message, input_type) = validate_input(path_or_url);

    if !status {
        eprintln!("{}", error_message);
        return;
    }

    if let Some(input_type) = input_type {
        println!("The input is valid and is of type: {} -> {}", input_type, path_or_url);
    }

}