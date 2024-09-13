use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};


pub fn stdlib_drop_in_current(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match ts.drop() {
        Ok(ts) => {
            return Ok(ts);
        }
        Err(err) => {
            bail!("Function drop() returned: {}", err);
        }
    }
}

pub fn stdlib_drop_in_current_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    stdlib_drop_in_current(ts, None, None)
}

pub fn stdlib_drop_in_stack(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
            match name_val.cast_string() {
                Ok(name) => {
                    match ts.drop_in(name.clone()) {
                        Ok(ts) => {
                            return Ok(ts);
                        }
                        Err(err) => {
                            bail!("Operation drop_in() returned: {}", err);
                        }
                    }
                }
                Err(err) => {
                    bail!("Operation drop_in() returned error: {}", err);
                }
            }
        }
        None => {
            bail!("Name of stack is missed for a drop_in() operation");
        }
    }
}

pub fn stdlib_drop_in_stack_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline drop_in()");
    }
    let name = ts.pull();
    stdlib_drop_in_stack(ts, name, None)
}

pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("drop".to_string(), stdlib_drop_in_current);
    let _ = ts.register_inline("drop".to_string(), stdlib_drop_in_current_inline);
    let _ = ts.register_function("drop_in".to_string(), stdlib_drop_in_stack);
    let _ = ts.register_inline("drop_in".to_string(), stdlib_drop_in_stack_inline);
}
