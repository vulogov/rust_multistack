use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_fold_current(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    ts.fold_current()
}

pub fn stdlib_fold_current_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    stdlib_fold_current(ts, None, None)
}

pub fn stdlib_fold_stack(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(name_val) => {
            match name_val.cast_string() {
                Ok(name) => {
                    return ts.fold_stack(name);
                }
                Err(err) => {
                    bail!("Operation fold_stack() returned error: {}", err);
                }
            }
        }
        None => {
            bail!("Name of the stack is missed for a fold_stack() operation");
        }
    }
}

pub fn stdlib_fold_stack_inline(ts: &mut TS) -> Result<&mut TS, Error> {
    if ts.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline fold_stack()");
    }
    let name = ts.pull();
    stdlib_fold_stack(ts, name, None)
}

pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("fold".to_string(), stdlib_fold_current);
    let _ = ts.register_inline("fold".to_string(), stdlib_fold_current_inline);
    let _ = ts.register_function("fold_stack".to_string(), stdlib_fold_stack);
    let _ = ts.register_inline("fold_stack".to_string(), stdlib_fold_stack_inline);
}
