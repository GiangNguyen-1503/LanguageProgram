use std::collections::HashMap;
use crate::typing::Type;

// Define a private type alias for our internal environment representation
type T = HashMap<String, Type>;

// Define a public struct to represent the environment
#[derive(Debug, Clone)]
pub struct TypeEnv {
    env: T,
}

impl TypeEnv {
    // Public function to create an empty environment
    pub fn new() -> Self {
        TypeEnv {
            env: HashMap::new(),
        }
    }

    // Public function to lookup a variable in the environment
    pub fn lookup(&self, var: &str) -> Type {
        self.env.get(var)
            .cloned()
            .unwrap_or_else(|| panic!("Variable '{}' not found in the environment", var))
    }

    // Public function to insert a value into the environment
    pub fn insert(&mut self, var: String, var_type: Type) {
        self.env.insert(var, var_type);
    }
}