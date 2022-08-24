use std::collections::HashMap;
use std::process;
use crate::lexer::{Keywords, TokenType};
use crate::parser::Node;

pub struct ByteCode {
    pub commands: HashMap<String, u8>,
    pub brackets_opened: i32,
    pub functions: HashMap<i32, String>
}

impl ByteCode {
    pub fn new() -> ByteCode {
        let mut commands: HashMap<String, u8> = HashMap::new();
        commands.insert(String::from(";"), 0);
        commands.insert(String::from("get"), 1);
        commands.insert(String::from("set_get"), 2);
        commands.insert(String::from("set_get_num"), 3);
        commands.insert(String::from("push"), 4);
        commands.insert(String::from("push_num"), 5);
        commands.insert(String::from("push_str"), 6);
        commands.insert(String::from("+"), 7);
        commands.insert(String::from("-"), 8);
        commands.insert(String::from("*"), 9);
        commands.insert(String::from("/"), 10);
        commands.insert(String::from("="), 11);
        commands.insert(String::from("call"), 21);
        commands.insert(String::from("fun_start"), 22);
        commands.insert(String::from("fun_end"), 23);
        commands.insert(String::from("scope_new"), 24);
        commands.insert(String::from("scope_end"), 25);
        commands.insert(String::from("arg"), 26);
        commands.insert(String::from("arg_type"), 27);
        commands.insert(String::from("push_bool"), 28);
        commands.insert(String::from("or"), 29);
        commands.insert(String::from("xor"), 30);
        commands.insert(String::from("and"), 31);
        commands.insert(String::from("!"), 32);
        commands.insert(String::from("if"), 33);
        commands.insert(String::from("while"), 34);
        commands.insert(String::from("condition"), 35);
        commands.insert(String::from("close"), 36);
        commands.insert(String::from("<"), 37);
        commands.insert(String::from(">"), 38);
        commands.insert(String::from("<="), 39);
        commands.insert(String::from(">="), 40);
        commands.insert(String::from("=="), 41);
        commands.insert(String::from("!="), 42);
        commands.insert(String::from("%"), 43);
        commands.insert(String::from("."), 44);
        commands.insert(String::from("class"), 45);
        commands.insert(String::from("return"), 46);
        commands.insert(String::from("execute"), 47);

        ByteCode {
            commands,
            brackets_opened: 0,
            functions: HashMap::new()
        }
    }

    pub fn generate_bytecode(&mut self, node: Node) -> Vec<u8> {
        match node.token.token_type {
            TokenType::Identifier => {
                // Function
                return if node.invoke {
                    let mut output = vec![];

                    // Get function
                    output.push(self.commands["get"]);
                    for ch in node.token.value.chars() {
                        output.push(ch as u8);
                    }
                    output.push(0);

                    for child in &node.children {
                        let bytecode = self.generate_bytecode(child.clone());
                        for byte in bytecode {
                            output.push(byte);
                        }
                        output.push(0);
                    }

                    // Call function
                    output.push(self.commands["call"]);
                    output.push(node.children.len() as u8);
                    output.push(0);

                    output
                // Variable
                } else {
                    let mut output = vec![];
                    output.push(self.commands["get"]);
                    for ch in node.token.value.chars() {
                        output.push(ch as u8);
                    }
                    output.push(0);

                    output
                }
            }
            TokenType::NumberLiteral => {
                let mut output = vec![];
                output.push(self.commands["push_num"]);
                for ch in node.token.value.chars() {
                    output.push(ch as u8);
                }
                output.push(0);
                return output
            }
            TokenType::StringLiteral => {
                let mut output = vec![];
                output.push(self.commands["push_str"]);
                for ch in node.token.value.chars() {
                    output.push(ch as u8);
                }
                output.push(0);
                return output;
            }
            TokenType::BooleanLiteral => {
                let mut output = vec![];
                output.push(self.commands["push_bool"]);
                output.push(if node.token.value == "true" {'1' as u8} else {'0' as u8});
                output.push(0);
                return output;
            }
            TokenType::Operator => {
                return if node.token.value == "!" {
                    let mut output = vec![];
                    output.append(
                        &mut self.generate_bytecode(node.children[0].clone())
                    );
                    output.push(self.commands[&node.token.value]);
                    output.push(0);
                    output
                } else if node.token.value == "." {
                    let mut output = vec![];
                    output.append(
                        &mut self.generate_bytecode(node.children[0].clone())
                    );
                    output.push(self.commands[&node.token.value]);
                    for ch in node.children[1].token.value.chars() {
                        output.push(ch as u8);
                    }
                    output.push(0);

                    // Is a function
                    if node.children[1].invoke {
                        for child in &node.children[1].children {
                            let bytecode = self.generate_bytecode(child.clone());
                            for byte in bytecode {
                                output.push(byte);
                            }
                            output.push(0);
                        }

                        // Call function
                        output.push(self.commands["call"]);
                        output.push(node.children[1].children.len() as u8);
                        output.push(0);
                    }
                    output
                } else {
                    let mut output = vec![];
                    output.append(
                        &mut self.generate_bytecode(node.children[0].clone())
                    );
                    output.append(
                        &mut self.generate_bytecode(node.children[1].clone())
                    );
                    output.push(self.commands[&node.token.value]);
                    output.push(0);
                    output
                }
            }
            TokenType::Keyword => {
                if Keywords::new().list.contains(&*node.token.value) {
                    match node.token.value.as_str(){
                        "let" => {
                            let mut output = vec![];
                            output.append(
                                &mut self.generate_bytecode(node.children[0].clone())
                            );
                            output[0] = self.commands["set_get"];
                            return output;
                        },
                        "function" => {
                            let mut output = vec![];
                            output.push(self.commands["fun_start"]);
                            for ch in node.children[0].token.value.chars() {
                                output.push(ch as u8);
                            }
                            self.functions.insert(
                                self.brackets_opened.clone()+1,
                                node.children[0].token.value.clone()
                            );
                            output.push(0);

                            // Arguments
                            for child in &node.children[0].children {
                                if child.token.value == ":" {
                                    // Add argument name to the bytecode
                                    output.push(self.commands["arg"]);
                                    for ch in child.children[0].token.value.chars() {
                                        output.push(ch as u8);
                                    }
                                    output.push(0);

                                    // Add argument type to the bytecode
                                    output.push(self.commands["arg_type"]);
                                    for ch in child.children[1].token.value.chars() {
                                        output.push(ch as u8);
                                    }
                                    output.push(0);
                                } else {
                                    println!(
                                        "Error at argument declaration in '{}'. \
                                         Expected ':' operator", node.children[0].token.value
                                    );
                                    process::exit(1);
                                }
                            }
                            return output;
                        },
                        "if" => {
                            let mut output = vec![];
                            output.append(
                                &mut self.generate_bytecode(node.children[0].clone())
                            );
                            output.push(self.commands["if"]);
                            output.push(0);
                            return output;
                        }
                        "while" => {
                            let mut output = vec![];
                            output.push(self.commands["condition"]);
                            output.push(0);
                            output.append(
                                &mut self.generate_bytecode(node.children[0].clone())
                            );
                            output.push(self.commands["while"]);
                            output.push(0);
                            return output;
                        }
                        "return" => {
                            let mut output = vec![];
                            output.append(
                                &mut self.generate_bytecode(node.children[0].clone())
                            );
                            output.push(0);
                            output.push(self.commands["return"]);
                            output.push(0);
                            return output;
                        }
                        "class" => {
                            let mut output = vec![];
                            output.push(self.commands["class"]);
                            for ch in node.children[0].token.value.chars() {
                                output.push(ch as u8);
                            }
                            output.push(0);
                            return output;
                        }
                        "execute" => {
                            let mut output = vec![];
                            output.push(self.commands["execute"]);
                            for ch in node.children[0].token.value.chars() {
                                output.push(ch as u8);
                            }
                            output.push(0);
                            return output;
                        }
                        _ => {}
                    }
                } else {
                    println!("{} not recognized", node.token.value);
                }
            }
            TokenType::BracketOpen => {
                self.brackets_opened += 1;
            }
            TokenType::BracketClose => {
                let mut output = vec![];
                if self.functions.contains_key(&self.brackets_opened) {
                    output.push(self.commands["fun_end"]);
                    for ch in self.functions[&self.brackets_opened].clone().chars() {
                        output.push(ch as u8);
                    }
                    output.push(0);

                    self.functions.remove(&self.brackets_opened);
                    self.brackets_opened -= 1;
                } else {
                    output.push(self.commands["close"]);
                    output.push(0);
                }
                self.brackets_opened -= 1;
                return output;
            }
            _ => {}
        }
        return vec![0]
    }
}
