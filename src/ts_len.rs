use crate::ts::TS;

impl TS {
    pub fn len(&self) -> usize {
        self.stacks.len()
    }

    pub fn current_stack_len(&mut self) -> usize {
        match self.current() {
            Some(curr) => {
                return curr.len();
            }
            None => {
                return 0;
            }
        }
    }

    pub fn stack_len(&mut self, name: String) -> usize {
        match self.stack(name) {
            Some(curr) => {
                return curr.len();
            }
            None => {
                return 0;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stacks.is_empty()
    }

}
