use crate::ts::TS;
use rust_dynamic::types::NODATA;

pub fn ts_move_from_current(ts: &mut TS, name_to: String) -> &mut TS {
    ts.move_from_current(name_to)
}

pub fn ts_move_from_stack(ts: &mut TS, name_from: String, name_to: String) -> &mut TS {
    ts.move_from_stack(name_from, name_to)
}

pub fn ts_move_to_current_stack(ts: &mut TS, name_from: String) -> &mut TS {
    ts.move_to_current(name_from)
}

impl TS {
    pub fn move_from_current(&mut self, name_to: String) -> &mut TS {
        loop {
            match self.pull() {
                Some(value) => {
                    if value.type_of() == NODATA {
                        break;
                    }
                    self.push_to_stack(name_to.clone(), value);
                }
                None => {
                    break;
                }
            }
        }
        self
    }
    pub fn move_to_current(&mut self, name_from: String) -> &mut TS {
        let stack_name = match ts.current_stack_name() {
            Some(stack_name) => stack_name,
            None => {
                return ts;
            }
        };
        loop {
            match self.pull_from_stack(name_from.clone()) {
                Some(value) => {
                    if value.type_of() == NODATA {
                        break;
                    }
                    self.push_to_stack(stack_name.clone(), value);
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
                    if value.type_of() == NODATA {
                        break;
                    }
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
