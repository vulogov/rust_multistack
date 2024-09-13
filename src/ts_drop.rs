use crate::ts::TS;
use easy_error::{bail, Error};

pub fn ts_drop_in_current(ts: &mut TS) -> Result<&mut TS, Error> {
    ts.drop()
}

pub fn ts_drop_in_stack(ts: &mut TS, name: String) -> Result<&mut TS, Error> {
    ts.drop_in(name)
}

impl TS {
    pub fn drop(&mut self) -> Result<&mut TS, Error> {
        match self.current() {
            Some(curr) => {
                match curr.pull() {
                    Some(_) => {
                        return Ok(self);
                    }
                    None => {
                        bail!("Stack is empty for drop() operation");
                    }
                }
            }
            None => {
                bail!("Can not detect stack for drop() operation");
            }
        }
    }

    pub fn drop_in(&mut self, name: String) -> Result<&mut TS, Error> {
        match self.stack(name.clone()) {
            Some(curr) => {
                match curr.pull() {
                    Some(_) => {
                        return Ok(self);
                    }
                    None => {
                        bail!("Stack is empty for drop_in() operation");
                    }
                }
            }
            None => {
                bail!("Can not detect stack {} for drop_in() operation", &name);
            }
        }
    }
}
