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
    fn test_ts_to_current() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.to_current("A".to_string()).unwrap();
        assert_eq!(ts.current_stack_name().unwrap(), "A".to_string());
    }

    #[test]
    fn test_ts_to_current_inline() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.ensure_stack("B".to_string());
        ts.f("push".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        ts.i("to_current".to_string()).unwrap();
        assert_eq!(ts.current_stack_name().unwrap(), "A".to_string());
    }

}
