#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;
    use rust_multistack::ts::TS;

    #[test]
    fn test_ts_push_to_current() {
        let mut ts = TS::new();
        ts.push(Value::from(42.0).unwrap());
        assert_eq!(ts.current_stack_len(), 1);
    }

    #[test]
    fn test_ts_push_to_named() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.push_to_stack("A".to_string(), Value::from(42.0).unwrap());
        assert_eq!(ts.stack_len("A".to_string()), 1);
    }

}
