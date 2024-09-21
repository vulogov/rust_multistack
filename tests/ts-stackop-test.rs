#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistack::ts::TS;
    use rust_multistack::ts_pull::*;
    use rust_multistack::ts_push::*;
    use rust_multistack::ts_stack_op::*;

    #[test]
    fn test_ts_dup_in_current_stack() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .push(Value::from(43.0).unwrap());
        ts.dup_in_current_stack(1).unwrap();
        assert_eq!(ts.current_stack_len(), 4);
    }

    #[test]
    fn test_ts_dup_in_stack() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.push_to_stack("A".to_string(), Value::from(41.0).unwrap())
          .push_to_stack("A".to_string(), Value::from(42.0).unwrap())
          .push_to_stack("A".to_string(), Value::from(43.0).unwrap());
        ts.dup_in_stack(1, "A".to_string()).unwrap();
        assert_eq!(ts.stack_len("A".to_string()), 4);
    }

    #[test]
    fn test_ts_many_in_current_stack() {
        let mut ts = TS::new();
        ts.many_in_current_stack(3, Value::from(42.0).unwrap()).unwrap();
        assert_eq!(ts.current_stack_len(), 3);
    }

    #[test]
    fn test_ts_many_in_stack() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts_many_in_stack(&mut ts, 3, "A".to_string(), Value::from(42.0).unwrap()).unwrap();
        assert_eq!(ts.stack_len("A".to_string()), 3);
    }

    #[test]
    fn test_ts_swap_in_current_stack() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .push(Value::from(43.0).unwrap());
        ts.swap_in_current_stack(1).unwrap();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_ts_swap_in_stack() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.push_to_stack("A".to_string(), Value::from(41.0).unwrap())
          .push_to_stack("A".to_string(), Value::from(42.0).unwrap())
          .push_to_stack("A".to_string(), Value::from(43.0).unwrap());
        ts.swap_in_stack(1, "A".to_string()).unwrap();
        let val = ts.pull_from_stack("A".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_inline_dup() {
        let mut ts = TS::new();
        ts.f("push".to_string(),
            Some(Value::from(42.0).unwrap()),
            None
        ).unwrap();
        ts.i("dup_one".to_string()).unwrap();
        assert_eq!(ts.current_stack_len(), 2);
    }

}
