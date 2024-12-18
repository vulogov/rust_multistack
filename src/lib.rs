pub mod stdlib;
pub mod stack;
pub mod stack_len;
pub mod stack_push;
pub mod stack_pull;
pub mod stack_rotate;
pub mod stack_clear;
pub mod stack_peek;

pub mod ts;
pub mod ts_add;
pub mod ts_len;
pub mod ts_ensure;
pub mod ts_capacity;
pub mod ts_to_current;
pub mod ts_clear;
pub mod ts_current;
pub mod ts_functions;
pub mod ts_inline;
pub mod ts_rotate;
pub mod ts_rotate_stack;
pub mod ts_push;
pub mod ts_pull;
pub mod ts_peek;
pub mod ts_move;
pub mod ts_drop;
pub mod ts_drop_stack;
pub mod ts_stack_op;
pub mod ts_list;
pub mod ts_workbench;

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}
