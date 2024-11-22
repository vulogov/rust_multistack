#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistack::ts::TS;
    use rust_multistack::ts_pull::*;
    use rust_multistack::ts_push::*;


    #[test]
    fn test_ts_move_from_stack() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.push(Value::from(42.0).unwrap());
        ts.ensure_stack("B".to_string());
        ts.move_from_stack("A".to_string(), "B".to_string());
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_ts_move_from_current() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.push(Value::from(42.0).unwrap());
        ts.move_from_current("A".to_string());
        let val = ts.pull_from_stack("A".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_ts_move_from_current_with_none() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.push(Value::from(41.0).unwrap());
        ts.push(Value::nodata());
        ts.push(Value::from(42.0).unwrap());
        ts.move_from_current("A".to_string());
        let val = ts.pull_from_stack("A".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_ts_move_from_current_function() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.push(Value::from(42.0).unwrap());
        ts.f("move".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        let val = ts.pull_from_stack("A".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_ts_move_from_empty_current_function() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.f("move".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        assert_eq!(ts.stack_len("A".to_string()), 0);
    }

    #[test]
    fn test_ts_move_from_stack_function() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.push(Value::from(42.0).unwrap());
        ts.f("move_from".to_string(),
            Some(Value::from("B").unwrap()),
            Some(Value::from("A").unwrap())
        ).unwrap();
        let val = ts.pull_from_stack("A".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

}
