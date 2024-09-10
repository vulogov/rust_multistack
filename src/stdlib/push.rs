use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_push_value_to_current(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(val) => {
            ts.push(val);
        }
        None => {
            bail!("Value is missed for a push() operation");
        }
    }
    Ok(ts)
}

pub fn stdlib_push_value_to_stack(ts: &mut TS, value1: Option<Value>, value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
            match value2 {
                Some(val) => {
                    match name_val.cast_string() {
                        Ok(name) => {
                            return Ok(ts.push_to_stack(name, val));
                        }
                        Err(err) => {
                            bail!("Operation push_to() returned error: {}", err);
                        }
                    }
                }
                None => {
                    bail!("Value is missed for a push_to() operation");
                }
            }
        }
        None => {
            bail!("Name of stack is missed for a push_to() operation");
        }
    }
}


pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("push".to_string(), stdlib_push_value_to_current);
    let _ = ts.register_function("push_to".to_string(), stdlib_push_value_to_stack);
}
