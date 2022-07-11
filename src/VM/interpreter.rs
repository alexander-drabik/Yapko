use std::collections::HashMap;
use crate::yapko::{generate_int, generate_null, Primitive, Variable, YapkoObject};
use crate::yapko::Primitive::Function;

pub struct VM {
    stack: Vec<YapkoObject>,
    pub(crate) global: HashMap<String, YapkoObject>
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: vec![],
            global: HashMap::new()
        }
    }

    pub fn interpret(&mut self, bytecode: Vec<u8>, commands: HashMap<u8, String>) {
        let mut new_command = true;
        let mut arguments:Vec<u8> = vec![];
        let mut argument = String::new();
        let mut command: u8 = 0;
        for byte in bytecode {
            if new_command {
                if byte == 0 {
                    continue;
                }
                command = byte;
                new_command = false;
            } else {
                if byte == 0 {
                    match commands[&command].as_str() {
                        "=" => {
                            if self.stack.len() < 2 {
                                println!("Missing arguments");
                                return;
                            }
                            let index = self.stack.len();
                            self.stack[index-1].name = String::from(&self.stack[index-2].name);
                            *self.global.get_mut(&self.stack[index - 2].name).unwrap() = self.stack[index - 1].clone();
                            for _ in 0..2 {
                                self.stack.remove(self.stack.len()-1);
                            }
                        }
                        "push_num" => {
                            self.stack.push(generate_int(String::from("$int"), argument.to_string().parse::<i32>().unwrap()));
                        }
                        "get" => {
                            if self.global.contains_key(&*argument) {
                                self.stack.push(self.global[&argument].clone());
                            } else {
                                println!("'{}' not found", argument);
                                return;
                            }
                        }
                        "set_get" => {
                            if self.global.contains_key(&*argument) {
                                return;
                            } else {
                                self.global.insert(argument.to_string(), generate_null(argument.to_string()));
                                self.stack.push(self.global[&argument].clone());
                            }
                        }
                        "call" => {
                            let a = self.stack[&self.stack.len()-1-arguments[0] as usize].clone();
                            match &a.members["value"] {
                                Variable::Primitive(Function(..)) => {
                                    let function = if let Variable::Primitive(Function(function)) = a.members["value"] {
                                        function(&mut self.stack);
                                    } else  {
                                        println!("Cannot invoke '{}'", a.name);
                                        return;
                                    };
                                }
                                _ => {
                                    println!("Cannot invoke '{}'", a.name);
                                    return;
                                }
                            }
                            self.stack.remove(self.stack.len()-1);
                        }
                        _ => {}
                    }

                    new_command = true;
                    argument.clear();
                    arguments.clear();
                } else {
                    arguments.push(byte);
                    argument.push(byte as char);
                }
            }
        }
    }
}
