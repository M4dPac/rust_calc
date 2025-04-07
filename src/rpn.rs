use crate::{error::CalcError, parser::Token};

/// Алгоритм сортировочной станции (Shunting-yard)
pub fn to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, CalcError> {
    let mut output: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            _ => {
                while let Some(top) = operators.last() {
                    if top.precedence() >= token.precedence() {
                        output.push(match operators.pop() {
                            Some(op) => op,
                            None => todo!(),
                        });
                    } else {
                        break;
                    }
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    Ok(output)
}
