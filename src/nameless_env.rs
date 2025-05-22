use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct NamelessEnv<T> {
    env: Vec<T>,
}

impl<T> NamelessEnv<T> {
    // Public function to create an empty environment
    pub fn new() -> Self {
        NamelessEnv {
            env: Vec::new(),
        }
    }

    // Public function to insert a value into the environment
    pub fn insert(&mut self, value: T) {
        self.env.insert(0, value);
    }
}

impl<T> NamelessEnv<T>
where 
    T: PartialEq + Display,
{
    // Public function to find the index of a value in the environment
    pub fn index_of(&self, value: &T) -> i32 {
        self.env.iter()
            .position(|x| x == value)
            .map(|i| i as i32)
            .unwrap_or_else(|| panic!("Unbound variable: {}", value))
    }
}
