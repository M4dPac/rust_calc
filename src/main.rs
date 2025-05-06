use calculator::{error::CalcError, parser, rpn};
use std::io;

fn main() {
    loop {
        println!("Введите выражение (или 'exit' для выхода):");
        let input = read_input();
        if &input == "exit" {
            break;
        }
        match run_repl(&input) {
            Ok(num) => println!("{}", num),
            Err(e) => eprintln!("{}", e),
        }
    }
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
