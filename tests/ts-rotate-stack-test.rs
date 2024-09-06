#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistack::ts::TS;

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

}
