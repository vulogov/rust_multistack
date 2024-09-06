use crate::stack::Stack;
use std::collections;
use rust_dynamic::value::Value;
use nanoid::nanoid;


#[derive(Clone)]
pub struct TS{
    pub id:             String,
    pub stack:          collections::HashMap<String, Stack<Value>>,
    pub stacks:         collections::VecDeque<String>,
    pub workbench:      Stack<Value>,
}

impl TS {
    fn init() -> Self {
        Self {
            id:         nanoid!(),
            stack:      collections::HashMap::new(),
            stacks:     collections::VecDeque::new(),
            workbench:  Stack::new(),
        }
    }
    pub fn new() -> Self {
        let mut res = TS::init();
        res.ensure();
        res
    }
    pub fn new_with_named(name: String) -> Self {
        let mut res = TS::init();
        res.ensure_stack(name);
        res
    }
}
