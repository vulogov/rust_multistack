use crate::ts::TS;
use rust_dynamic::value::Value;

pub fn ts_dup_in_current_stack(ts: &mut TS, n: i64) -> &mut TS {
    ts.dup_in_current_stack(n)
}

pub fn ts_dup_in_stack(ts: &mut TS, n: i64, name: String) -> &mut TS {
    ts.dup_in_stack(n, name)
}

pub fn ts_many_in_current_stack(ts: &mut TS, n: i64, value: Value) -> &mut TS {
    ts.many_in_current_stack(n, value)
}

pub fn ts_many_in_stack(ts: &mut TS, n: i64, name: String, value: Value) -> &mut TS {
    ts.many_in_stack(n, name, value)
}

impl TS {
    pub fn dup_in_current_stack(&mut self, n: i64) -> &mut TS {
        match self.peek() {
            Some(val) => {
                for _ in 0..n {
                    let _ = match val.dup() {
                        Ok(nval) => self.push(nval),
                        Err(_err) => {
                            continue;
                        }
                    };
                }
                return self;
            }
            None => {
                return self;
            }
        }
    }

    pub fn dup_in_stack(&mut self, n: i64, name: String) -> &mut TS {
        match self.peek_from_stack(name.clone()) {
            Some(val) => {
                for _ in 0..n {
                    let _ = match val.dup() {
                        Ok(nval) => self.push_to_stack(name.clone(), nval),
                        Err(_err) => {
                            continue;
                        }
                    };
                }
                return self;
            }
            None => {
                return self;
            }
        }
    }

    pub fn many_in_current_stack(&mut self, n: i64, value: Value) -> &mut TS {
        for _ in 0..n {
            let _ = match value.dup() {
                Ok(nval) => self.push(nval),
                Err(_err) => {
                    continue;
                }
            };
        }
        self
    }

    pub fn many_in_stack(&mut self, n: i64, name: String, value: Value) -> &mut TS {
        for _ in 0..n {
            let _ = match value.dup() {
                Ok(nval) => self.push_to_stack(name.clone(), nval),
                Err(_err) => {
                    continue;
                }
            };
        }
        self
    }

}
