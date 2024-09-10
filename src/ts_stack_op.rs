use crate::ts::TS;
use rust_dynamic::value::Value;
use easy_error::{bail, Error};

pub fn ts_dup_in_current_stack(ts: &mut TS, n: i64) -> Result<&mut TS, Error> {
    ts.dup_in_current_stack(n)
}

pub fn ts_dup_in_stack(ts: &mut TS, n: i64, name: String) -> Result<&mut TS, Error> {
    ts.dup_in_stack(n, name)
}

pub fn ts_many_in_current_stack(ts: &mut TS, n: i64, value: Value) -> Result<&mut TS, Error> {
    ts.many_in_current_stack(n, value)
}

pub fn ts_many_in_stack(ts: &mut TS, n: i64, name: String, value: Value) -> Result<&mut TS, Error> {
    ts.many_in_stack(n, name, value)
}

pub fn ts_swap_in_current_stack(ts: &mut TS, n: i64) -> Result<&mut TS, Error> {
    ts.swap_in_current_stack(n)
}

pub fn ts_swap_in_stack(ts: &mut TS, n: i64, name: String) -> Result<&mut TS, Error> {
    ts.swap_in_stack(n, name)
}

impl TS {
    pub fn dup_in_current_stack(&mut self, n: i64) -> Result<&mut TS, Error> {
        match self.peek() {
            Some(val) => {
                for _ in 0..n {
                    let _ = match val.dup() {
                        Ok(nval) => self.push(nval),
                        Err(err) => {
                            bail!("Error duplicating data in current stack: {}", err);
                        }
                    };
                }
                return Ok(self);
            }
            None => {
                bail!("Error duplicating data in current stack: unable to peek()");
            }
        }
    }

    pub fn dup_in_stack(&mut self, n: i64, name: String) -> Result<&mut TS, Error> {
        match self.peek_from_stack(name.clone()) {
            Some(val) => {
                for _ in 0..n {
                    let _ = match val.dup() {
                        Ok(nval) => self.push_to_stack(name.clone(), nval),
                        Err(err) => {
                            bail!("Error duplicating data in current stack: {}", err);
                        }
                    };
                }
                return Ok(self);
            }
            None => {
                bail!("Error duplicating data in current stack: unable to peek()");
            }
        }
    }

    pub fn many_in_current_stack(&mut self, n: i64, value: Value) -> Result<&mut TS, Error> {
        for _ in 0..n {
            let _ = match value.dup() {
                Ok(nval) => self.push(nval),
                Err(err) => {
                    bail!("Error duplicating data in current stack: {}", err);
                }
            };
        }
        Ok(self)
    }

    pub fn many_in_stack(&mut self, n: i64, name: String, value: Value) -> Result<&mut TS, Error> {
        for _ in 0..n {
            let _ = match value.dup() {
                Ok(nval) => self.push_to_stack(name.clone(), nval),
                Err(err) => {
                    bail!("Error duplicating data in current stack: {}", err);
                }
            };
        }
        Ok(self)
    }

    pub fn swap_in_current_stack(&mut self, n: i64) -> Result<&mut TS, Error> {
        if self.current_stack_len() < 2 {
            bail!("Swap in current stack had failed. Stack too shallow.");
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
                        return Ok(self);
                    }
                    None => {
                        bail!("Swap in current stack had failed. Can not pick primary.");
                    }
                }
            }
            None => {
                bail!("Swap in current stack had failed. Can not pick secondary.");
            }
        }
    }

    pub fn swap_in_stack(&mut self, n: i64, name: String) -> Result<&mut TS, Error> {
        if self.stack_len(name.clone()) < 2 {
            bail!("Swap in stack {} had failed. Stack too shallow.", &name);
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
                        return Ok(self);
                    }
                    None => {
                        bail!("Swap in stack {} had failed. Can not pick primary.", &name);
                    }
                }
            }
            None => {
                bail!("Swap in stack {} had failed. Can not pick primary.", &name);
            }
        }
    }

}
