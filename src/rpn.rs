use crate::{error::CalcError, parser::Token};

/// Алгоритм сортировочной станции (Shunting-yard)
pub fn to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, CalcError> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            _ => {
                todo!()
                }
            }
        }
    }

    Ok(output)
}
