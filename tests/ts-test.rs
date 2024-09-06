#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistack::ts::TS;

    #[test]
    fn test_ts_new() {
        let ts = TS::new();
        assert_eq!(ts.len(), 1);
    }
    #[test]
    fn test_ts_clear() {
        let mut ts = TS::new();
        ts.add_stack()
          .add_stack()
          .clear();
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn test_ts_ensure() {
        let mut ts = TS::new();
        ts.ensure();
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn test_ts_ensure_stack1() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        assert_eq!(ts.len(), 2);
    }
    #[test]
    fn test_ts_ensure_stack2() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        let s = ts.current().expect("No value been pulled");
        assert_eq!(s.stack_id(), "A");
    }
    #[test]
    fn test_ts_ensure_stack3() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        let s = ts.stack("A".to_string()).expect("No value been pulled");
        assert_eq!(s.stack_id(), "A");
    }

    #[test]
    fn test_ts_stack_exists() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        assert!(ts.stack_exists("A".to_string()));
    }

    #[test]
    fn test_ts_add_named_stack() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
          .add_named_stack("B".to_string());
        let s = ts.current().expect("No value been pulled");
        assert_eq!(s.stack_id(), "B");
    }
    #[test]
    fn test_ts_current_stack_name() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
          .add_named_stack("B".to_string());
        let s_name = ts.current_stack_name().expect("No value been pulled");
        assert_eq!(s_name, "B");
    }
    #[test]
    fn test_ts_rotate_left() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
          .add_named_stack("B".to_string())
          .add_named_stack("C".to_string())
          .left();
        let s_name = ts.current_stack_name().expect("No value been pulled");
        assert_ne!(s_name, "B");
    }
    #[test]
    fn test_ts_rotate_right() {
        let mut ts = TS::new();
        ts.add_named_stack("A".to_string())
          .add_named_stack("B".to_string())
          .add_named_stack("C".to_string())
          .right();
        let s_name = ts.current_stack_name().expect("No value been pulled");
        assert_eq!(s_name, "B");
    }
}
