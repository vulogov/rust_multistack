use crate::ts::TS;

pub mod push;
pub mod workbench;
pub mod dup;
pub mod stack_move;

pub fn init_stdlib(ts: &mut TS) {
    push::init_stdlib(ts);
    workbench::init_stdlib(ts);
    dup::init_stdlib(ts);
    stack_move::init_stdlib(ts);
}
