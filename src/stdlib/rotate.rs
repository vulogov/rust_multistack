use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_stacks_left(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    Ok(ts.left())
}

pub fn stdlib_stacks_left_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    stdlib_stacks_left(ts, None, None)
}

pub fn stdlib_stacks_right(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    Ok(ts.right())
}

pub fn stdlib_stacks_right_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    stdlib_stacks_right(ts, None, None)
}

pub fn stdlib_current_left(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    Ok(ts.rotate_current_stack_left())
}

pub fn stdlib_current_left_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    stdlib_current_left(ts, None, None)
}

pub fn stdlib_current_right(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    Ok(ts.rotate_current_stack_right())
}

pub fn stdlib_current_right_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    stdlib_current_right(ts, None, None)
}

pub fn stdlib_stack_left(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
            match name_val.cast_string() {
                Ok(name) => {
                    ts.rotate_stack_left(name);
                }
                Err(err) => {
                    bail!("Operation rotate_stack_left() returned error: {}", err);
                }
            }
        }
        None => {
            bail!("Name of the stack is missed for a ensure_stack() operation");
        }
    }
    Ok(ts)
}

pub fn stdlib_stack_left_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline rotate_stack_left()");
    }
    let name = ts.pull();
    stdlib_stack_left(ts, name, None)
}

pub fn stdlib_stack_right(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
            match name_val.cast_string() {
                Ok(name) => {
                    ts.rotate_stack_right(name);
                }
                Err(err) => {
                    bail!("Operation rotate_stack_right() returned error: {}", err);
                }
            }
        }
        None => {
            bail!("Name of the stack is missed for a rotate_stack_right() operation");
        }
    }
    Ok(ts)
}

pub fn stdlib_stack_right_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline rotate_stack_right()");
    }
    let name = ts.pull();
    stdlib_stack_left(ts, name, None)
}


pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("stacks_left".to_string(), stdlib_stacks_left);
    let _ = ts.register_inline("stacks_right".to_string(), stdlib_stacks_right_inline);
    let _ = ts.register_function("rotate_current_left".to_string(), stdlib_current_left);
    let _ = ts.register_inline("rotate_current_left".to_string(), stdlib_current_left_inline);
    let _ = ts.register_function("rotate_current_right".to_string(), stdlib_current_right);
    let _ = ts.register_inline("rotate_current_right".to_string(), stdlib_current_right_inline);
    let _ = ts.register_function("rotate_stack_left".to_string(), stdlib_stack_left);
    let _ = ts.register_inline("rotate_stack_left".to_string(), stdlib_stack_left_inline);
    let _ = ts.register_function("rotate_stack_right".to_string(), stdlib_stack_right);
    let _ = ts.register_inline("rotate_stack_right".to_string(), stdlib_stack_right_inline);
}
