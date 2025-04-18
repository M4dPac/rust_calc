use std::io;

use calculator::{error::CalcError, parser, rpn};

fn main() {
    run_repl().unwrap();
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

fn run_repl() -> Result<(), CalcError> {
    println!("Введите выражение (или 'exit' для выхода):");
    loop {
        let input = read_input();

        if &input == "exit" {
            break;
        }

        let tokens = parser::tokenize(&input)?;
        parser::validate_parens(&tokens)?;
        let rpn = rpn::to_rpn(tokens)?;
        let result = rpn::eval_rpn(rpn)?;

        println!("{:?}", result);
    }

    Ok(())
}
