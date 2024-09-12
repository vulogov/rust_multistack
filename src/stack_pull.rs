use crate::stack::Stack;

impl<T> Stack<T> {
    //!
    //! Pull the element from stack
    //!
    pub fn pull(&mut self) -> Option<T> {
        if self.policy {
            self.stack.pop_back()
        } else {
            self.stack.pop_front()
        }
    }
}
