struct Token {
    token_type: TokenType,
    value: String,
}

enum TokenType {
    Identifier,
    NumberLiteral,
    StringLiteral,
    ParenOpen,
    ParenClose,
}