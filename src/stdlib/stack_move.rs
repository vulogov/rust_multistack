use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};


pub fn stdlib_move_from_current(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
                match name_val.cast_string() {
                    Ok(name) => {
                        ts.move_from_current(name);
                    }
                    Err(err) => {
                        bail!("Operation move() returned error: {}", err);
                    }
                }
        }
        None => {
            bail!("Name of stack is missed for a move() operation");
        }
    }
    Ok(ts)
}

pub fn stdlib_move_from_stack(ts: &mut TS, value1: Option<Value>, value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
                match name_val.cast_string() {
                    Ok(name) => {
                        match value2 {
                            Some(name_to_val) => {
                                match name_to_val.cast_string() {
                                    Ok(name_to) => {
                                        ts.move_from_stack(name, name_to);
                                    }
                                    Err(err) => {
                                        bail!("Operation move_from() returned error: {}", err);
                                    }
                                }
                            }
                            None => {
                                bail!("Name of to stack is missed for a move_from() operation");
                            }
                        }
                    }
                    Err(err) => {
                        bail!("Operation move_from() returned error: {}", err);
                    }
                }
        }
        None => {
            bail!("Name of from stack is missed for a move_from() operation");
        }
    }
    Ok(ts)
}

pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("move".to_string(), stdlib_move_from_current);
    let _ = ts.register_function("move_from".to_string(), stdlib_move_from_stack);
}
