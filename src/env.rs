use std::collections::HashMap;
use crate::Value;

// Define a private type alias for our internal environment representation
type T = HashMap<String, Value>;

// Define a public struct to represent the environment
#[derive(Debug, Clone)]
pub struct Env {
    env: T,
}

impl Env {
    // Public function to create an empty environment
    pub fn new() -> Self {
        Env {
            env: HashMap::new(),
        }
    }

    // Public function to lookup a variable in the environment
    pub fn lookup(&self, var: &str) -> Value {
        self.env.get(var)
            .cloned()
            .unwrap_or_else(|| panic!("Variable '{}' not found in the environment", var))
    }

    // Public function to insert a value into the environment
    pub fn insert(&mut self, var: String, value: Value) {
        self.env.insert(var, value);
    }

    pub fn remove(&mut self, var: &str) {
        self.env.remove(var);
    }
}