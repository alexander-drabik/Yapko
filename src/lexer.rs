use std::sync::mpsc::channel;

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub enum TokenType {
    Identifier,
    NumberLiteral,
    StringLiteral,
    Operator,
    ParenOpen,
    ParenClose,
    End,
    NONE
}

pub(crate) fn tokenize(code: &str) -> Vec<Token> {
    let mut output = vec![];
    let mut current = String::new();
    for character in code.chars() {
        let mut operator = true;
        match character {
            '+'|'-'|'*'|'/'|'('|')' => {
                let token = Token {
                    token_type: TokenType::Operator,
                    value: character.to_string()
                };
                output.push(token);
            }
            '\n'|';' => {
                let token = Token {
                    token_type: TokenType::End,
                    value: character.to_string()
                };
                output.push(token);
            }
            _ => {
                operator = false;
                current.push(character);
            }
        };
        if character.is_whitespace() || operator {
            let mut index = 0;
            if operator {
                index = output.len()-1;
            } else {
                index = output.len()-2;
            }

            let token = generate_token_from_string(current.to_string());
            if !matches!(token.token_type, TokenType::NONE) {
                output.insert(index, token);
                current.clear();
            }
        }
    }
    return output;
}

fn generate_token_from_string(str: String) -> Token {
    if str.chars().all(char::is_numeric) {
        let token = Token {
            token_type: TokenType::NumberLiteral,
            value: str.to_string()
        };
        return token;
    } else if str.trim().chars().all(char::is_alphabetic) {
        let token = Token {
            token_type: TokenType::Identifier,
            value: str.to_string()
        };
        return token;
    }
    return Token {
        token_type: TokenType::NONE,
        value: String::new()
    }
}