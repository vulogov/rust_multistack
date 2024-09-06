use crate::stack::Stack;

impl<T> Stack<T> {
    pub fn left(&mut self) -> &mut Stack<T> {
        if ! self.stack.is_empty() {
            self.stack.rotate_left(1);
        }
        self
    }
    pub fn right(&mut self) -> &mut Stack<T> {
        if ! self.stack.is_empty() {
            self.stack.rotate_right(1);
        }
        self
    }
}
