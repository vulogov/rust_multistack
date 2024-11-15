use crate::ts::TS;

impl TS {

    pub fn stack_capacity(&self, name: String) -> Option<usize> {
        if self.stack_cap.contains_key(&name) {
            match self.stack_cap.get(&name) {
                Some(cap) => {
                    return Some(*cap);
                }
                None => {
                    return None;
                }
            }
        }
        None
    }
}
