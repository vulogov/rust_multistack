use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_return_from_current(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
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

pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("return".to_string(), stdlib_return_from_current);
    let _ = ts.register_function("return_from".to_string(), stdlib_return_from_stack);
    let _ = ts.register_function("from_workbench".to_string(), stdlib_return_to_current);
    let _ = ts.register_function("return_to".to_string(), stdlib_return_to_stack);
}
