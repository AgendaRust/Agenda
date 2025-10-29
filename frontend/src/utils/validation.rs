/// Validation utilities for user input

/// Validates username
/// Returns empty Vec if valid, Vec of error messages if invalid
pub fn validate_username(username: &str) -> Vec<String> {
    let mut errors = Vec::new();
    
    if username.len() < 6 {
        errors.push("O nome de usuário deve ter pelo menos 6 caracteres".to_string());
    }
    
    errors
}

/// Validates password
/// Returns empty Vec if valid, Vec of error messages if invalid
pub fn validate_password(password: &str) -> Vec<String> {
    let mut errors = Vec::new();
    
    if password.len() < 8 {
        errors.push("Senha curta, no mínimo 8 dígitos".to_string());
    }
    
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    
    if !has_lowercase {
        errors.push("Pelo menos uma letra minúscula".to_string());
    }
    
    if !has_uppercase {
        errors.push("Pelo menos uma letra maiúscula".to_string());
    }
    
    if !has_digit {
        errors.push("Pelo menos um número".to_string());
    }
    
    errors
}

/// Validates both username and password
/// Returns (username_errors, password_errors)
pub fn validate_credentials(username: &str, password: &str) -> (Vec<String>, Vec<String>) {
    (validate_username(username), validate_password(password))
}
