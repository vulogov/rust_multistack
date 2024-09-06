use crate::ts::TS;

impl TS {
    pub fn len(&self) -> usize {
        self.stacks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stacks.is_empty()
    }

}
