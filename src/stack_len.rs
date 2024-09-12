use crate::stack::Stack;

impl<T> Stack<T> {
    //!
    //! Return stack length
    //!
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    //!
    //! Returns ```true``` if stack is empty, ```false``` otherwise
    //!
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
