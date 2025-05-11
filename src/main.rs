use calculator::{error::CalcError, output, parser, rpn};
use std::io;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if output::is_not_interactive() {
        // Режим CLI
        // FIX: сделать обработку передачи выражения с пробелами или заключатъ выражение в ""
        let input = args[1].trim();
        match run_repl(input) {
            Ok(num) => println!("{}", num),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Итерактивный режим
    run_repl_interactive().unwrap();
}

fn run_repl_interactive() -> Result<(), CalcError> {
    output::print_prompt();
    loop {
        let input = read_input();
        if &input == "exit" {
            break;
        }

        match run_repl(&input) {
            Ok(num) => output::print_result(num),
            Err(e) => output::print_error(&e.to_string()),
        }
    }

    Ok(())
}

fn read_input() -> String {
    loop {
        let mut s = String::new();
        if io::stdin().read_line(&mut s).is_err() {
            eprintln!("Ошибка чтения ввода.");
            continue;
        }
        return s.trim().to_owned();
    }
}

/// Обрабатывает выражение и возвращает результат
fn run_repl(input: &str) -> Result<f64, CalcError> {
    let tokens = parser::tokenize(input)?;
    parser::validate_parens(&tokens)?;
    let rpn = rpn::to_rpn(tokens)?;
    rpn::eval_rpn(rpn)
}

#[cfg(test)]
mod tests_run_repl {
    use super::*;

    #[test]
    fn test_simple_expression() {
        assert_eq!(run_repl("2 + 3").unwrap(), 5.0);
        assert_eq!(run_repl("10 - 4").unwrap(), 6.0);
        assert_eq!(run_repl("3 * 4").unwrap(), 12.0);
        assert_eq!(run_repl("8 / 2").unwrap(), 4.0);
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(run_repl("2 + 3 * 4").unwrap(), 14.0); // 2 + (3*4)
        assert_eq!(run_repl("10 - 2 * 3").unwrap(), 4.0); // 10 - (2*3)
        assert_eq!(run_repl("1 + 2 * 3 - 4").unwrap(), 3.0); // 1 + (2*3) - 4
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(run_repl("(2 + 3) * 4").unwrap(), 20.0); // (2+3)*4
        assert_eq!(run_repl("((2 + 3) * 4) - 5").unwrap(), 15.0); // ((2+3)*4)-5
        assert_eq!(run_repl("2 * (3 + (4 * 5))").unwrap(), 46.0); // 2*(3+(4*5))
    }

    #[test]
    fn test_unary_minus() {
        assert_eq!(run_repl("-5").unwrap(), -5.0);
        assert_eq!(run_repl("2 + (-3)").unwrap(), -1.0);
        assert_eq!(run_repl("-(-4)").unwrap(), 4.0);
        assert_eq!(run_repl("2^(-1)").unwrap(), 0.5);
    }

    #[test]
    fn test_power_right_associativity() {
        assert_eq!(run_repl("2^3^2").unwrap(), 512.0); // 2^(3^2)
        assert_eq!(run_repl("2^(3^2)").unwrap(), 512.0);
        assert_eq!(run_repl("(2^3)^2").unwrap(), 64.0);
        assert_eq!(run_repl("3^2^2").unwrap(), 81.0); // 3^(2^2) = 3^4
    }

    #[test]
    fn test_divide_by_zero() {
        let err = run_repl("1 / 0").unwrap_err();
        assert_eq!(err.to_string(), "Деление на 0.");
    }

    #[test]
    fn test_invalid_tokens() {
        let err = run_repl("2 + abc").unwrap_err();
        assert!(err.to_string().contains("Некорректный символ"));

        let err = run_repl("1.2.3").unwrap_err();
        assert!(err.to_string().contains("Некорректный символ"));

        let err = run_repl("1 + 2 *").unwrap_err();
        assert!(err.to_string().contains("Некорректное выражение"));
    }

    #[test]
    fn test_unmatched_parens() {
        let err = run_repl("(2 + 3").unwrap_err();
        assert_eq!(err.to_string(), "Не совпадают скобки.");

        let err = run_repl("2 + 3)").unwrap_err();
        assert_eq!(err.to_string(), "Не совпадают скобки.");

        let err = run_repl("((2 + 3) * 4").unwrap_err();
        assert_eq!(err.to_string(), "Не совпадают скобки.");
    }

    #[test]
    fn test_mixed_operations() {
        assert_eq!(run_repl("2 + 3 * (4 - 1)^2").unwrap(), 29.0); // 2 + 3*(3^2)
        assert_eq!(run_repl("10 / (2 + 3) * 4").unwrap(), 8.0); // (10/5)*4
        assert_eq!(run_repl("-2^3").unwrap(), -8.0); // -(2^3)
        assert_eq!(run_repl("(-2)^3").unwrap(), -8.0); // (-2)^3
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(run_repl("1234567890 + 987654321").unwrap(), 2222222211.0);
        // assert_eq!(run_repl("1e6 * 1e3").unwrap(), 1_000_000_000.0); // 10^6 * 10^3
        assert_eq!(run_repl("2^10").unwrap(), 1024.0);
    }

    #[test]
    fn test_edge_cases() {
        // Пустое выражение
        let err = run_repl("").unwrap_err();
        assert!(matches!(err, CalcError::InvalidExpression(_)));
        assert_eq!(
            err.to_string(),
            "Некорректное выражение: Стек пуст после вычислений"
        );

        // Незакрытая скобка
        let err = run_repl(")").unwrap_err();
        assert!(matches!(err, CalcError::UnmatchedParens));
        assert_eq!(err.to_string(), "Не совпадают скобки.");

        // Некорректный оператор
        let err = run_repl("1 + 2 * / 3").unwrap_err();
        assert!(matches!(err, CalcError::InvalidExpression(_)));
        assert_eq!(
            err.to_string(),
            "Некорректное выражение: Недостаточно операндов для операции 'Plus'"
        );

        // Деление на ноль
        let err = run_repl("1 / 0").unwrap_err();
        assert!(matches!(err, CalcError::DivideByZero));
        assert_eq!(err.to_string(), "Деление на 0.");

        // Недостаточно операндов для операции
        let err = run_repl("1 +").unwrap_err();
        assert!(matches!(err, CalcError::InvalidExpression(_)));
        assert_eq!(
            err.to_string(),
            "Некорректное выражение: Недостаточно операндов для операции 'Plus'"
        );

        // Унарная операция без операнда
        let err = run_repl("-").unwrap_err();
        assert!(matches!(err, CalcError::InvalidExpression(_)));
        assert_eq!(
            err.to_string(),
            "Некорректное выражение: Унарный минус требует одного операнда"
        );

        // Лишние числа в стеке
        let err = run_repl("1 2 + 2").unwrap_err();
        assert!(matches!(err, CalcError::InvalidExpression(_)));
        assert_eq!(
            err.to_string(),
            "Некорректное выражение: В стеке остались лишние числа"
        );

        // Некорректный токен
        let err = run_repl("abc").unwrap_err();
        assert!(matches!(err, CalcError::InvalidToken(_)));
        assert_eq!(
            err.to_string(),
            "Некорректный символ: Некорректный символ в выражении: 'a'"
        );

        // Несколько точек в числе
        let err = run_repl("1.2.3").unwrap_err();
        assert!(matches!(err, CalcError::InvalidToken(_)));
        assert_eq!(err.to_string(), "Некорректный символ: 1.2.3");

        // Незакрытые скобки в начале выражения
        let err = run_repl("((2 + 3)").unwrap_err();
        assert!(matches!(err, CalcError::UnmatchedParens));
        assert_eq!(err.to_string(), "Не совпадают скобки.");

        // Лишние закрывающие скобки
        let err = run_repl("2 + 3))").unwrap_err();
        assert!(matches!(err, CalcError::UnmatchedParens));
        assert_eq!(err.to_string(), "Не совпадают скобки.");

        // Оператор в конце выражения без операндов
        let err = run_repl("5 + 2 *").unwrap_err();
        assert!(matches!(err, CalcError::InvalidExpression(_)));
        assert_eq!(
            err.to_string(),
            "Некорректное выражение: Недостаточно операндов для операции 'Plus'"
        );
    }
}
