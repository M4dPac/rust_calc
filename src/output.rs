// ANSI-коды для цветов
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const RESET: &str = "\x1b[0m";

// Проверяет, запущен ли в интерактивном режиме
pub fn is_not_interactive() -> bool {
    std::env::args().len() > 1
}

// Проверяет, поддерживает ли терминал ANSI-цвета
pub fn supports_ansi() -> bool {
    !cfg!(windows) || std::env::var("TERM").is_ok()
}

// Форматированный вывод ошибок
pub fn print_error(message: &str) {
    if supports_ansi() {
        eprintln!("{}Error:{} {}", RED, RESET, message);
    } else {
        eprintln!("Error: {}", message)
    }
}

// Форматированный вывод результата
pub fn print_result(result: f64) {
    if supports_ansi() {
        println!("{}Результат: {}{}", GREEN, result, RESET);
    } else {
        println!("{}", result)
    }
}

// Форматированный вывод приглашения
pub fn print_prompt() {
    if supports_ansi() {
        println!(
            "{}Введите выражение (или 'exit' для выхода):{}",
            YELLOW, RESET
        );
    } else {
        println!("Введите выражение (или 'exit' для выхода):");
    }
}
