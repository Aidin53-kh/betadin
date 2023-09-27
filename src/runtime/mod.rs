use ::std::collections::HashMap;
use ::std::fmt;

use ::std::sync::{Arc, Mutex};

use crate::ast::{Arg, Statement};

use self::eval::statement::{eval_statements, Escape};
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
    fn simple(value: &Value) -> String;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum Type {
    Alias(String),
    Builtin(BuiltinType),
}

impl Simple for Type {
    fn simple(value: &Value) -> String {
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
            Value::Func(..) => "function".to_string(),
            Value::Module(_) => "module".to_string(),
            Value::Tuple(_) => "tuple".to_string(),
            Value::Type(_, _) => "type".to_string(),
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
            "tuple" => Type::Builtin(BuiltinType::Tuple(vec![])),
            "function" => Type::Builtin(BuiltinType::Fn(
                vec![],
                Box::new(Type::Builtin(BuiltinType::Null)),
            )),
            other => Type::Alias(other.to_string()),
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
            Value::Func(args, ret_type, block) => {
                let mut scopes = ScopeStack::new(vec![Arc::new(Mutex::new(StdLib::exports()))]);
                let mut args_types = Vec::new();

                for arg in args {
                    args_types.push(arg.datatype.clone());
                    scopes
                        .declare(
                            &arg.ident,
                            Value::from(arg.datatype.clone()),
                            &Some(arg.datatype.clone()),
                            DeclType::Mutable,
                        )
                        .unwrap();
                }

                if let Some(ret_type) = ret_type {
                    return Type::Builtin(BuiltinType::Fn(args_types, Box::new(ret_type.clone())));
                } else {
                    let ret = eval_statements(&mut scopes, block, &Prototypes::exports()).unwrap();

                    match ret {
                        Escape::Return(value) => {
                            return Type::Builtin(BuiltinType::Fn(
                                args_types,
                                Box::new(Type::from(&value)),
                            ))
                        }

                        _ => {
                            return Type::Builtin(BuiltinType::Fn(
                                args_types,
                                Box::new(Type::Builtin(BuiltinType::Null)),
                            ))
                        }
                    }
                }
            }
            Value::Type(_, t) => t.clone(),
            Value::Object(_) => Type::Alias("object".to_string()),
            Value::BuiltInFn(_) => Type::Alias("function".to_string()),
            Value::BuiltInMethod(_, _) => Type::Alias("function".to_string()),
            Value::Module(_) => Type::Alias("module".to_string()),
        }
    }
}

impl From<Type> for String {
    fn from(value: Type) -> Self {
        match value {
            Type::Alias(t) => t,
            Type::Builtin(t) => t.to_string(),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Builtin(t) => write!(f, "{}", t.to_string()),
            Type::Alias(t) => write!(f, "{}", t),
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
                    "property `{}` is reserved in object prototype",
                    kv.key
                ));
            }
        }

        if current_scope.contains_key(name) {
            return Err(format!("`{}` already define in this scope", name));
        }

        // all list items most be have same type
        if let Value::List(list) = &value {
            check_list_items(list)?;
        }

        if let Value::Func(arg, ret_type, block) = &value {
            let mut x = self.clone();
            let ret = eval_statements(&mut x, block, &Prototypes::exports())?;
            if let Some(ret_t) = ret_type {
                if let Escape::Return(val) = ret {
                    if &Type::from(&val) != ret_t {
                        return Err(format!(
                            "extected `{}` found `{}` (15)",
                            Type::from(&Value::Func(
                                arg.to_vec(),
                                Some(ret_t.clone()),
                                block.clone()
                            )),
                            Type::from(&Value::Func(
                                arg.to_vec(),
                                Some(Type::from(&val)),
                                block.clone()
                            )),
                        ));
                    }
                }
            }
        }

        // type checking
        if let Some(datatype) = datatype {
            if let Type::Alias(type_name) = datatype {
                std::mem::drop(current_scope);
                match self.get(type_name) {
                    Some(val) => match &val {
                        Value::Type(_, _) => {
                            if Type::from(&value) != self.get_type_alias(datatype)? {
                                return Err(format!(
                                    "expected `{}: ({})`, found `{}` (2)",
                                    type_name,
                                    self.get_type_alias(datatype)?,
                                    Type::from(&value),
                                ));
                            }
                            let mut current_scope = self
                                .0
                                .last()
                                .expect("`ScopeStack` stack shouldn't be empty")
                                .lock()
                                .unwrap();

                            current_scope
                                .insert(name.to_string(), (value, decl_type, datatype.clone()));
                        }
                        _ => return Err(format!("expected `type`, but `{}` is a `value`", name)),
                    },
                    None => return Err(format!("type `{}` is not defined (9)", type_name)),
                }
            } else {
                if Type::from(&value) != self.get_type_alias(datatype)? {
                    return Err(format!(
                        "expected `{} ({})`, found `{}` (1)",
                        datatype,
                        self.get_type_alias(datatype)?,
                        Type::from(&value)
                    ));
                }

                current_scope.insert(name.to_string(), (value, decl_type, datatype.clone()));
            }
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

    fn declare_fn_statement(
        &mut self,
        fn_name: &String,
        args: &Vec<Arg>,
        ret_type: &Type,
        block: &Vec<Statement>,
    ) -> Result<(), String> {
        let expected_ret_type = self.get_type_alias(ret_type)?;

        let current_scope = self
            .0
            .last()
            .expect("`ScopeStack` stack shouldn't be empty")
            .lock()
            .unwrap();

        if current_scope.contains_key(fn_name) {
            return Err(format!("'{}' already define in this scope", fn_name));
        }

        let mut inner_scope = self.new_from_push(HashMap::new());

        for arg in args {
            inner_scope.declare_variable(
                &arg.ident,
                &arg.datatype,
                &Value::from(arg.datatype.clone()),
                DeclType::Mutable,
            )?;
        }

        let ret = eval_statements(&mut inner_scope, block, &Prototypes::exports())?;

        if let Escape::Return(ret_value) = ret {
            if expected_ret_type != Type::from(&ret_value) {
                return Err(format!(
                    "expected `{}` found `{}` (13)",
                    expected_ret_type,
                    Type::from(&ret_value)
                ));
            }
        } else {
            if expected_ret_type != Type::Builtin(BuiltinType::Null) {
                return Err(format!(
                    "expected `{}` found `{}` (14)",
                    expected_ret_type,
                    Type::Builtin(BuiltinType::Null)
                ));
            }
        }

        Ok(())
    }

    fn declare_variable(
        &mut self,
        name: &String,
        datatype: &Type,
        value: &Value,
        decl_type: DeclType,
    ) -> Result<(), String> {
        let extected_type = self.get_type_alias(datatype)?;
        let value_type = self.get_type_alias(&Type::from(value))?;

        if let Value::Func(args, ret_type, block) = &value {
            if &extected_type != &value_type {
                return Err(format!(
                    "expected `{}` found `{}` (16)",
                    extected_type, value_type
                ));
            }

            if let Some(ret_type) = ret_type {
                let value_type = Type::from(&Value::Func(
                    args.clone(),
                    Some(self.get_type_alias(ret_type)?),
                    block.clone(),
                ));

                let current_scope = self
                    .0
                    .last()
                    .expect("`ScopeStack` stack shouldn't be empty")
                    .lock()
                    .unwrap();

                if current_scope.contains_key(name) {
                    return Err(format!("'{}' already define in this scope", name));
                }

                let mut inner_scope = self.new_from_push(HashMap::new());

                for arg in args {
                    inner_scope.declare_variable(
                        &arg.ident,
                        &arg.datatype,
                        &Value::from(arg.datatype.clone()),
                        DeclType::Mutable,
                    )?;
                }

                let ret = eval_statements(&mut inner_scope, block, &Prototypes::exports())?;

                if let Escape::Return(ret_value) = ret {
                    let ret_type = Type::from(&Value::Func(
                        args.clone(),
                        Some(Type::from(&ret_value)),
                        block.clone(),
                    ));
                    if ret_type != value_type {
                        return Err(format!(
                            "expected `{}` found `{}` (13)",
                            value_type, ret_type
                        ));
                    }
                } else {
                    let ret_type = Type::from(&Value::Func(
                        args.clone(),
                        Some(Type::Builtin(BuiltinType::Null)),
                        block.clone(),
                    ));
                    if value_type != ret_type {
                        return Err(format!(
                            "expected `{}` found `{}` (14)",
                            value_type, ret_type
                        ));
                    }
                }
            }
        } else {
            let mut current_scope = self
                .0
                .last()
                .expect("`ScopeStack` stack shouldn't be empty")
                .lock()
                .unwrap();

            if current_scope.contains_key(name) {
                return Err(format!("`{}` already define in this scope", name));
            }

            if &extected_type != &value_type {
                return Err(format!(
                    "expected `{}` found `{}`",
                    extected_type, value_type
                ));
            }

            current_scope.insert(name.to_string(), (value.clone(), decl_type, value_type));
        }

        Ok(())
    }

    fn declare_type_alias(&mut self, type_name: &String, datatype: &Type) -> Result<(), String> {
        let mut current_scope = self
            .0
            .last()
            .expect("`ScopeStack` stack shouldn't be empty")
            .lock()
            .unwrap();

        if current_scope.contains_key(type_name) {
            return Err(format!("'{}' already define in this scope", type_name));
        }

        current_scope.insert(
            type_name.to_string(),
            (
                Value::Type(type_name.to_string(), datatype.clone()),
                DeclType::Immutable,
                datatype.clone(),
            ),
        );

        Ok(())
    }

    fn get_type_alias(&self, datatype: &Type) -> Result<Type, String> {
        match datatype {
            Type::Builtin(bt) => match bt {
                BuiltinType::Fn(a, rt) => match *rt.clone() {
                    Type::Alias(t) => {
                        return Ok(Type::Builtin(BuiltinType::Fn(
                            a.clone(),
                            Box::new(self.get_type_alias(&Type::Alias(t))?),
                        )))
                    }
                    Type::Builtin(b) => {
                        return Ok(Type::Builtin(BuiltinType::Fn(
                            a.clone(),
                            Box::new(self.get_type_alias(&Type::Builtin(b))?),
                        )))
                    }
                },
                BuiltinType::Tuple(items) => {
                    let mut b_types = Vec::new();

                    for item in items {
                        match item {
                            Type::Alias(type_name) => {
                                b_types.push(
                                    self.get_type_alias(&Type::Alias(type_name.to_string()))?,
                                );
                            }
                            Type::Builtin(b) => {
                                b_types.push(self.get_type_alias(&Type::Builtin(b.clone()))?);
                            }
                        }
                    }

                    Ok(Type::Builtin(BuiltinType::Tuple(b_types)))
                }
                BuiltinType::List(data_type) => {
                    return Ok(Type::Builtin(BuiltinType::List(Box::new(
                        self.get_type_alias(&*data_type.clone())?,
                    ))))
                }
                s => return Ok(Type::Builtin(s.clone())),
            },

            Type::Alias(tn) => match self.get(tn) {
                Some(t) => match &Type::from(&t) {
                    Type::Alias(s) => {
                        return self.get_type_alias(&Type::Alias(s.to_string()));
                    }
                    Type::Builtin(b) => match b {
                        BuiltinType::Fn(a, rt) => match *rt.clone() {
                            Type::Alias(t) => {
                                return Ok(Type::Builtin(BuiltinType::Fn(
                                    a.clone(),
                                    Box::new(self.get_type_alias(&Type::Alias(t))?),
                                )));
                            }
                            Type::Builtin(b) => {
                                return Ok(Type::Builtin(BuiltinType::Fn(
                                    a.clone(),
                                    Box::new(self.get_type_alias(&Type::Builtin(b))?),
                                )));
                            }
                        },
                        BuiltinType::Tuple(items) => {
                            let mut b_types = Vec::new();

                            for item in items {
                                match item {
                                    Type::Alias(type_name) => {
                                        b_types.push(
                                            self.get_type_alias(&Type::Alias(
                                                type_name.to_string(),
                                            ))?,
                                        );
                                    }
                                    Type::Builtin(b) => {
                                        b_types
                                            .push(self.get_type_alias(&Type::Builtin(b.clone()))?);
                                    }
                                }
                            }

                            Ok(Type::Builtin(BuiltinType::Tuple(b_types)))
                        }
                        BuiltinType::List(data_type) => {
                            return Ok(Type::Builtin(BuiltinType::List(Box::new(
                                self.get_type_alias(&*data_type.clone())?,
                            ))))
                        }
                        f => return Ok(Type::Builtin(f.clone())),
                    },
                },
                None => return Err(format!("type `{}` is not defined (10)", tn)),
            },
        }
    }
}
