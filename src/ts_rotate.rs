use crate::ts::TS;

pub fn ts_rotate_stacks_left(ts: &mut TS) -> &mut TS {
    ts.left()
}

pub fn ts_rotate_stacks_right(ts: &mut TS) -> &mut TS {
    ts.right()
}

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
