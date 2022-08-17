use std::collections::{HashMap};
use crate::lexer::{Token, TokenType};

#[derive(Clone)]
pub struct Node {
    pub(crate) token: Token,
    pub(crate) children: Vec<Node>,
    pub(crate) invoke: bool,
}

impl Node {
    pub(crate) fn print(&self, amount: i32) {
        for _i in 0..amount {
            print!(" ");
        }
        println!("{}", self.token.value);
        for child in &self.children {
            child.print(amount + 1);
        }
    }
}

pub struct Parser {
    operator_values: HashMap<String, i32>,
}

impl Parser {
    pub fn new() -> Parser {
        let mut operator: HashMap<String, i32> = Default::default();
        // Arithmetical operators
        operator.insert(String::from("="), 0);
        operator.insert(String::from("+"), 1);
        operator.insert(String::from("-"), 2);
        operator.insert(String::from("*"), 3);
        operator.insert(String::from("/"), 4);
        operator.insert(String::from(":"), 5);

        // Logical operators
        operator.insert(String::from("and"), 6);
        operator.insert(String::from("or"), 7);
        operator.insert(String::from("xor"), 8);
        operator.insert(String::from("!"), 9);

        // Comparison operators
        operator.insert(String::from("<"), 10);
        operator.insert(String::from("<="), 11);
        operator.insert(String::from(">"), 12);
        operator.insert(String::from(">="), 13);
        operator.insert(String::from("=="), 14);
        operator.insert(String::from("!="), 15);

        operator.insert(String::from("."), 16);

        Parser { operator_values: operator }
    }

    pub fn parse_tokens(&self, tokens: Vec<Token>) -> Node {
        let mut nodes: Vec<Node> = vec![];

        #[derive(Clone)]
        struct Operator {
            value: String,
            index: usize,
        }
        let mut operators = vec![];
        let mut tokens_inside_parens = vec![];

        let mut parens_opened = 0;
        let mut index = 0;
        for token in tokens {
            if matches!(token.token_type, TokenType::ParenOpen) {
                parens_opened += 1;
                if parens_opened == 1 {
                    continue;
                }
            } else if matches!(token.token_type, TokenType::ParenClose) {
                parens_opened -= 1;

                if parens_opened == 0 {
                    if nodes.len() > 0 && matches!(nodes[index-1].token.token_type, TokenType::Identifier) {
                        if tokens_inside_parens.len() > 0 {
                            nodes[index - 1].children.push(self.parse_tokens(tokens_inside_parens.clone()));
                        }
                        nodes[index - 1].invoke = true;
                        tokens_inside_parens.clear();
                    } else {
                        if tokens_inside_parens.len() > 0 {
                            nodes.push(self.parse_tokens(tokens_inside_parens.clone()));
                            index += 1;
                        }
                        tokens_inside_parens.clear();
                    }

                    continue;
                }
            }

            if parens_opened == 0 {
                nodes.push(Node {
                    token: token.clone(),
                    children: vec![],
                    invoke: false,
                });

                match nodes.last().expect("Node loading error").token.token_type {
                    TokenType::Operator => {
                        operators.push(Operator {
                            value: nodes[nodes.len() - 1].clone().token.value,
                            index,
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

        for operator_index in 0..operators.len() {
            let operator = &operators[operator_index];
            let index = operator.index.clone();
            if operator.value == "!" {
                // add right side
                let node = nodes[operator.index + 1].clone();
                nodes[operator.index].children.push(node);

                nodes.remove(operator.index + 1);

                // two items were removed, so every index higher than the current one needs to be lowered by 2
                for operator2 in &mut operators {
                    if operator2.index > index {
                        operator2.index -= 1;
                    }
                }
            } else {
                // add left side
                let node = nodes[operator.index - 1].clone();
                nodes[operator.index].children.push(node);

                // add right side
                let node = nodes[operator.index + 1].clone();
                nodes[operator.index].children.push(node);

                // remove used nodes
                nodes.remove(operator.index + 1);
                nodes.remove(operator.index - 1);

                // two items were removed, so every index higher than the current one needs to be lowered by 2
                for operator2 in &mut operators {
                    if operator2.index > index {
                        operator2.index -= 2;
                    }
                }
            }
        }

        let mut max_index = if nodes.len() >= 1 {
            nodes.len() - 1
        } else {
            0
        };
        for index in 0..nodes.len() {
            if index > max_index {
                break;
            }
            if matches!(nodes[max_index - index].token.token_type, TokenType::Keyword) {
                if (max_index - index) + 1 <= nodes.len() {
                    let node = nodes[(max_index - index) + 1].clone();
                    nodes[max_index - index].children.push(node);
                    nodes.remove((max_index - index) + 1);
                    max_index -= 1;
                }
            }
        }
        nodes[0].clone()
    }
}
