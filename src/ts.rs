use crate::stack::Stack;
use crate::stdlib::init_stdlib;
use std::collections;
use rust_dynamic::value::Value;
use easy_error::{Error};
use nanoid::nanoid;

pub type AppFn      = fn(&mut TS, Option<Value>, Option<Value>) -> Result<&mut TS, Error>;
pub type InlineFn   = fn(&mut TS) -> Result<&mut TS, Error>;

///
/// Principial structure provining interface to all funcitonality of multi-stack. To create TS structure you shall call TS::new() or TS::new_with_named(name)
///
#[derive(Clone)]
pub struct TS {
    pub id:             String,
    pub stack:          collections::HashMap<String, Stack<Value>>,
    pub stacks:         collections::VecDeque<String>,
    pub workbench:      Stack<Value>,
    pub functions:      collections::HashMap<String, AppFn>,
    pub inline_fun:     collections::HashMap<String, InlineFn>,
}

impl TS {
    fn init() -> Self {
        Self {
            id:         nanoid!(),
            stack:      collections::HashMap::new(),
            stacks:     collections::VecDeque::new(),
            workbench:  Stack::new(),
            functions:  collections::HashMap::new(),
            inline_fun: collections::HashMap::new(),
        }
    }
    ///
    /// Create and initialize TS structure with a single anonymous stack
    ///
    pub fn new() -> Self {
        let mut res = TS::init();
        init_stdlib(&mut res);
        res.ensure();
        res
    }
    ///
    /// Create and initialize TS structure with a single named stack
    ///
    pub fn new_with_named(name: String) -> Self {
        let mut res = TS::init();
        init_stdlib(&mut res);
        res.ensure_stack(name);
        res
    }
}
