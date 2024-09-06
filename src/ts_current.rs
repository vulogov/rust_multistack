use crate::ts::TS;
use crate::stack::Stack;
use rust_dynamic::value::Value;

impl TS {
    pub fn current_stack_name(&mut self) -> Option<String> {
        self.ensure().stacks.back().cloned()
    }
    pub fn current(&mut self) -> Option<&mut Stack<Value>> {
        match self.current_stack_name() {
            Some(curr) => {
                return self.stack.get_mut(&curr);
            }
            None => {
                return None
            }
        }
    }
    pub fn stack(&mut self, name: String) -> Option<&mut Stack<Value>> {
        if self.stack.contains_key(&name) {
            match self.stack.get_mut(&name) {
                Some(curr) => {
                    return Some(curr);
                }
                None => {
                    return None;
                }
            }
        }
        None
    }
}
