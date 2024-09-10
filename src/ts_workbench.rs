use crate::ts::TS;
use rust_dynamic::value::Value;
use easy_error::{bail, Error};

pub fn ts_return_from_current_to_workbench(ts: &mut TS) -> Result<&mut TS, Error> {
    ts.return_from_current_to_workbench()
}

pub fn ts_return_from_stack_to_workbench(ts: &mut TS, name: String) -> Result<&mut TS, Error> {
    ts.return_from_stack_to_workbench(name)
}

pub fn ts_return_from_workbench_to_current(ts: &mut TS) -> Result<&mut TS, Error> {
    ts.return_from_workbench_to_current()
}

pub fn ts_return_from_workbench_to_stack(ts: &mut TS, name: String) -> Result<&mut TS, Error> {
    ts.return_from_workbench_to_stack(name)
}

impl TS {
    pub fn pull_from_workbench(&mut self) -> Option<Value> {
        self.workbench.pull()
    }
    pub fn push_to_workbench(&mut self, value: Value) -> &mut TS {
        self.workbench.push(value.clone());
        return self;
    }
    pub fn return_from_current_to_workbench(&mut self) -> Result<&mut TS, Error> {
        self.ensure();
        match self.pull() {
            Some(val) => Ok::<&mut TS, Error>(self.push_to_workbench(val.clone())),
            None => {bail!("Nothing has been returned from current stack to workbench");}
        }
    }

    pub fn return_from_stack_to_workbench(&mut self, name: String) -> Result<&mut TS, Error> {
        self.ensure();
        match self.pull_from_stack(name.clone()) {
            Some(val) => Ok::<&mut TS, Error>(self.push_to_workbench(val.clone())),
            None => {bail!("Nothing has been returned from stack {} to workbench", &name);}
        }
    }

    pub fn return_from_workbench_to_current(&mut self) -> Result<&mut TS, Error> {
        self.ensure();
        match self.pull_from_workbench() {
            Some(val) => Ok::<&mut TS, Error>(self.push(val.clone())),
            None => {bail!("Nothing has been returned from workbench to current stack");}
        }
    }

    pub fn return_from_workbench_to_stack(&mut self, name: String) -> Result<&mut TS, Error> {
        self.ensure();
        match self.pull_from_workbench() {
            Some(val) => Ok::<&mut TS, Error>(self.push_to_stack(name.clone(), val.clone())),
            None => {bail!("Nothing has been returned from workbench to stack {}", &name);}
        }
    }
}
