use crate::ts::TS;

impl TS {
    pub fn ensure(&mut self) -> &mut TS {
        if self.stack.len() == 0 {
            self.add_stack();
        }
        self
    }
    pub fn ensure_stack(&mut self, name: String) -> &mut TS {
        if ! self.stack.contains_key(&name) {
            self.add_named_stack(name);
        }
        self
    }
    pub fn ensure_stack_with_capacity(&mut self, name: String, capacity: usize) -> &mut TS {
        if ! self.stack.contains_key(&name) {
            self.add_named_stack(name.clone());
        }
        if ! self.stack_cap.contains_key(&name) {
            self.stack_cap.insert(name.clone(), capacity);
        }
        self
    }
    pub fn stack_exists(&mut self, name: String) -> bool {
        self.stack.contains_key(&name)
    }
}
