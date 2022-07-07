use std::fmt;
use std::sync::mpsc::channel;
use strum_macros::Display;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

#[derive(Display, Debug, Clone)]
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

pub(crate) fn tokenize(code: String) -> Vec<Token> {
    let mut output = vec![];
    let mut current = String::new();
    for character in code.chars() {
        let mut single_character_token_present = true;
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
                single_character_token_present = false;
                current.push(character);
            }
        };
        if character.is_whitespace() || single_character_token_present {
            let mut index = 0;
            if output.len() > 1 {
                if single_character_token_present {
                    index = output.len()-1;
                } else {
                    index = output.len()-2;
                }
            }

            let token = generate_token_from_string(String::from(current.trim()));
            if !matches!(token.token_type, TokenType::NONE) {
                output.insert(index, token);
                current.clear();
            }
        }
    }
    return output;
}

fn generate_token_from_string(str: String) -> Token {
    if !str.is_empty() {
        if str.chars().all(char::is_numeric) {
            let token = Token {
                token_type: TokenType::NumberLiteral,
                value: str.to_string()
            };
            return token;
        } else if str.chars().all(char::is_alphabetic) {
            let token = Token {
                token_type: TokenType::Identifier,
                value: str.to_string()
            };
            return token;
        }
    }
    return Token {
        token_type: TokenType::NONE,
        value: String::new()
    }
}