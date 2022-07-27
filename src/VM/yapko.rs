use std::collections::HashMap;
use std::process;
use crate::yapko::Primitive::{Function, YapkoString};

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
fn execute_function(stack: &mut Vec<YapkoObject>, function_object: &YapkoObject) {
    let value = &function_object.members[&String::from("value")];
    if let Variable::Primitive(Primitive::Function(function)) = value {
        function(stack);
    }
}

pub fn generate_standard() -> HashMap<String, YapkoObject> {
    fn print_line(stack: &mut Vec<YapkoObject>) {
        // If there are no arguments - just write new line
        if stack.len() == 0 {
            println!();
            return;
        }

        let value = stack[stack.len() - 1].clone();
        if !value.members.contains_key("toString") {
            // We do not need to convert string to string
            if value.yapko_type == "String" {
                let value = &value.members[&String::from("value")];
                if let Variable::Primitive(YapkoString(string)) = value {
                    println!("{}", string);
                }
                return
            } else {
                println!("Function toString() not found in {}", value.yapko_type);
                process::exit(1);
            }
        }

        let to_string_object = &value.members[&String::from("toString")];
        if let Variable::YapkoObject(yapko_function) = to_string_object {
            // toString function will push converted string to the stack
            execute_function(stack, yapko_function);
            let string_object = &stack[stack.len() - 1];
            let string_value = &string_object.members[&String::from("value")];
            if let Variable::Primitive(YapkoString(text)) = string_value {
                println!("{}", text);
            } else {
                println!("Error converting {} to String", value.name);
            };
            // Remove string from stack
            stack.remove(stack.len() - 1);
        } else {
            println!("Error converting {} to String", value.name);
        };

        // Remove value from stack
        stack.remove(stack.len() - 1);
    }
    let mut output = HashMap::new();
    output.insert(String::from("printLine"), generate_function(String::from("printLine"), print_line));

    fn print(stack: &mut Vec<YapkoObject>) {
        let value = stack[stack.len() - 1].clone();
        if !value.members.contains_key("toString") {
            // We do not need to convert string to string
            if value.yapko_type == "String" {
                let value = &value.members[&String::from("value")];
                if let Variable::Primitive(YapkoString(string)) = value {
                    println!("{}", string);
                }
                return
            } else {
                println!("Function toString() not found in {}", value.yapko_type);
                process::exit(1);
            }
        }

        let to_string_object = &value.members[&String::from("toString")];
        if let Variable::YapkoObject(yapko_function) = to_string_object {
            // toString function will push converted string to the stack
            execute_function(stack, yapko_function);
            let string_object = &stack[stack.len() - 1];
            let string_value = &string_object.members[&String::from("value")];
            if let Variable::Primitive(YapkoString(text)) = string_value {
                println!("{}", text);
            } else {
                println!("Error converting {} to String", value.name);
            };
            // Remove string from stack
            stack.remove(stack.len() - 1);
        } else {
            println!("Error converting {} to String", value.name);
        };

        // Remove value from stack
        stack.remove(stack.len() - 1);
    }
    output.insert(String::from("print"), generate_function(String::from("print"), print));

    let mut int_class = generate_int(String::from("Int"), 0);
    int_class.yapko_type = String::from("class");
    output.insert(String::from("Int"), int_class);

    let mut string_class = generate_string(String::from("Int"), String::new());
    string_class.yapko_type = String::from("class");
    output.insert(String::from("String"), string_class);

    let mut boolean_class = generate_boolean(String::from("Boolean"), false);
    boolean_class.yapko_type = String::from("class");
    output.insert(String::from("Boolean"), boolean_class);

    return output;
}

#[derive(Clone)]
pub enum Variable {
    Primitive(Primitive),
    YapkoObject(YapkoObject),
}

#[derive(Clone)]
pub enum Primitive {
    Int(i32),
    YapkoString(String),
    YapkoFunction(Vec<u8>, Vec<(usize, String)>),
    Function(fn (stack: &mut Vec<YapkoObject>)),
    Boolean(bool),
    Null
}

#[derive(Clone)]
pub struct YapkoObject {
    pub name: String,
    pub yapko_type: String,
    pub members: HashMap<String, Variable>,
}

pub fn generate_int(name: String, value: i32) -> YapkoObject {
    fn to_string(stack: &mut Vec<YapkoObject>) {
        let int = stack[stack.len()-1].clone();
        stack.remove(stack.len()-1);
        if let Variable::Primitive(Primitive::Int(value)) = int.members["value"] {
            stack.push(generate_string(int.name, value.to_string()));
        } else {
            println!("Error converting {} to String", int.name);
            return;
        };
    }

    fn add(stack: &mut Vec<YapkoObject>) {
        let left  = stack[stack.len()-2].clone();
        let right = stack[stack.len()-1].clone();
        stack.remove(stack.len()-1);
        stack.remove(stack.len()-1);

        if let Variable::Primitive(Primitive::Int(left_value)) = left.members["value"] {
            if right.yapko_type != "Int" {
                println!("Int does not implement add({})", right.yapko_type);
            }
            if let Variable::Primitive(Primitive::Int(right_value)) = right.members["value"] {
                stack.push(generate_int(left.name, left_value + right_value));
            }
        }
    }
    fn sub(stack: &mut Vec<YapkoObject>) {
        let left  = stack[stack.len()-2].clone();
        let right = stack[stack.len()-1].clone();
        stack.remove(stack.len()-1);
        stack.remove(stack.len()-1);

        if let Variable::Primitive(Primitive::Int(left_value)) = left.members["value"] {
            if right.yapko_type != "Int" {
                println!("Int does not implement add({})", right.yapko_type);
            }
            if let Variable::Primitive(Primitive::Int(right_value)) = right.members["value"] {
                stack.push(generate_int(left.name, left_value - right_value));
            }
        }
    }
    fn div(stack: &mut Vec<YapkoObject>) {
        let left  = stack[stack.len()-2].clone();
        let right = stack[stack.len()-1].clone();
        stack.remove(stack.len()-1);
        stack.remove(stack.len()-1);

        if let Variable::Primitive(Primitive::Int(left_value)) = left.members["value"] {
            if right.yapko_type != "Int" {
                println!("Int does not implement add({})", right.yapko_type);
            }
            if let Variable::Primitive(Primitive::Int(right_value)) = right.members["value"] {
                stack.push(generate_int(left.name, left_value / right_value));
            }
        }
    }
    fn mul(stack: &mut Vec<YapkoObject>) {
        let left  = stack[stack.len()-2].clone();
        let right = stack[stack.len()-1].clone();
        stack.remove(stack.len()-1);
        stack.remove(stack.len()-1);

        if let Variable::Primitive(Primitive::Int(left_value)) = left.members["value"] {
            if right.yapko_type != "Int" {
                println!("Int does not implement add({})", right.yapko_type);
            }
            if let Variable::Primitive(Primitive::Int(right_value)) = right.members["value"] {
                stack.push(generate_int(left.name, left_value * right_value));
            }
        }
    }
    fn smaller_than(stack: &mut Vec<YapkoObject>) {
        let left = stack[stack.len()-2].clone();
        let right= stack[stack.len()-1].clone();
        stack.remove(stack.len()-1);
        stack.remove(stack.len()-1);

        if let Variable::Primitive(Primitive::Int(left_value)) = left.members["value"] {
            if let Variable::Primitive(Primitive::Int(right_value)) = right.members["value"] {
                stack.push(generate_boolean("$bool".parse().unwrap(), left_value < right_value))
            } else {
                println!("Cannot compare Int to not-Int")
            }
        }
    }

    fn greater_than(stack: &mut Vec<YapkoObject>) {
        let left = stack[stack.len()-2].clone();
        let right= stack[stack.len()-1].clone();
        stack.remove(stack.len()-1);
        stack.remove(stack.len()-1);

        if let Variable::Primitive(Primitive::Int(left_value)) = left.members["value"] {
            if let Variable::Primitive(Primitive::Int(right_value)) = right.members["value"] {
                stack.push(generate_boolean("$bool".parse().unwrap(), left_value > right_value))
            } else {
                println!("Cannot compare Int to not-Int")
            }
        }
    }

    YapkoObject {
        name,
        yapko_type: "Int".parse().unwrap(),
        members: hashmap![String::from("value") => Variable::Primitive(Primitive::Int(value)),
            String::from("toString") => Variable::YapkoObject(generate_function(String::from("toString"), to_string)),
            String::from("add") => Variable::YapkoObject(generate_function(String::from("add"), add)),
            String::from("sub") => Variable::YapkoObject(generate_function(String::from("sub"), sub)),
            String::from("mul") => Variable::YapkoObject(generate_function(String::from("mul"), mul)),
            String::from("div") => Variable::YapkoObject(generate_function(String::from("div"), div)),
            String::from("smallerThan") => Variable::YapkoObject(generate_function(String::from("smallerThan"), smaller_than)),
            String::from("greaterThan") => Variable::YapkoObject(generate_function(String::from("greaterThan"), greater_than))
        ]
    }
}

pub fn generate_string(name: String, value: String) -> YapkoObject {
    YapkoObject {
        name,
        yapko_type: "String".parse().unwrap(),
        members: hashmap![String::from("value") => Variable::Primitive(Primitive::YapkoString(value))]
    }
}

pub fn generate_boolean(name: String, value: bool) -> YapkoObject {
    fn to_string(stack: &mut Vec<YapkoObject>) {
        let boolean = stack[stack.len()-1].clone();
        stack.remove(stack.len()-1);
        if let Variable::Primitive(Primitive::Boolean(value)) = boolean.members["value"] {
            stack.push(generate_string(boolean.name, value.to_string()));
        } else {
            println!("Error converting {} to String", boolean.name);
            return;
        };
    }
    YapkoObject {
        name,
        yapko_type: "Boolean".parse().unwrap(),
        members: hashmap![
            String::from("value") => Variable::Primitive(Primitive::Boolean(value)),
            String::from("toString") => Variable::YapkoObject(generate_function(String::from("toString"), to_string))
        ]
    }
}

pub fn generate_yapko_function(name: String, bytecode: Vec<u8>, mut used_variables: Vec<(usize, String)>, scope: usize) -> YapkoObject {
    used_variables.push((scope, name.clone()));
    YapkoObject {
        name,
        yapko_type: String::from("YapkoFunction"),
        members: hashmap!(
            String::from("value") => Variable::Primitive(Primitive::YapkoFunction(bytecode, used_variables))
        )
    }
}

pub fn generate_function(name: String, function: fn(stack: &mut Vec<YapkoObject>)) -> YapkoObject {
    YapkoObject {
        name,
        yapko_type: String::from("Function"),
        members: hashmap![String::from("value") => Variable::Primitive(Primitive::Function(function))]
    }
}

pub fn generate_null(name: String) -> YapkoObject {
    YapkoObject {
        name,
        yapko_type: "Null".parse().unwrap(),
        members: hashmap!(String::from("value") => Variable::Primitive(Primitive::Null))
    }
}
