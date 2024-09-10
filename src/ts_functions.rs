use crate::ts::*;
use rust_dynamic::value::Value;
use easy_error::{bail, Error};

impl TS {
    pub fn register_function(&mut self, name: String, fun: AppFn) -> Result<&mut TS, Error> {
        match self.unregister_function(name.clone()) {
            Ok(_) => {
                self.functions.insert(name.clone(), fun);
            }
            Err(err) => {
                bail!("Function unregistering returns error: {}", err);
            }
        }
        Ok(self)
    }

    pub fn unregister_function(&mut self, name: String) -> Result<&mut TS, Error> {
        if self.functions.contains_key(&name) {
            self.functions.remove(&name);
        }
        Ok(self)
    }

    pub fn get_function(&mut self, name: String) -> Result<AppFn, Error> {
        if self.functions.contains_key(&name) {
            return match self.functions.get(&name) {
                Some(fun) => Ok(*fun),
                None => bail!("Function {} is registered, but not found.", &name),
            };
        }
        bail!("Function {} not registered", &name);
    }

    pub fn f(&mut self, name: String, value1: Option<Value>, value2: Option<Value>) -> Result<&mut TS, Error> {
        match self.get_function(name.clone()) {
            Ok(fun) => {
                return fun(self, value1, value2);
            }
            Err(err) => {
                bail!("f({}) returned: {}", &name, err);
            }
        }
    }

}
