use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};


pub fn stdlib_clear_in_current(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match ts.current() {
        Some(stack) => {
            stack.clear();
        }
        None => {
            bail!("Can not locate current stack for clear()");
        }
    }
    Ok(ts)
}

pub fn stdlib_clear_in_current_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    stdlib_clear_in_current(ts, None, None)
}

pub fn stdlib_clear_in_stack(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
            match name_val.cast_string() {
                Ok(name) => {
                    match ts.stack(name.clone()) {
                        Some(stack) => {
                            stack.clear();
                        }
                        None => {
                            bail!("Operation clear_in() can not find the stack: {}", &name);
                        }
                    }
                }
                Err(err) => {
                    bail!("Operation clear_in() returned error: {}", err);
                }
            }
        }
        None => {
            bail!("Name of stack is missed for a clear_in() operation");
        }
    }
    Ok(ts)
}

pub fn stdlib_clear_in_stack_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline clear_in()");
    }
    let name = ts.pull();
    stdlib_clear_in_stack(ts, name, None)
}

pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("clear".to_string(), stdlib_clear_in_current);
    let _ = ts.register_inline("clear".to_string(), stdlib_clear_in_current_inline);
    let _ = ts.register_function("clear_in".to_string(), stdlib_clear_in_stack);
    let _ = ts.register_inline("clear_in".to_string(), stdlib_clear_in_stack_inline);
}
