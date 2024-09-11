use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_return_from_current(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    ts.return_from_current_to_workbench()
}

pub fn stdlib_return_from_current_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    ts.return_from_current_to_workbench()
}

pub fn stdlib_return_to_current(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    ts.return_from_workbench_to_current()
}

pub fn stdlib_return_from_stack(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
                match name_val.cast_string() {
                    Ok(name) => {
                        return ts.return_from_stack_to_workbench(name);
                    }
                    Err(err) => {
                        bail!("Operation return_from() returned error: {}", err);
                    }
                }
        }
        None => {
            bail!("Name of stack is missed for a return_from() operation");
        }
    }
}

pub fn stdlib_return_from_stack_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline return_from()");
    }
    let name = ts.pull();
    stdlib_return_from_stack(ts, name, None)
}

pub fn stdlib_return_to_stack(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
                match name_val.cast_string() {
                    Ok(name) => {
                        return ts.return_from_workbench_to_stack(name);
                    }
                    Err(err) => {
                        bail!("Operation return_to() returned error: {}", err);
                    }
                }
        }
        None => {
            bail!("Name of stack is missed for a return_to() operation");
        }
    }
}

pub fn stdlib_return_to_stack_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline return_to()");
    }
    let name = ts.pull();
    stdlib_return_to_stack(ts, name, None)
}

pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("return".to_string(), stdlib_return_from_current);
    let _ = ts.register_inline("return".to_string(), stdlib_return_from_current_inline);
    let _ = ts.register_function("return_from".to_string(), stdlib_return_from_stack);
    let _ = ts.register_inline("return_from".to_string(), stdlib_return_from_stack_inline);
    let _ = ts.register_function("from_workbench".to_string(), stdlib_return_to_current);
    let _ = ts.register_function("return_to".to_string(), stdlib_return_to_stack);
    let _ = ts.register_inline("return_to".to_string(), stdlib_return_to_stack_inline);
}
