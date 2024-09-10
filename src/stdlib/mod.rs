use crate::ts::TS;

pub mod push;
pub mod workbench;

pub fn init_stdlib(ts: &mut TS) {
    push::init_stdlib(ts);
    workbench::init_stdlib(ts);
}
