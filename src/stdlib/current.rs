use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};


pub fn stdlib_to_current(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
                match name_val.cast_string() {
                    Ok(name) => {
                        return ts.to_current(name);
                    }
                    Err(err) => {
                        bail!("Operation to_current() returned error: {}", err);
                    }
                }
        }
        None => {
            bail!("Name of stack is missed for a to_current() operation");
        }
    }
}

pub fn stdlib_to_current_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline to_current()");
    }
    let name = ts.pull();
    stdlib_to_current(ts, name, None)
}

pub fn stdlib_to_stack(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
                match name_val.cast_string() {
                    Ok(name) => {
                        return ts.to_stack(name);
                    }
                    Err(err) => {
                        bail!("Operation to_stack() returned error: {}", err);
                    }
                }
        }
        None => {
            bail!("Name of stack is missed for a to_stack() operation");
        }
    }
}

pub fn stdlib_to_stack_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline to_stack()");
    }
    let name = ts.pull();
    stdlib_to_stack(ts, name, None)
}

pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("to_current".to_string(), stdlib_to_current);
    let _ = ts.register_inline("to_current".to_string(), stdlib_to_current_inline);
    let _ = ts.register_function("to_stack".to_string(), stdlib_to_stack);
    let _ = ts.register_inline("to_stack".to_string(), stdlib_to_stack_inline);
}
