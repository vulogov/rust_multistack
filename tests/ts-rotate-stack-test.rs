#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistack::ts::TS;
    use rust_multistack::ts_pull::*;
    use rust_multistack::ts_push::*;
    use rust_multistack::ts_rotate_stack::*;

    #[test]
    fn test_ts_rotate_current_stack_left() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .push(Value::from(43.0).unwrap());
        ts.rotate_current_stack_left();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 41.0 as f64);
    }

    #[test]
    fn test_ts_rotate_current_stack_right() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .push(Value::from(43.0).unwrap());
        ts.rotate_current_stack_right();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_ts_rotate_named_stack_left() {
        let mut ts = TS::new_with_named("Z".to_string());
        ts.add_named_stack("A".to_string())
          .add_named_stack("B".to_string())
          .add_named_stack("C".to_string());
        ts.push_to_stack("B".to_string(), Value::from(41.0).unwrap())
          .push_to_stack("B".to_string(), Value::from(42.0).unwrap())
          .push_to_stack("B".to_string(), Value::from(43.0).unwrap());
        ts.rotate_stack_left("B".to_string());
        let val = ts_pull_from_stack(&mut ts, "B".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 41.0 as f64);
    }

    #[test]
    fn test_ts_rotate_named_stack_right() {
        let mut ts = TS::new_with_named("Z".to_string());
        ts.add_named_stack("A".to_string())
          .add_named_stack("B".to_string())
          .add_named_stack("C".to_string());
        ts.push_to_stack("B".to_string(), Value::from(41.0).unwrap())
          .push_to_stack("B".to_string(), Value::from(42.0).unwrap())
          .push_to_stack("B".to_string(), Value::from(43.0).unwrap());
        ts_rotate_stack_right(&mut ts, "B".to_string());
        let val = ts_pull_from_stack(&mut ts, "B".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

}
