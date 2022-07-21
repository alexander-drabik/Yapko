use std::collections::HashMap;
use std::process;
use crate::yapko::{generate_boolean, generate_int, generate_null, generate_string, generate_yapko_function, Primitive, Variable, YapkoObject};
use crate::yapko::Primitive::{Function, YapkoFunction};

pub struct VM {
    stack: Vec<YapkoObject>,
    pub(crate) scopes: Vec<HashMap<String, YapkoObject>>,
    inside_function: bool
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: vec![],
            scopes: vec![HashMap::new()],
            inside_function: false
        }
    }

    pub fn interpret(&mut self, mut bytecode: Vec<u8>, commands: HashMap<u8, String>) {
        let mut new_command = true;
        let mut arguments:Vec<u8> = vec![];
        let mut argument = String::new();
        let mut command: u8 = 0;
        let mut bytecode_of_function = vec![];

        let mut current_scope = 0;

        let mut used_variables:Vec<(usize, String)> = Vec::new();
        let mut current_function_argument = String::new();
        let mut invoke_started_at_scope = 0;

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
                            for i in (0..=current_scope).rev() {
                                if self.scopes[i].contains_key(&self.stack[index - 2].name) {
                                    *self.scopes[i].get_mut(&self.stack[index - 2].name).unwrap() = self.stack[index - 1].clone();
                                    break;
                                }
                            }

                            for _ in 0..2 {
                                self.stack.remove(self.stack.len()-1);
                            }
                        }
                        "push_num" => {
                            self.stack.push(generate_int(String::from("$int"), argument.to_string().parse::<i32>().unwrap()));
                        }
                        "push_str" => {
                            self.stack.push(generate_string(String::from("$string"), argument.to_string()));
                        }
                        "push_bool" => {
                            self.stack.push(generate_boolean(String::from("$string"), if argument == "1" {true} else {false}));
                        }
                        "get" => {
                            if used_variables.len() == 0 {
                                let mut index2 = 0;
                                let mut contains = -1;
                                for scope in &self.scopes {
                                    if scope.contains_key(&*argument) {
                                        contains = index2;
                                    }
                                    index2 += 1;
                                }
                                if contains >= 0 {
                                    self.stack.push(self.scopes[contains as usize][&argument].clone());
                                } else {
                                    println!("'{}' not found", argument);
                                    return;
                                }
                            } else {
                                if self.scopes[current_scope].contains_key(&argument) {
                                    self.stack.push(self.scopes[current_scope][&argument].clone());
                                } else {
                                    let mut contains = false;
                                    let mut index = 0;
                                    for (i, j) in &used_variables {
                                        if *j == argument {
                                            index = i.clone();
                                            contains = true;
                                        }
                                    }
                                    if contains {
                                        self.stack.push(self.scopes[index][&argument].clone());
                                    } else {
                                        println!("'{}' not found", argument);
                                        return;
                                    }
                                }
                            }
                        }
                        "set_get" => {
                            if self.scopes[current_scope].contains_key(&*argument) {
                                println!("{} was already defined", &*argument);
                                return;
                            } else {
                                self.scopes[current_scope].insert(argument.to_string(), generate_null(argument.to_string()));
                                self.stack.push(self.scopes[current_scope][&argument].clone());
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
                                    if let Variable::Primitive(
                                        YapkoFunction(
                                            mut function_bytecode,
                                            this_used_variables
                                        )
                                    ) = a.members["value"].clone() {
                                        let mut index = i+2;
                                        // Insert scope_new (val: 24)
                                        bytecode.insert(i, 24);

                                        // Insert everything inside function
                                        bytecode.insert(i+1, 0);
                                        for byte2 in function_bytecode {
                                            bytecode.insert(index, byte2);
                                            index += 1;
                                        }

                                        // Insert scope_end (val: 25)
                                        bytecode.insert(index, 25);

                                        // Set function's scope
                                        used_variables.append(&mut this_used_variables.clone());
                                        invoke_started_at_scope = current_scope;

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
                            if let Variable::Primitive(Function(function)) = if a.members.contains_key(operator) {
                                a.members[operator].clone()
                            } else {
                                println!("Variable {} ({}) does not implement function '{}'", a.name, a.yapko_type, operator);
                                return;
                            } {
                                function(&mut self.stack)
                            } else {
                                println!("Error at adding");
                            }
                        }
                        "fun_start" => {
                            self.inside_function = true;
                        }
                        "scope_new" => {
                            self.scopes.push(HashMap::new());
                            current_scope += 1;
                        }
                        "scope_end" => {
                            self.scopes.remove(current_scope);
                            current_scope -= 1;
                            if current_scope == invoke_started_at_scope {
                                used_variables.clear();
                            }
                        }
                        "arg" => {
                            current_function_argument = argument.clone();
                        }
                        "arg_type" => {
                            if self.stack[&self.stack.len()-1].yapko_type != argument {
                                println!(
                                    "Expected {}, but got {}",
                                    argument,
                                    self.stack[&self.stack.len()-1].yapko_type,
                                )
                            }
                            self.scopes[current_scope].insert(
                                current_function_argument.clone(),
                                self.stack[&self.stack.len()-1].clone()
                            );
                            self.stack.remove(&self.stack.len()-1);
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
                            // Create function outside scope
                            let mut used_variables:Vec<(usize, String)> = Vec::new();
                            for i in (0..=current_scope).rev() {
                                for map in &self.scopes[i] {
                                    // Check if used_variables already contain variable with the same name
                                    let mut contains = false;
                                    for j in 0..used_variables.len() {
                                        if self.scopes[used_variables[j].0][&used_variables[j].1].name == map.0.clone() {
                                            contains = true
                                        }
                                    }

                                    // If it does not contain the variable add it to scope
                                    if !contains {
                                        used_variables.push((i, map.0.clone()));
                                    }
                                }
                            }

                            self.inside_function = false;
                            self.scopes[current_scope].insert(
                                argument.clone(),
                                generate_yapko_function(
                                    argument.clone(),
                                    bytecode_of_function.clone(),
                                    used_variables,
                                    current_scope
                                )
                            );
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
