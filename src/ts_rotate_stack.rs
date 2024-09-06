use crate::ts::TS;

pub fn ts_rotate_current_stack_left(ts: &mut TS) -> &mut TS {
    ts.rotate_current_stack_left()
}

pub fn ts_rotate_current_stack_right(ts: &mut TS) -> &mut TS {
    ts.rotate_current_stack_right()
}

pub fn ts_rotate_stack_left(ts: &mut TS, name: String) -> &mut TS {
    ts.rotate_stack_left(name)
}

pub fn ts_rotate_stack_right(ts: &mut TS, name: String) -> &mut TS {
    ts.rotate_stack_right(name)
}

impl TS {
    pub fn rotate_current_stack_left(&mut self) -> &mut TS {
        match self.current() {
            Some(curr) => {
                curr.left();
            }
            None => {}
        }
        return self;
    }

    pub fn rotate_stack_left(&mut self, name: String) -> &mut TS {
        match self.stack(name) {
            Some(curr) => {
                curr.left();
            }
            None => {}
        }
        return self;
    }

    pub fn rotate_current_stack_right(&mut self) -> &mut TS {
        match self.current() {
            Some(curr) => {
                curr.right();
            }
            None => {}
        }
        return self;
    }

    pub fn rotate_stack_right(&mut self, name: String) -> &mut TS {
        match self.stack(name) {
            Some(curr) => {
                curr.right();
            }
            None => {}
        }
        return self;
    }
}
