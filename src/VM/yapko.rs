use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[derive(Clone)]
pub enum Variable {
    Primitive(Primitive),
    YapkoObject(YapkoObject),

}

#[derive(Clone)]
pub enum Primitive {
    Int(i32),
    String(String),
    Null
}

#[derive(Clone)]
pub struct YapkoObject {
    pub name: String,
    pub yapko_type: String,
    pub variables: HashMap<String, Variable>,
}

pub fn generate_int(name: String, value: i32) -> YapkoObject {
    YapkoObject {
        name,
        yapko_type: "Int".parse().unwrap(),
        variables: hashmap![String::from("value") => Variable::Primitive(Primitive::Int(value))]
    }
}

pub fn generate_null(name: String) -> YapkoObject {
    YapkoObject {
        name,
        yapko_type: "Null".parse().unwrap(),
        variables: hashmap!(String::from("value") => Variable::Primitive(Primitive::Null))
    }
}
