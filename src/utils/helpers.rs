//Returing filtered alphanumeric characters string
pub fn sanitize_name(name: String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}
