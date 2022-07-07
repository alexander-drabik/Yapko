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
    fn print(&self, amount: i32) {
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

    pub fn parse_tokens(&self, tokens: Vec<Token>) {
        let mut nodes = vec![];

        #[derive(Clone)]
        struct Operator {
            value: String,
            index: usize
        }
        let mut operators = vec![];

        let mut index = 0;
        for token in tokens {
            let node = Node {
                token,
                children: vec![]
            };
            nodes.push(node);

            if matches!(nodes[nodes.len()-1].token.token_type, TokenType::Operator) {
                operators.push(Operator {
                    value: nodes[nodes.len()-1].clone().token.value,
                    index
                });
            }
            index += 1;
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
    }
}
