use std::collections::HashMap;
use std::env::remove_var;
use std::hash::Hash;
use std::process;
use crate::yapko::{generate_boolean, generate_int, generate_null, generate_string, generate_yapko_function, Primitive, Variable, YapkoObject};
use crate::yapko::Primitive::{Boolean, Function, YapkoFunction};

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
        fn new_scope(scopes: &mut Vec<HashMap<String, YapkoObject>>, scope: &mut usize) {
            *scope += 1;
            scopes.push(HashMap::new());
        }

        fn end_scope(scopes: &mut Vec<HashMap<String, YapkoObject>>, scope: &mut usize, invoke_started_at_scope: usize, used_variables: &mut Vec<(usize, String)>) {
            if scopes.len() > *scope {
                scopes.remove(*scope);
            }
            *scope -= 1;
            if *scope == invoke_started_at_scope {
                used_variables.clear();
            }
        }

        fn operator_to_function_name(a: String) -> &'static str {
            match a.as_str() {
                "+" => "add",
                "-" => "sub",
                "*" => "mul",
                "/" => "div",
                "<" => "smallerThan",
                ">" => "greaterThan",
                "<=" => "smallerOrEqual",
                ">=" => "greaterOrEqual",
                "==" => "equalTo",
                &_ => {""}
            }
        }

        let mut used_variables:Vec<(usize, String)> = Vec::new();
        let mut current_function_argument = String::new();
        let mut invoke_started_at_scope = 0;

        let mut inside_if = false;
        let mut if_opened_at = 0;

        let mut inside_loop = false;
        let mut loop_map:HashMap<usize, Vec<u8>> = HashMap::new();
        let mut loop_started_at = 0;
        let mut loop_current = 0;

        let mut inside_class = false;
        let mut class_name = String::new();

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
                if byte == 0 && !self.inside_function && !inside_if && !inside_loop {
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
                            self.stack.push(generate_boolean(String::from("bool"), if argument == "1" {true} else {false}));
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
                            if a.yapko_type == "Function" || a.yapko_type == "YapkoFunction" {
                                match &a.members["value"] {
                                    Variable::Primitive(Function(..)) => {
                                        if let Variable::Primitive(Function(function)) = a.members["value"] {
                                            function(&mut self.stack);
                                        } else {
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
                                            let mut index = i + 2;
                                            // Insert scope_new (val: 24)
                                            bytecode.insert(i, 24);

                                            // Insert everything inside function
                                            bytecode.insert(i + 1, 0);
                                            for byte2 in function_bytecode {
                                                bytecode.insert(index, byte2);
                                                index += 1;
                                            }

                                            // Insert scope_end (val: 25)
                                            bytecode.insert(index, 25);
                                            bytecode.insert(index+1, 0);

                                            // Set function's scope
                                            used_variables.append(&mut this_used_variables.clone());
                                            invoke_started_at_scope = current_scope;
                                        } else {
                                            println!("Cannot invoke '{}'", a.name);
                                            return;
                                        };
                                    }
                                    _ => {
                                    }
                                }
                            } else {
                                if a.yapko_type == "class" {
                                    // Create variable with default parameters
                                    let mut new_a = a.clone();
                                    new_a.yapko_type = a.name.clone();
                                    self.stack.push(new_a.clone());
                                } else {
                                    println!("Cannot invoke '{}'", a.name);
                                    return;
                                }
                            }
                        }
                        "+"|"-"|"*"|"/"|"<"|">"|"<="|">="|"=="|"!=" => {
                            let function_name= operator_to_function_name(commands[&command].clone());

                            let a = self.stack[&self.stack.len()-2].clone();
                            if a.members.contains_key(&*function_name) {
                                if let Variable::YapkoObject(yapko_function) = a.members[function_name].clone() {
                                    if let Variable::Primitive(Function(function)) = yapko_function.members["value"] {
                                        function(&mut self.stack);
                                    }
                                } else {
                                    println!("Variable {} ({}) does not implement function '{}'", a.name, a.yapko_type, function_name);
                                }
                            }
                        }
                        "and"|"or"|"xor" => {
                            let left = self.stack[&self.stack.len()-2].clone();
                            let right = self.stack[&self.stack.len()-1].clone();
                            if let Variable::Primitive(Boolean(left_value)) = left.members["value"] {
                                if let Variable::Primitive(Boolean(right_value)) = right.members["value"] {
                                    let result = match commands[&command].as_str() {
                                        "and" => {
                                            left_value && right_value
                                        }
                                        "or"  => {
                                            left_value || right_value
                                        }
                                        "xor" => {
                                            left_value ^ right_value
                                        }
                                        _ => {
                                            process::exit(1);
                                        }
                                    };

                                    self.stack.remove(&self.stack.len()-1);
                                    self.stack.remove(&self.stack.len()-1);
                                    self.stack.push(generate_boolean(String::from("$bool"), result));
                                }
                            } else {
                                println!("Error")
                            }
                        }
                        "." => {
                            let left = self.stack[&self.stack.len()-1].clone();
                            self.stack.remove(&self.stack.len()-1);

                            if left.members.contains_key(&*argument) {
                                if let Variable::YapkoObject(variable) = left.members[&argument.clone()].clone() {
                                    self.stack.push(variable);
                                }
                            } else {
                                println!("Variable {} does not implement {}", left.name, argument);
                                process::exit(1);
                            }
                        }
                        "!" => {
                            let left = self.stack[&self.stack.len()-1].clone();
                            if let Variable::Primitive(Boolean(boolean)) = left.members["value"] {
                                self.stack.remove(&self.stack.len()-1);
                                self.stack.push(generate_boolean(String::from("$bool"), !boolean));
                            }
                        }
                        "fun_start" => {
                            self.inside_function = true;
                        }
                        "scope_new" => {
                            new_scope(&mut self.scopes, &mut current_scope);
                        }
                        "scope_end" => {
                            end_scope(&mut self.scopes, &mut current_scope, invoke_started_at_scope.clone(), &mut used_variables);
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
                                );
                                process::exit(1);
                            }
                            self.scopes[current_scope].insert(
                                current_function_argument.clone(),
                                self.stack[&self.stack.len()-1].clone()
                            );
                            self.stack.remove(&self.stack.len()-1);
                        }
                        "if" => {
                            let condition = self.stack[&self.stack.len()-1].clone();
                            if let Variable::Primitive(Boolean(boolean)) = condition.members["value"] {
                                if !boolean {
                                    inside_if = true;
                                    if_opened_at = current_scope;
                                }
                                new_scope(&mut self.scopes, &mut current_scope);
                            }
                        }

                        "condition" => {
                            inside_loop = true;
                            loop_map.remove(&current_scope);
                            loop_map.insert(current_scope, vec![]);
                            loop_started_at = current_scope;
                            loop_current = current_scope;
                        }

                        "while" => {
                            let condition = self.stack[&self.stack.len()-1].clone();
                            if let Variable::Primitive(Boolean(boolean)) = condition.members["value"] {
                                if !boolean {
                                    loop_map.remove(&current_scope);
                                    inside_if = true;
                                    if_opened_at = current_scope;
                                }
                                new_scope(&mut self.scopes, &mut current_scope);
                            }
                        }

                        "class" => {
                            new_scope(&mut self.scopes, &mut current_scope);
                            inside_class = true;
                            class_name = argument.clone();
                        }

                        "close" => {
                            if inside_class {
                                let mut hashmap = HashMap::new();
                                for (name, yapko_object) in &self.scopes[current_scope] {
                                    hashmap.insert(name.to_string(), Variable::YapkoObject(yapko_object.clone()));
                                }
                                self.scopes[current_scope-1].insert(class_name.clone(), YapkoObject {
                                    name: class_name.clone(),
                                    yapko_type: "class".to_string(),
                                    members: hashmap
                                });

                                inside_class = false;
                            }

                            end_scope(&mut self.scopes, &mut current_scope, invoke_started_at_scope.clone(), &mut used_variables);
                            if loop_map.contains_key(&current_scope) {
                                let mut index = i;
                                for byte2 in &loop_map[&current_scope] {
                                    bytecode.insert(index, *byte2);
                                    index += 1;
                                }
                            }
                        }
                        _ => {}
                    }

                    new_command = true;
                    argument.clear();
                    arguments.clear();
                } else if !self.inside_function && !inside_loop {
                    if inside_if {
                        if byte == 0 {
                            new_command = true;
                        }
                        if commands[&command] == "close" {
                            argument.clear();
                            end_scope(&mut self.scopes, &mut current_scope, invoke_started_at_scope.clone(), &mut used_variables);
                            if if_opened_at == current_scope {
                                inside_if = false;
                                continue;
                            }
                        } else if commands[&command] == "if" || commands[&command] == "while" {
                            current_scope+=1;
                        }
                    }
                    arguments.push(byte);
                    argument.push(byte as char);
                } else if self.inside_function {
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
                } else if inside_loop {
                    if byte == 0 {
                        if loop_map.contains_key(&loop_started_at) {
                            let mut current_bytecode = loop_map[&loop_started_at].clone();
                            current_bytecode.push(command);
                            current_bytecode.append(&mut arguments);
                            current_bytecode.push(0);

                            loop_map.remove(&loop_started_at);
                            loop_map.insert(loop_started_at, current_bytecode);
                        }
                        match commands[&command].as_str() {
                            "close" => {
                                loop_current -= 1;
                                if loop_current == loop_started_at {
                                    let mut index = i;
                                    for byte2 in &loop_map[&loop_started_at] {
                                        bytecode.insert(index, *byte2);
                                        index += 1;
                                    }
                                    inside_loop = false;
                                }
                            }
                            "while"|"if" => {
                                loop_current += 1;
                            }
                            _ => {}
                        }

                        new_command = true;
                    } else {
                        arguments.push(byte);
                    }
                }
            }
        }
    }
}
