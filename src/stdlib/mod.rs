use crate::ts::TS;

pub mod push;

pub fn init_stdlib(ts: &mut TS) {
    push::init_stdlib(ts);
}
