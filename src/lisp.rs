use std::borrow::Cow;

enum Value {
    Symbol(String),
    Function(String, Vec<String>),
}
