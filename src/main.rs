use std::io;
fn main() {
    run_repl();
}

fn read_input() -> String {
    loop {
        let mut s = String::new();
        if io::stdin().read_line(&mut s).is_err() {
            eprintln!("Ошибка ввода.");
            continue;
        }

        return s.trim().to_owned();
    }
}

fn run_repl() {
    loop {
        let input = read_input();
    }
}
