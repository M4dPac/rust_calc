pub enum CalcError {
    InvalidToken(char),
    UnmatchedParens,
    DivideByZero,
    InvalidExpression,
}
