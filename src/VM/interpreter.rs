use std::collections::HashMap;
use crate::yapko::{generate_int, generate_null, generate_yapko_function, Primitive, Variable, YapkoObject};
use crate::yapko::Primitive::{Function, YapkoFunction};

pub struct VM {
    stack: Vec<YapkoObject>,
    pub(crate) global: HashMap<String, YapkoObject>,
    inside_function: bool
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: vec![],
            global: HashMap::new(),
            inside_function: false
        }
    }

    pub fn interpret(&mut self, mut bytecode: Vec<u8>, commands: HashMap<u8, String>) {
        let mut new_command = true;
        let mut arguments:Vec<u8> = vec![];
        let mut argument = String::new();
        let mut command: u8 = 0;
        let mut bytecode_of_function = vec![];
        let mut i = 0;
        loop {
            if i >= bytecode.len() {
                break;
            }
            let byte = bytecode[i];
            i += 1;

            if new_command {
                if byte == 0 {
                    continue;
                }
                command = byte;
                if self.inside_function && commands[&command] != String::from("fun_end") {
                    bytecode_of_function.push(byte);
                }
                new_command = false;
            } else {
                if byte == 0 && !self.inside_function {
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
                            let index = if arguments.len() < 1 {
                                self.stack.len()-1
                            } else {
                                self.stack.len()-1-arguments[0] as usize
                            };
                            let a = self.stack[index].clone();
                            self.stack.remove(index);
                            match &a.members["value"] {
                                Variable::Primitive(Function(..)) => {
                                    if let Variable::Primitive(Function(function)) = a.members["value"] {
                                        function(&mut self.stack);
                                    } else  {
                                        println!("Cannot invoke '{}'", a.name);
                                        return;
                                    };
                                }
                                Variable::Primitive(YapkoFunction(..)) => {
                                    if let Variable::Primitive(YapkoFunction(mut function_bytecode)) = a.members["value"].clone() {
                                        bytecode.append(&mut function_bytecode.clone());
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
                        }
                        "+"|"-"|"*"|"/" => {
                            let operator = match commands[&command].as_str() {
                                "+" => {
                                    "add"
                                }
                                "-" => {
                                    "sub"
                                }
                                "*" => {
                                    "mul"
                                }
                                "/" => {
                                    "div"
                                }
                                &_ => {""}
                            };
                            let a = self.stack[&self.stack.len()-2].clone();
                            if let Variable::Primitive(Function(function)) = a.members[operator] {
                                function(&mut self.stack)
                            } else {
                                println!("Error at adding");
                            }
                        }
                        "fun_start" => {
                            self.inside_function = true;
                        }
                        _ => {}
                    }

                    new_command = true;
                    argument.clear();
                    arguments.clear();
                } else if !self.inside_function {
                    arguments.push(byte);
                    argument.push(byte as char);
                } else {
                    if byte == 0 {
                        new_command = true;
                    }
                    if commands[&command] == String::from("fun_end") {
                        if byte == 0 {
                            self.inside_function = false;
                            self.global.insert(argument.clone(), generate_yapko_function(argument.clone(), bytecode_of_function.clone()));
                            bytecode_of_function.clear();
                            argument.clear();
                        } else {
                            argument.push(byte as char);
                        }
                    } else {
                        bytecode_of_function.push(byte);
                    }
                }
            }
        }
    }
}
