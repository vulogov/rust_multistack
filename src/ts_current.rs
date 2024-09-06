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
}
