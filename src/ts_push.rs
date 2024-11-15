use crate::ts::TS;
use rust_dynamic::value::Value;

pub fn ts_push_to_current(ts: &mut TS, value: Value) {
    ts.push(value);
}

pub fn ts_push_to_stack(ts: &mut TS, name: String, value: Value) {
    ts.push_to_stack(name, value);
}

impl TS {
    pub fn push(&mut self, mut value: Value) -> &mut TS {
        let stack_name = match self.current_stack_name() {
            Some(name) => name,
            None => {
                return self;
            }
        };
        let stack_len = self.current_stack_len();
        let cap = self.stack_capacity(stack_name);

        match self.current() {
            Some(curr) => {
                value.set_tag("stack", &curr.stack_id());
                match cap {
                    Some(cap) => {
                        if stack_len >= cap {
                            let _ = curr.pull();
                        }
                        curr.push(value);
                    }
                    None => {
                        curr.push(value);
                    }
                }
                return self;
            }
            None => {
                self.add_stack();
                return self.push(value);
            }
        }
    }
    pub fn push_to_stack(&mut self, name: String, mut value: Value) -> &mut TS {
        self.ensure_stack(name.clone());
        let stack_len = self.current_stack_len();
        let cap = self.stack_capacity(name.clone());
        match self.stack(name.clone()) {
            Some(curr) => {
                value.set_tag("stack", &curr.stack_id());
                match cap {
                    Some(cap) => {
                        if stack_len >= cap {
                            let _ = curr.pull();
                        }
                        curr.push(value);
                    }
                    None => {
                        curr.push(value);
                    }
                }
                return self;
            }
            None => {
                self.add_stack();
                return self.push(value);
            }
        }
    }
}
