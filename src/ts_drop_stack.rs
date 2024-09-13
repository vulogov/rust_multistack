use crate::ts::TS;
use easy_error::{bail, Error};

pub fn ts_drop_stack(ts: &mut TS) -> Result<&mut TS, Error> {
    ts.drop_stack()
}

impl TS {

    pub fn drop_stack(&mut self) -> Result<&mut TS, Error> {
        match self.current_stack_name() {
            Some(name) => {
                if self.stack.contains_key(&name) {
                    self.stacks.pop_back();
                    self.stack.remove(&name);
                }
            }
            None => {
                bail!("Can not detect stack name for drop_stack() operation");
            }
        }
        Ok(self)
    }
}
