use crate::ts::TS;
use rust_dynamic::value::Value;

pub fn ts_pull_from_current(ts: &mut TS) -> Option<Value> {
    ts.pull()
}

pub fn ts_pull_from_stack(ts: &mut TS, name: String) -> Option<Value> {
    ts.pull_from_stack(name)
}

impl TS {
    pub fn pull(&mut self) -> Option<Value> {
        match self.current() {
            Some(curr) => {
                match curr.pull() {
                    Some(value) => Some(value.clone()),
                    None => None,
                }
            }
            None => {
                return None;
            }
        }
    }

    pub fn pull_from_stack(&mut self, name: String) -> Option<Value> {
        match self.stack(name) {
            Some(curr) => {
                match curr.pull() {
                    Some(value) => Some(value.clone()),
                    None => None,
                }
            }
            None => {
                return None;
            }
        }
    }
}
