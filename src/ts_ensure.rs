use crate::ts::TS;

impl TS {
    pub fn ensure(&mut self) -> &mut TS {
        if self.stack.len() == 0 {
            self.add_stack();
        }
        self
    }
}
