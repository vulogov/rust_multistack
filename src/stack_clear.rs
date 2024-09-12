use crate::stack::Stack;

impl<T> Stack<T> {
    ///
    /// Clears the stack and removes all content
    ///
    pub fn clear(&mut self) -> &mut Stack<T> {
        self.stack.clear();
        self
    }
}
