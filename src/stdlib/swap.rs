use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_swap_in_current(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(ndup) => {
            match ndup.cast_int() {
                Ok(n) => {
                    return ts.swap_in_current_stack(n);
                }
                Err(err) => {
                    bail!("Operation swap() returned error: {}", err);
                }
            }
        }
        None => {
            bail!("Number of operations is missed for a swap() operation");
        }
    }
}

pub fn stdlib_swap_in_current_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline swap()");
    }
    let n = ts.pull();
    stdlib_swap_in_current(ts, n, None)
}

pub fn stdlib_swap_one_in_current_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline swap_one()");
    }
    match ts.pull() {
        Some(value1) => {
            match ts.pull() {
                Some(value2) => {
                    ts.push(value1);
                    ts.push(value2);
                }
                None => {
                    bail!("Value #2 missed for swap_one()");
                }
            }
        }
        None => {
            bail!("Value #1 missed for swap_one()");
        }
    }
    Ok(ts)
}

pub fn stdlib_swap_in_stack(ts: &mut TS, value1: Option<Value>, value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
            match name_val.cast_string() {
                Ok(name) => {
                    match value2 {
                        Some(ndup) => {
                            match ndup.cast_int() {
                                Ok(n) => {
                                    return ts.swap_in_stack(n, name);
                                }
                                Err(err) => {
                                    bail!("Operation dup() returned error: {}", err);
                                }
                            }
                        }
                        None => {
                            bail!("Number of operations is missed for a swap_in() operation");
                        }
                    }
                }
                Err(err) => {
                    bail!("Operation swap_in() returned error: {}", err);
                }
            }
        }
        None => {
            bail!("Name of stack is missed for a swap_in() operation");
        }
    }
}

pub fn stdlib_swap_in_stack_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline swap_in()");
    }
    let n = ts.pull();
    let name = ts.pull();
    stdlib_swap_in_stack(ts, n, name)
}

pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("swap".to_string(), stdlib_swap_in_current);
    let _ = ts.register_function("swap_in".to_string(), stdlib_swap_in_stack);
    let _ = ts.register_inline("swap".to_string(), stdlib_swap_in_current_inline);
    let _ = ts.register_inline("swap_in".to_string(), stdlib_swap_in_stack_inline);
    let _ = ts.register_inline("swap_one".to_string(), stdlib_swap_one_in_current_inline);
}
