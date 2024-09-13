use crate::ts::TS;
use rust_dynamic::value::Value;
use easy_error::{Error};


impl TS {
    pub fn fold_current(&mut self) -> Result<&mut TS, Error> {
        let mut data = Value::list();
        loop {
            match self.pull() {
                Some(value) => {
                    data = data.push(value);
                }
                None => {
                    break;
                }
            }
        }
        self.push(data);
        Ok(self)
    }
    pub fn fold_stack(&mut self, name: String) -> Result<&mut TS, Error> {
        let mut data = Value::list();
        loop {
            match self.pull_from_stack(name.clone()) {
                Some(value) => {
                    data = data.push(value);
                }
                None => {
                    break;
                }
            }
        }
        self.push_to_stack(name.clone(), data);
        Ok(self)
    }
}
