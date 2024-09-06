use crate::stack::Stack;

impl<T> Stack<T> {
    pub fn push(&mut self, value: T) -> &mut Stack<T> {
        if self.policy {
            self.stack.push_back(value);
        } else {
            self.stack.push_front(value);
        }
        self
    }
}
