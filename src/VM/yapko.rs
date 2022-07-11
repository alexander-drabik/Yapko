use std::collections::HashMap;

#[derive(Clone)]
pub enum Primitive {
    Int(i32),
    String(String)
}

#[derive(Clone)]
pub struct YapkoObject {
    pub name: String,
    pub yapko_type: String,
    pub variables: HashMap<String, YapkoObject>,
    pub primitive: Primitive
}

pub fn generate_int(name: String, value: i32) -> YapkoObject {
    YapkoObject {
        name,
        yapko_type: "Int".parse().unwrap(),
        variables: Default::default(),
        primitive: Primitive::Int(value)
    }
}

pub fn generate_null(name: String) -> YapkoObject {
    YapkoObject {
        name,
        yapko_type: "Null".parse().unwrap(),
        variables: Default::default(),
        primitive: Primitive::Int(0)
    }
}
