use std::collections::HashMap;
use crate::yapko::Primitive::YapkoString;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub fn generate_standard() -> HashMap<String, YapkoObject> {
    fn print_line(stack: &mut Vec<YapkoObject>) {
        if stack.len() > 0 {
            let value = stack[stack.len() - 1].clone();
            if value.members.contains_key("toString") {
                if let Variable::Primitive(Primitive::Function(function)) = &value.members[&String::from("toString")] {
                    function(stack);
                    if let Variable::Primitive(YapkoString(string)) = &stack[stack.len() - 1].members[&String::from("value")] {
                        println!("{}", string);
                    } else {
                        println!("Error converting {} to String", value.name);
                    };
                    stack.remove(stack.len() - 1);
                } else {
                    println!("Error converting {} to String", value.name);
                };
            } else {
                if value.yapko_type == "String" {
                    if let Variable::Primitive(YapkoString(string)) = &value.members[&String::from("value")] {
                        println!("{}", string);
                    }
                } else {
                    println!("Function toString() not found in {}", value.yapko_type);
                }
            }
        } else {
            println!();
        }
    }
    let mut output = HashMap::new();
    output.insert(String::from("printLine"), generate_function(String::from("printLine"), print_line));
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

    YapkoObject {
        name,
        yapko_type: "Int".parse().unwrap(),
        members: hashmap![String::from("value") => Variable::Primitive(Primitive::Int(value)),
            String::from("toString") => Variable::Primitive(Primitive::Function(to_string)),
            String::from("add") => Variable::Primitive(Primitive::Function(add)),
            String::from("sub") => Variable::Primitive(Primitive::Function(sub)),
            String::from("mul") => Variable::Primitive(Primitive::Function(mul)),
            String::from("div") => Variable::Primitive(Primitive::Function(div))
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
