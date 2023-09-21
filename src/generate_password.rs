use rand::Rng;

pub fn generate_password(use_uppercase: bool, use_lowercase: bool, use_symbols: bool, use_numbers: bool, length: i32) -> String {
    let uppercase_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lowercase_chars = "abcdefghijklmnopqrstuvwxyz";
    let symbol_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
    let number_chars = "0123456789";

    let mut password = String::new();

    if use_uppercase {
        password.push_str(uppercase_chars);
    }

    if use_lowercase {
        password.push_str(lowercase_chars);
    }

    if use_symbols {
        password.push_str(symbol_chars);
    }

    if use_numbers {
        password.push_str(number_chars);
    }

    if password.is_empty() {
        // Если ни один параметр не выбран, верните сообщение об ошибке
        return "Выберите хотя бы один параметр для генерации пароля.".to_string();
    }

    // Генерация случайного пароля заданной длины
    let mut rng = rand::thread_rng();
    let password_chars: Vec<char> = password.chars().collect();
    let mut result_password = String::new();

    for _ in 0..length {
        let random_index = rng.gen_range(0..password_chars.len());
        result_password.push(password_chars[random_index]);
    }

    result_password
}
