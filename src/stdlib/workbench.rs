use crate::ts::{TS};
use rust_dynamic::value::Value;
use easy_error::{Error};

pub fn stdlib_return_from_current(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    ts.return_from_current_to_workbench()
}

pub fn stdlib_return_to_current(ts: &mut TS, _value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    ts.return_from_workbench_to_current()
}



pub fn init_stdlib(ts: &mut TS) {
    let _ = ts.register_function("return".to_string(), stdlib_return_from_current);
    let _ = ts.register_function("from_workbench".to_string(), stdlib_return_to_current);
}
