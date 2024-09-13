use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_ensure_stack(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
            match name_val.cast_string() {
                Ok(name) => {
                    ts.ensure_stack(name);
                }
                Err(err) => {
                    bail!("Operation ensure_stack() returned error: {}", err);
                }
            }
        }
        None => {
            bail!("Name of the stack is missed for a ensure_stack() operation");
        }
    }
    Ok(ts)
}

pub fn stdlib_ensure_stack_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline ensure_stack()");
    }
    let name = ts.pull();
    stdlib_ensure_stack(ts, name, None)
}

pub fn stdlib_stack_exists_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline stack_exists()");
    }
    let name = ts.pull();
    match name {
        Some(name) => {
            match name.cast_string() {
                Ok(stack_name) => {
                    match Value::from(ts.stack_exists(stack_name)) {
                        Ok(val) => {
                            ts.push(val);
                        }
                        Err(err) => {
                            bail!("stack_exists() can not create responce: {}", err);
                        }
                    }
                }
                Err(err) => {
                    bail!("stack exists() returned: {}", err);
                }
            }
        }
        None => {
            bail!("stack_exists() can not get stack name");
        }
    }
    Ok(ts)
}

pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("ensure_stack".to_string(), stdlib_ensure_stack);
    let _ = ts.register_inline("ensure_stack".to_string(), stdlib_ensure_stack_inline);
    let _ = ts.register_inline("stack_exists".to_string(), stdlib_stack_exists_inline);
}
