use std::collections::HashSet;
use strum_macros::Display;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

#[derive(Clone, Display)]
pub enum TokenType {
    Identifier,
    NumberLiteral,
    StringLiteral,
    Operator,
    ParenOpen,
    ParenClose,
    End,
    Keyword,
    NONE
}

pub struct Keywords {
    pub list: HashSet<String>
}

impl Keywords {
    pub fn new() -> Keywords {
        let mut list: HashSet<String> = HashSet::new();
        list.insert(String::from("let"));
        Keywords {
            list
        }
    }
}

pub(crate) fn tokenize(code: String) -> Vec<Token> {
    let mut output = vec![];
    let mut current = String::new();
    let mut string = String::new();
    for character in code.chars() {
        let mut single_character_token_present = true;
        string.push(character);
        match character {
            '+'|'-'|'*'|'/'|'=' => {
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
            '(' => {
                let token = Token {
                    token_type: TokenType::ParenOpen,
                    value: character.to_string()
                };
                output.push(token);
            }
            ')' => {
                let token = Token {
                    token_type: TokenType::ParenClose,
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
            let mut index = output.len();
            if output.len() > 1 {
                index = output.len();
                if single_character_token_present {
                    index = output.len() - 1;
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
            return if Keywords::new().list.contains(&*str) {
                Token {
                    token_type: TokenType::Keyword,
                    value: str.to_string()
                }
            } else {
                Token {
                    token_type: TokenType::Identifier,
                    value: str.to_string()
                }
            }
        }
    }
    return Token {
        token_type: TokenType::NONE,
        value: String::new()
    }
}