use ::std::collections::HashMap;
use ::std::sync::{Arc, Mutex};

use self::prototypes::object::object_proto;
use self::value::Value;

pub mod eval;
pub mod lib;
pub mod prototypes;
pub mod value;

pub use lib::StdLib;
pub use prototypes::Prototypes;

#[derive(Debug, Clone)]
pub struct ScopeStack(Vec<Arc<Mutex<Scope>>>);

pub type Scope = HashMap<String, (Value, DeclType)>;

#[derive(Debug, Clone)]
pub enum DeclType {
    Mutable,
    Immutable,
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

    fn declare(&mut self, name: &String, value: Value, decl_type: DeclType) -> Result<(), String> {
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

        current_scope.insert(name.to_string(), (value, decl_type));

        Ok(())
    }

    fn assgin(&mut self, name: String, value: Value) -> Result<(), String> {
        for scope in self.0.iter().rev() {
            let mut unlocked_scope = scope.lock().unwrap();
            if let Some(v) = unlocked_scope.get(&name) {
                if let DeclType::Immutable = v.1 {
                    return Err(format!("cannot mutate a immutable item '{}'", name));
                }
                unlocked_scope.insert(name, (value, DeclType::Mutable));
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
