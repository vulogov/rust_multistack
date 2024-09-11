use crate::ts::*;
use easy_error::{bail, Error};

impl TS {
    pub fn register_inline(&mut self, name: String, fun: InlineFn) -> Result<&mut TS, Error> {
        match self.unregister_inline(format!("{}_inline", &name)) {
            Ok(_) => {
                self.inline_fun.insert(format!("{}_inline", &name), fun);
            }
            Err(err) => {
                bail!("Inline unregistering returns error: {}", err);
            }
        }
        Ok(self)
    }

    pub fn unregister_inline(&mut self, name: String) -> Result<&mut TS, Error> {
        if self.inline_fun.contains_key(&name) {
            self.inline_fun.remove(&name);
        }
        Ok(self)
    }

    pub fn get_inline(&mut self, name: String) -> Result<InlineFn, Error> {
        if self.inline_fun.contains_key(&format!("{}_inline", &name)) {
            return match self.inline_fun.get(&format!("{}_inline", &name)) {
                Some(fun) => Ok(*fun),
                None => bail!("Inline {} is registered, but not found.", &name),
            };
        }
        bail!("Inline {} not registered", &name);
    }

    pub fn i(&mut self, name: String) -> Result<&mut TS, Error> {
        match self.get_inline(name.clone()) {
            Ok(fun) => {
                return fun(self);
            }
            Err(err) => {
                bail!("i({}) returned: {}", &name, err);
            }
        }
    }

}
