use crate::ts::TS;

pub fn ts_move_from_current(ts: &mut TS, name_to: String) -> &mut TS {
    ts.move_from_current(name_to)
}

pub fn ts_move_from_stack(ts: &mut TS, name_from: String, name_to: String) -> &mut TS {
    ts.move_from_stack(name_from, name_to)
}

impl TS {
    pub fn move_from_current(&mut self, name_to: String) -> &mut TS {
        loop {
            match self.pull() {
                Some(value) => {
                    self.push_to_stack(name_to.clone(), value);
                }
                None => {
                    break;
                }
            }
        }
        self
    }
    pub fn move_from_stack(&mut self, name_from: String, name_to: String) -> &mut TS {
        loop {
            match self.pull_from_stack(name_from.clone()) {
                Some(value) => {
                    self.push_to_stack(name_to.clone(), value);
                }
                None => {
                    break;
                }
            }
        }
        self
    }
}
