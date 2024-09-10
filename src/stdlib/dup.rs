use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_dup_in_current(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(ndup) => {
            match ndup.cast_int() {
                Ok(n) => {
                    return ts.dup_in_current_stack(n);
                }
                Err(err) => {
                    bail!("Operation dup() returned error: {}", err);
                }
            }
        }
        None => {
            bail!("Number of operations is missed for a dup() operation");
        }
    }
}

pub fn stdlib_dup_in_stack(ts: &mut TS, value1: Option<Value>, value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
            match name_val.cast_string() {
                Ok(name) => {
                    match value2 {
                        Some(ndup) => {
                            match ndup.cast_int() {
                                Ok(n) => {
                                    return ts.dup_in_stack(n, name);
                                }
                                Err(err) => {
                                    bail!("Operation dup() returned error: {}", err);
                                }
                            }
                        }
                        None => {
                            bail!("Number of operations is missed for a dup() operation");
                        }
                    }
                }
                Err(err) => {
                    bail!("Operation dup_in() returned error: {}", err);
                }
            }
        }
        None => {
            bail!("Name of stack is missed for a dup_in() operation");
        }
    }
}

pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("dup".to_string(), stdlib_dup_in_current);
    let _ = ts.register_function("dup_in".to_string(), stdlib_dup_in_stack);
}
