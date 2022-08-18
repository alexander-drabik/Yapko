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
    BooleanLiteral,
    Operator,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
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
        list.insert(String::from("function"));
        list.insert(String::from("return"));
        list.insert(String::from("if"));
        list.insert(String::from("while"));
        list.insert(String::from("class"));
        list.insert(String::from("execute"));

        Keywords {
            list
        }
    }
}

pub(crate) fn tokenize(code: String) -> Vec<Token> {
    let mut output = vec![];
    let mut current = String::new();
    let mut string = String::new();

    let mut string_literal_start = false;
    for character in code.chars() {
        let mut single_character_token_present = true;
        if string_literal_start {
            if character == '"' {
                output.push(Token {
                    token_type: TokenType::StringLiteral,
                    value: string.clone()
                });
                string_literal_start = false;
                current.clear();
            } else {
                string.push(character);
            }
            continue
        }
        match character {
            '+'|'-'|'*'|'/'|'='|':'|'!'|'>'|'<' => {
                let token = Token {
                    token_type: TokenType::Operator,
                    value: character.to_string()
                };
                output.push(token);
            }
            '.' => {
                if !current.chars().all(|c| char::is_numeric(c)) || current.is_empty() {
                    let token = Token {
                        token_type: TokenType::Operator,
                        value: character.to_string()
                    };
                    output.push(token);
                } else {
                    current += ".";
                    single_character_token_present = false;
                }
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
            '{' => {
                output.push(Token {
                    token_type: TokenType::End,
                    value: "\n".parse().unwrap()
                });
                output.push(Token {
                    token_type: TokenType::BracketOpen,
                    value: character.to_string()
                });
            }
            '}' => {
                output.push(Token {
                    token_type: TokenType::BracketClose,
                    value: character.to_string()
                });
            }
            '"' => {
                if !string_literal_start {
                    string.clear();
                    current.clear();
                    string_literal_start = true;
                }
            }
            _ => {
                single_character_token_present = false;
                current.push(character);
            }
        };
        if character.is_whitespace() || single_character_token_present {
            let mut index = output.len();
            if output.len() > 1 {
                if single_character_token_present {
                    index = output.len() - 1;
                }
            }

            let token = generate_token_from_string(String::from(current.trim()));
            if !matches!(token.token_type, TokenType::NONE) {
                output.insert(index, token);
            }
            current.clear();
        }
    }
    return output;
}

fn generate_token_from_string(str: String) -> Token {
    if !str.is_empty() {
        if str.chars().all(|c| char::is_numeric(c) || c == '.') {
            let token = Token {
                token_type: TokenType::NumberLiteral,
                value: str.to_string()
            };
            return token;
        } else if str.chars().nth(0).unwrap().is_alphabetic() {
            return if Keywords::new().list.contains(&*str) {
                Token {
                    token_type: TokenType::Keyword,
                    value: str.to_string()
                }
            } else {
                if str == "true" || str == "false" {
                    return Token {
                        token_type: TokenType::BooleanLiteral,
                        value: str.to_string()
                    }
                } else {
                    match &str as &str {
                        "and"|"or"|"xor" => {
                            return Token {
                                token_type: TokenType::Operator,
                                value: str.to_string()
                            }
                        }
                        _ => {}
                    }
                }
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