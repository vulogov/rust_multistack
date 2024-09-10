use crate::ts::TS;

pub mod push;
pub mod workbench;
pub mod dup;

pub fn init_stdlib(ts: &mut TS) {
    push::init_stdlib(ts);
    workbench::init_stdlib(ts);
    dup::init_stdlib(ts);
}
