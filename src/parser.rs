use std::collections::{HashMap, LinkedList};
use std::fmt;
use std::ops::Add;
use crate::lexer::{Token, TokenType};
use crate::lexer::TokenType::Operator;

#[derive(Clone)]
pub struct Node {
    token: Token,
    children: Vec<Node>
}

impl Node {
    pub(crate) fn print(&self, amount: i32) {
        for _i in 0..amount {
            print!(" ");
        }
        println!("{}", self.token.value);
        for child in &self.children {
            child.print(amount+1);
        }
    }
}

pub struct Parser {
    operator_values: HashMap<String, i32>
}

impl Parser {
    pub fn new() -> Parser {
        let mut operator: HashMap<String, i32> = Default::default();
        operator.insert(String::from("+"), 0);
        operator.insert(String::from("-"), 1);
        operator.insert(String::from("*"), 2);
        operator.insert(String::from("/"), 3);
        Parser{operator_values: operator}
    }

    pub fn parse_tokens(&self, tokens: Vec<Token>) -> Node {
        let mut nodes = vec![];

        #[derive(Clone)]
        struct Operator {
            value: String,
            index: usize
        }
        let mut operators = vec![];
        let mut tokens_inside_parens = vec![];

        let mut parens_opened = 0;
        let mut index = 0;
        for token in tokens {
            if matches!(token.token_type, TokenType::ParenOpen) {
                parens_opened += 1;
                continue;
            } else if matches!(token.token_type, TokenType::ParenClose) {
                parens_opened -= 1;
                nodes.push(self.parse_tokens(tokens_inside_parens.clone()));
                index += 1;
                tokens_inside_parens.clear();
                continue;
            }

            if parens_opened == 0 {
                nodes.push(Node {
                    token: token.clone(),
                    children: vec![]
                });

                match nodes.last().expect("Node loading error").token.token_type {
                    TokenType::Operator => {
                        operators.push(Operator {
                            value: nodes[nodes.len() - 1].clone().token.value,
                            index
                        });
                    }
                    _ => {}
                }

                index += 1;
            } else {
                tokens_inside_parens.push(token);
            }
        }

        operators.sort_by(|a, b| self.operator_values[&b.value].cmp(&self.operator_values[&a.value]));

        for operator in operators.to_vec() {
            let node = nodes[operator.index+1].clone();
            nodes[operator.index].children.push(node);
            let node = nodes[operator.index-1].clone();
            nodes[operator.index].children.push(node);
            nodes.remove(operator.index+1);
            nodes.remove(operator.index-1);

            for operator2 in &mut operators {
                if operator2.index > operator.index {
                    operator2.index -= 2;
                }
            }
        }

        nodes[0].clone()
    }
}
