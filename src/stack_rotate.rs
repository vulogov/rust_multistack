use crate::stack::Stack;

impl<T> Stack<T> {
    ///
    /// Rotate elements in the stack to left
    ///
    pub fn left(&mut self) -> &mut Stack<T> {
        if ! self.stack.is_empty() {
            self.stack.rotate_left(1);
        }
        self
    }
    ///
    /// Rotate elements in the stack to the right
    ///
    pub fn right(&mut self) -> &mut Stack<T> {
        if ! self.stack.is_empty() {
            self.stack.rotate_right(1);
        }
        self
    }
}
