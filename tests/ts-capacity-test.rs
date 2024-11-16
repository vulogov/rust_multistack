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
    fn test_capacity_1() {
        let mut ts = TS::new();
        ts.ensure_stack_with_capacity("A".to_string(), 128);
        assert_eq!(ts.stack_capacity("A".to_string()).unwrap(), 128);
    }

    #[test]
    fn test_capacity_2() {
        let mut ts = TS::new();
        ts.ensure_stack_with_capacity("A".to_string(), 1);
        ts.push_to_stack("A".to_string(), Value::from(41.0).unwrap());
        ts.push_to_stack("A".to_string(), Value::from(42.0).unwrap());
        assert_eq!(ts.stack_len("A".to_string()), 1);
    }

    #[test]
    fn test_capacity_3() {
        let mut ts = TS::new();
        ts.ensure_stack_with_capacity("A".to_string(), 1);
        ts.push_to_stack("A".to_string(), Value::from(41.0).unwrap());
        ts.push_to_stack("A".to_string(), Value::from(42.0).unwrap());
        let val = ts.pull_from_stack("A".to_string()).unwrap();
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_inline_ensure_stack_with_capacity() {
        let mut ts = TS::new();
        ts.f("push".to_string(),
            Some(Value::from(1).unwrap()),
            None
        ).unwrap();
        ts.f("push".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        ts.i("ensure_stack_with_capacity".to_string()).unwrap();
        ts.push_to_stack("A".to_string(), Value::from(41.0).unwrap());
        ts.push_to_stack("A".to_string(), Value::from(42.0).unwrap());
        assert_eq!(ts.stack_len("A".to_string()), 1);
    }

}
