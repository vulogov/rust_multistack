use crate::ts::TS;
use crate::stack::Stack;

impl TS {
    pub fn add_stack(&mut self) -> &mut TS {
        let stack = Stack::new();
        self.stacks.push_back(stack.stack_id());
        self.stack.insert(stack.stack_id(), stack);
        self
    }
    pub fn add_named_stack(&mut self, name: String) -> &mut TS {
        if ! self.stack.contains_key(&name) {
            let stack = Stack::init(name);
            self.stacks.push_back(stack.stack_id());
            self.stack.insert(stack.stack_id(), stack);
        }
        self
    }
}
