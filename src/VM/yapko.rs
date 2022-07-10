use std::collections::HashMap;

#[derive(Clone)]
pub struct YapkoObject {
    pub name: String,
    pub yapko_type: String,
    pub variables: HashMap<String, YapkoObject>,
    pub int_variable: i32,
    pub string_variable: String,
}

pub fn generate_int(name: String, value: i32) -> YapkoObject {
    YapkoObject {
        name,
        yapko_type: "Int".parse().unwrap(),
        variables: Default::default(),
        int_variable: value,
        string_variable: "".to_string()
    }
}

pub fn generate_null(name: String) -> YapkoObject {
    YapkoObject {
        name,
        yapko_type: "Null".parse().unwrap(),
        variables: Default::default(),
        int_variable: 0,
        string_variable: "".to_string()
    }
}
