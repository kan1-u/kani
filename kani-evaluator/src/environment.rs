use super::builtin::builtins;
use super::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut store = HashMap::new();
        fill_env_with_builtins(&mut store);
        Self::new(store, None)
    }
}

impl From<Rc<RefCell<Environment>>> for Environment {
    fn from(environment: Rc<RefCell<Environment>>) -> Self {
        let mut store = HashMap::new();
        fill_env_with_builtins(&mut store);
        Self::new(store, Some(environment))
    }
}

fn fill_env_with_builtins(hashmap: &mut HashMap<String, Object>) {
    for f in builtins().iter() {
        hashmap.insert(f.name.clone(), Object::Builtin(f.clone()));
    }
}

impl Environment {
    pub fn new(store: HashMap<String, Object>, parent: Option<Rc<RefCell<Environment>>>) -> Self {
        Self { store, parent }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.store.insert(name.to_string(), val);
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(o) => Some(o.clone()),
            None => match self.parent {
                Some(ref env) => env.borrow().get(name),
                None => None,
            },
        }
    }
}
