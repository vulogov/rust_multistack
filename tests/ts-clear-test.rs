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
    fn test_clear_current() {
        let mut ts = TS::new();
        ts.f("push".to_string(),
            Some(Value::from(42.0).unwrap()),
            None
        ).unwrap();
        ts.i("clear".to_string()).unwrap();
        assert_eq!(ts.current_stack_len(), 0);
    }

    #[test]
    fn test_clear_stack() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.f("push".to_string(),
            Some(Value::from(42.0).unwrap()),
            None
        ).unwrap();
        ts.f("clear_in".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        assert_eq!(ts.stack_len("A".to_string()), 0);
    }

}
