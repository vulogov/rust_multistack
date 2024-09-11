use crate::ts::TS;
use easy_error::{bail, Error};


impl TS {
    pub fn to_current(&mut self, name: String) -> Result<&mut TS, Error> {
        self.ensure();
        if ! self.stack.contains_key(&name) {
            bail!("Stake with name {} noexists", &name);
        }
        let start_stack_name = match self.current_stack_name() {
            Some(stack_name) => stack_name,
            None => {
                bail!("Can not locate start stack for to_current() operation");
            }
        };
        self.left();
        loop {
            match self.stacks.back() {
                Some(stack_name) => {
                    if *stack_name == name {
                        break;
                    }
                    if *stack_name == start_stack_name {
                        bail!("We made a full circle over stacks and did not find {}", &name);
                    }
                    self.left();
                }
                None => {
                    bail!("We can not read stack name in to_current() operation");
                }
            }

        }
        return Ok(self);
    }
}
