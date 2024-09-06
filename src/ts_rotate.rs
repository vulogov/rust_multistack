use crate::ts::TS;

impl TS {
    pub fn left(&mut self) -> &mut TS {
        self.ensure();
        if ! self.stacks.is_empty() {
            self.stacks.rotate_left(1);
        }
        self
    }
    pub fn right(&mut self) -> &mut TS  {
        self.ensure();
        if ! self.stacks.is_empty() {
            self.stacks.rotate_right(1);
        }
        self
    }
}
