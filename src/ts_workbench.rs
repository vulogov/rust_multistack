use crate::ts::TS;
use rust_dynamic::value::Value;

impl TS {
    pub fn pull_from_workbench(&mut self) -> Option<Value> {
        self.workbench.pull()
    }
    pub fn push_to_workbench(&mut self, value: Value) -> &mut TS {
        self.workbench.push(value.clone());
        return self;
    }
    pub fn return_from_current_to_workbench(&mut self) -> &mut TS {
        self.ensure();
        match self.pull() {
            Some(val) => self.push_to_workbench(val.clone()),
            None => {return self;}
        };
        self
    }

    pub fn return_from_stack_to_workbench(&mut self, name: String) -> &mut TS {
        self.ensure();
        match self.pull_from_stack(name.clone()) {
            Some(val) => self.push_to_workbench(val.clone()),
            None => {return self;}
        };
        self
    }

    pub fn return_from_workbench_to_current(&mut self) -> &mut TS {
        self.ensure();
        match self.pull_from_workbench() {
            Some(val) => self.push(val.clone()),
            None => {return self;}
        };
        self
    }

    pub fn return_from_workbench_to_stack(&mut self, name: String) -> &mut TS {
        self.ensure();
        match self.pull_from_workbench() {
            Some(val) => self.push_to_stack(name.clone(), val.clone()),
            None => {return self;}
        };
        self
    }
}
