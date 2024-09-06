use crate::ts::TS;
use rust_dynamic::value::Value;

pub fn ts_push_to_current(ts: &mut TS, value: Value) {
    ts.push(value);
}

pub fn ts_push_to_stack(ts: &mut TS, name: String, value: Value) {
    ts.push_to_stack(name, value);
}

impl TS {
    pub fn push(&mut self, value: Value) -> &mut TS {
        match self.current() {
            Some(curr) => {
                curr.push(value);
                return self;
            }
            None => {
                self.add_stack();
                return self.push(value);
            }
        }
    }
    pub fn push_to_stack(&mut self, name: String, value: Value) -> &mut TS {
        self.ensure_stack(name.clone());
        match self.stack(name.clone()) {
            Some(curr) => {
                curr.push(value.clone());
                return self;
            }
            None => {
                self.add_stack();
                return self.push(value);
            }
        }
    }
}