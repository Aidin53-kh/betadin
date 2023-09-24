use ::std::collections::HashMap;
use ::std::fmt;
use ::std::sync::{Arc, Mutex};

use self::prototypes::object::object_proto;
use self::value::{check_list_items, BuiltinType, Value};

pub mod eval;
pub mod lib;
pub mod prototypes;
pub mod value;

pub use lib::StdLib;
pub use prototypes::Prototypes;

#[derive(Debug, Clone)]
pub struct ScopeStack(Vec<Arc<Mutex<Scope>>>);

pub type Scope = HashMap<String, (Value, DeclType, Type)>;

#[derive(Debug, Clone)]
pub enum DeclType {
    Mutable,
    Immutable,
}

pub trait Simple {
    fn simple(value: Value) -> String;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum Type {
    Custom(String),
    Builtin(BuiltinType),
}

impl Simple for Type {
    fn simple(value: Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Int(_) => "int".to_string(),
            Value::Float(_) => "float".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::List(_) => "list".to_string(),
            Value::Object(_) => "object".to_string(),
            Value::BuiltInFn(_) => "function".to_string(),
            Value::BuiltInMethod(_, _) => "function".to_string(),
            Value::Func(_, _) => "function".to_string(),
            Value::Module(_) => "module".to_string(),
            Value::Tuple(_) => "tuple".to_string(),
        }
    }
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        match value.as_str() {
            "null" => Type::Builtin(BuiltinType::Null),
            "string" => Type::Builtin(BuiltinType::String),
            "int" => Type::Builtin(BuiltinType::Int),
            "float" => Type::Builtin(BuiltinType::Float),
            "bool" => Type::Builtin(BuiltinType::Bool),
            "list" => Type::Builtin(BuiltinType::List(Box::new(Type::Builtin(
                BuiltinType::Null,
            )))),
            other => Type::Custom(other.to_string()),
        }
    }
}

impl From<&Value> for Type {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => Type::Builtin(BuiltinType::Null),
            Value::Int(_) => Type::Builtin(BuiltinType::Int),
            Value::Float(_) => Type::Builtin(BuiltinType::Float),
            Value::String(_) => Type::Builtin(BuiltinType::String),
            Value::Bool(_) => Type::Builtin(BuiltinType::Bool),
            Value::List(l) => match l.get(0) {
                Some(value) => Type::Builtin(BuiltinType::List(Box::new(Type::from(value)))),
                None => Type::Builtin(BuiltinType::List(Box::new(Type::Builtin(
                    BuiltinType::Null,
                )))),
            },
            Value::Tuple(values) => {
                let mut types = Vec::new();

                for value in values {
                    types.push(Type::from(value));
                }

                Type::Builtin(BuiltinType::Tuple(types))
            }
            Value::Object(_) => Type::Custom("object".to_string()),
            Value::BuiltInFn(_) => Type::Custom("function".to_string()),
            Value::BuiltInMethod(_, _) => Type::Custom("function".to_string()),
            Value::Func(_, _) => Type::Custom("function".to_string()),
            Value::Module(_) => Type::Custom("module".to_string()),
        }
    }
}

impl From<Type> for String {
    fn from(value: Type) -> Self {
        match value {
            Type::Custom(t) => t,
            Type::Builtin(t) => t.to_string(),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Builtin(t) => write!(f, "{}", t.to_string()),
            Type::Custom(t) => write!(f, "{}", t),
        }
    }
}

impl ScopeStack {
    pub fn new(scopes: Vec<Arc<Mutex<Scope>>>) -> ScopeStack {
        ScopeStack(scopes)
    }

    fn new_from_push(&self, scope: Scope) -> ScopeStack {
        let mut scopes = self.0.clone();
        scopes.push(Arc::new(Mutex::new(scope)));

        ScopeStack::new(scopes)
    }

    fn push(&mut self, scope: Scope) {
        self.0.push(Arc::new(Mutex::new(scope)));
    }

    fn declare(
        &mut self,
        name: &String,
        value: Value,
        datatype: &Option<Type>,
        decl_type: DeclType,
    ) -> Result<(), String> {
        let mut current_scope = self
            .0
            .last()
            .expect("`ScopeStack` stack shouldn't be empty")
            .lock()
            .unwrap();

        if let Value::Object(props) = &value {
            let obj_proto = object_proto();
            let mut keys: Vec<String> = vec![];

            for prop in props {
                if keys.contains(&prop.key) {
                    return Err(format!("duplicate property '{}'", prop.key));
                }
                keys.push(prop.key.to_string());
            }

            let res = props
                .into_iter()
                .find(|kv| obj_proto.get(&kv.key).is_some());

            if let Some(kv) = res {
                return Err(format!(
                    "property '{}' is reserved in object prototype",
                    kv.key
                ));
            }
        }

        if current_scope.contains_key(name) {
            return Err(format!("'{}' already define in this scope", name));
        }

        // all list items most be have same type
        if let Value::List(list) = &value {
            check_list_items(list)?;
        }

        // type checking
        if let Some(t) = datatype {
            if Type::from(&value) != *t {
                return Err(format!(
                    "expected `{}`, found `{}` (1)",
                    t,
                    Type::from(&value)
                ));
            }
            current_scope.insert(name.to_string(), (value, decl_type, t.clone()));
        } else {
            current_scope.insert(
                name.to_string(),
                (value.clone(), decl_type, Type::from(&value)),
            );
        }

        Ok(())
    }

    fn assgin(&mut self, name: String, value: Value) -> Result<(), String> {
        for scope in self.0.iter().rev() {
            let mut unlocked_scope = scope.lock().unwrap();
            if let Some((_, decl_type, datatype)) = unlocked_scope.clone().get(&name) {
                if let DeclType::Immutable = decl_type {
                    return Err(format!("cannot mutate a immutable item '{}'", name));
                }

                if &&Type::from(&value) != &datatype {
                    return Err(format!(
                        "expected `{}`, found `{}`",
                        datatype,
                        Type::from(&value)
                    ));
                }

                unlocked_scope.insert(name, (value, DeclType::Mutable, datatype.clone()));
                return Ok(());
            }
        }

        Err(format!("'{}' is not defined", name))
    }

    fn get(&self, name: &String) -> Option<Value> {
        for scope in self.0.iter().rev() {
            let unlocked_scope = scope.lock().unwrap();
            if let Some(v) = unlocked_scope.get(name) {
                return Some(v.0.clone());
            }
        }

        None
    }
}
