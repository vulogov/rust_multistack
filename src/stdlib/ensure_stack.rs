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

pub fn stdlib_set_stack_capacity_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline set_stack_capacity()");
    }
    let name_value = match ts.pull() {
        Some(name) => name,
        None => {
            bail!("capacity returns NO DATA #1");
        }
    };
    let cap_value = match ts.pull() {
        Some(name) => name,
        None => {
            bail!("capacity returns NO DATA #2");
        }
    };
    let name_str = match name_value.cast_string() {
        Ok(name_str) => name_str,
        Err(err) => {
            bail!("capacity casting error #1: {}", err);
        }
    };
    let cap_int = match cap_value.cast_int() {
        Ok(cap_int) => cap_int,
        Err(err) => {
            bail!("capacity casting error #2: {}", err);
        }
    };
    Ok(ts.ensure_stack_with_capacity(name_str, cap_int as usize))
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
    let _ = ts.register_inline("ensure_stack_with_capacity".to_string(), stdlib_set_stack_capacity_inline);
    let _ = ts.register_inline("stack_exists".to_string(), stdlib_stack_exists_inline);
}
