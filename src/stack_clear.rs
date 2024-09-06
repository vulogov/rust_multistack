use crate::stack::Stack;

impl<T> Stack<T> {
    pub fn clear(&mut self) -> &mut Stack<T> {
        self.stack.clear();
        self
    }
}
