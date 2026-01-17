use crate::runtime::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeId {
    Any,
    Null,
    Bool,
    Int,
    Float,
    Number,
    String,
    Array,
    Object,
    Regex,
    HashMap,
    Function,
    #[cfg(feature = "sys")]
    Task,
    #[cfg(feature = "sys")]
    Socket,
    #[cfg(feature = "sys")]
    Server,
    Error,
}

pub fn parse_type(name: &str) -> TypeId {
    match name {
        "any" => TypeId::Any,
        "null" => TypeId::Null,
        "bool" | "boolean" => TypeId::Bool,
        "int" => TypeId::Int,
        "float" => TypeId::Float,
        "number" => TypeId::Number,
        "string" => TypeId::String,
        "array" => TypeId::Array,
        "object" => TypeId::Object,
        "regex" => TypeId::Regex,
        "hashmap" => TypeId::HashMap,
        "function" => TypeId::Function,
        #[cfg(feature = "sys")]
        "task" => TypeId::Task,
        #[cfg(feature = "sys")]
        "socket" => TypeId::Socket,
        #[cfg(feature = "sys")]
        "server" => TypeId::Server,
        "Error" | "error" => TypeId::Error,
        _ => TypeId::Any,
    }
}

pub fn value_type(v: &Value) -> TypeId {
    match v {
        Value::Null => TypeId::Null,
        Value::Bool(_) => TypeId::Bool,
        Value::Int(_) => TypeId::Int,
        Value::Float(_) => TypeId::Float,
        Value::Str(_) => TypeId::String,
        Value::Array(_) => TypeId::Array,
        Value::Object(_) => TypeId::Object,
        Value::Regex(_) => TypeId::Regex,
        Value::HashMap(_) => TypeId::HashMap,
        Value::Function(_) => TypeId::Function,
        #[cfg(feature = "sys")]
        Value::Task(_) => TypeId::Task,
        #[cfg(feature = "sys")]
        Value::Socket(_) => TypeId::Socket,
        #[cfg(feature = "sys")]
        Value::Server(_) => TypeId::Server,
    }
}

pub fn is_assignable(to: &TypeId, from: &TypeId) -> bool {
    if *to == TypeId::Any || *from == TypeId::Any {
        return true;
    }
    if to == from {
        return true;
    }
    // allow int -> float/number, float -> number
    matches!(
        (to, from),
        (TypeId::Float, TypeId::Int)
            | (TypeId::Number, TypeId::Int)
            | (TypeId::Number, TypeId::Float)
    )
}
