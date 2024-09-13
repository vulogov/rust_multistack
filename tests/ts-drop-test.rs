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
    fn test_ts_drop_from_current() {
        let mut ts = TS::new();
        ts.push(Value::from(42.0).unwrap());
        ts.push(Value::from(41.0).unwrap());
        ts.drop().unwrap();
        assert_eq!(ts.current_stack_len(), 1);
    }

    #[test]
    fn test_ts_drop_in_stack() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.push(Value::from(42.0).unwrap());
        ts.push(Value::from(41.0).unwrap());
        ts.drop_in("A".to_string()).unwrap();
        assert_eq!(ts.stack_len("A".to_string()), 1);
    }

}
