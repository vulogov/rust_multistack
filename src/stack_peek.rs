use crate::stack::Stack;

impl<T> Stack<T> {
    //!
    //! Returns the reference to the elemet on top of the stack without removing it
    //!
    pub fn peek(&mut self) -> Option<&mut T> {
        if self.policy {
            self.stack.back_mut()
        } else {
            self.stack.front_mut()
        }
    }
}
