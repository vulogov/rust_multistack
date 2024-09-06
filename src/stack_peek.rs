use crate::stack::Stack;

impl<T> Stack<T> {
    pub fn peek(&mut self) -> Option<&mut T> {
        if self.policy {
            self.stack.back_mut()
        } else {
            self.stack.front_mut()
        }
    }
}
