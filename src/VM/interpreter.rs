use std::collections::HashMap;
use std::os::linux::raw::stat;
use crate::yapko::{generate_int, generate_null, YapkoObject};

pub struct VM {
    stack: Vec<YapkoObject>,
    global: HashMap<String, YapkoObject>
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
        let mut arguments:Vec<String> = vec![];
        let mut argument = String::new();
        let mut command: u8 = 0;
        for byte in bytecode {
            if new_command {
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
                            self.stack.push(generate_int(String::from("int"), argument.trim().to_string().parse::<i32>().unwrap()));
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
                        _ => {}
                    }

                    new_command = true;
                    argument.clear();
                    arguments.clear();
                } else {
                    argument.push(byte as char);
                }
            }
        }
    }
}
