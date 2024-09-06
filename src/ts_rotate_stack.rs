use crate::ts::TS;


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
