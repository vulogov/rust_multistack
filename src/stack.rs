use std::collections;
use nanoid::nanoid;

//!
//! Abstraction of the stack structure, implemented on top of std::collections::VecDeque
//!
#[derive(Clone, Debug)]
pub struct Stack<T> {
    id:             String,
    pub policy:     bool,
    pub stack:      collections::VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            id:     nanoid!(),
            policy: true,
            stack:  collections::VecDeque::<T>::new(),
        }
    }
    pub fn init(id: String) -> Self {
        let mut res = Stack::new();
        res.id = id;
        res
    }
    pub fn stack_id(&self) -> String {
        self.id.clone()
    }
}
