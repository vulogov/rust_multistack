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

pub fn ts_swap_in_current_stack(ts: &mut TS, n: i64) -> &mut TS {
    ts.swap_in_current_stack(n)
}

pub fn ts_swap_in_stack(ts: &mut TS, n: i64, name: String) -> &mut TS {
    ts.swap_in_stack(n, name)
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

    pub fn swap_in_current_stack(&mut self, n: i64) -> &mut TS {
        if self.current_stack_len() < 2 {
            return self;
        }
        match self.peek() {
            Some(val) => {
                for _ in 0..n {
                    self.rotate_current_stack_right();
                }
                match self.pull() {
                    Some(oval) => {
                        self.push(val);
                        for _ in 0..n {
                            self.rotate_current_stack_left();
                        }
                        self.pull();
                        self.push(oval);
                        return self;
                    }
                    None => {
                        return self;
                    }
                }
            }
            None => {
                return self;
            }
        }
    }

    pub fn swap_in_stack(&mut self, n: i64, name: String) -> &mut TS {
        if self.stack_len(name.clone()) < 2 {
            return self;
        }
        match self.peek_from_stack(name.clone()) {
            Some(val) => {
                for _ in 0..n {
                    self.rotate_stack_right(name.clone());
                }
                match self.pull_from_stack(name.clone()) {
                    Some(oval) => {
                        self.push_to_stack(name.clone(), val);
                        for _ in 0..n {
                            self.rotate_stack_left(name.clone());
                        }
                        self.pull_from_stack(name.clone());
                        self.push_to_stack(name.clone(), oval);
                        return self;
                    }
                    None => {
                        return self;
                    }
                }
            }
            None => {
                return self;
            }
        }
    }

}
