use crate::ts::TS;
use rust_dynamic::value::Value;

pub fn ts_peek_from_current(ts: &mut TS) -> Option<Value> {
    ts.peek()
}

pub fn ts_peek_from_stack(ts: &mut TS, name: String) -> Option<Value> {
    ts.peek_from_stack(name)
}

impl TS {
    pub fn peek(&mut self) -> Option<Value> {
        match self.current() {
            Some(curr) => {
                match curr.peek() {
                    Some(value) => Some(value.clone()),
                    None => None,
                }
            }
            None => {
                return None;
            }
        }
    }

    pub fn peek_from_stack(&mut self, name: String) -> Option<Value> {
        match self.stack(name) {
            Some(curr) => {
                match curr.peek() {
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
