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
    fn test_ts_from_current_to_workbench1() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .push(Value::from(43.0).unwrap());
        ts.return_from_current_to_workbench().unwrap();
        assert_eq!(ts.current_stack_len(), 2);
    }

    #[test]
    fn test_ts_from_current_to_workbench2() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .push(Value::from(43.0).unwrap());
        ts.return_from_current_to_workbench().unwrap();
        let val = ts.pull_from_workbench().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 43.0 as f64);
    }

    #[test]
    fn test_ts_from_workbench_to_current() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .push(Value::from(43.0).unwrap());
        ts.return_from_current_to_workbench().unwrap();
        ts.return_from_workbench_to_current().unwrap();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 43.0 as f64);
    }

    #[test]
    fn test_ts_stack_to_workbench() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.push_to_stack("A".to_string(), Value::from(41.0).unwrap())
          .push_to_stack("A".to_string(), Value::from(42.0).unwrap())
          .push_to_stack("A".to_string(), Value::from(43.0).unwrap());
        ts.return_from_stack_to_workbench("A".to_string()).unwrap();
        let val = ts.pull_from_workbench().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 43.0 as f64);
    }

    #[test]
    fn test_ts_workbench_to_stack() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.push_to_stack("A".to_string(), Value::from(41.0).unwrap())
          .push_to_stack("A".to_string(), Value::from(42.0).unwrap())
          .push_to_stack("A".to_string(), Value::from(43.0).unwrap());
        ts.return_from_stack_to_workbench("A".to_string()).unwrap();
        ts.return_from_workbench_to_stack("B".to_string()).unwrap();
        let val = ts.pull_from_stack("B".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 43.0 as f64);
    }

}
