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
    fn test_function_push1() {
        let mut ts = TS::new();
        ts.f("push".to_string(),
            Some(Value::from(42.0).unwrap()),
            None
        ).unwrap();
        assert_eq!(ts.current_stack_len(), 1);
    }

    #[test]
    fn test_function_push2() {
        let mut ts = TS::new();
        ts.f("push".to_string(),
            Some(Value::from(42.0).unwrap()),
            None
        ).unwrap();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_function_push_to1() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.f("push_to".to_string(),
            Some(Value::from("A").unwrap()),
            Some(Value::from(42.0).unwrap())
        ).unwrap();
        assert_eq!(ts.stack_len("A".to_string()), 1);
    }

    #[test]
    fn test_function_push_to2() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.f("push_to".to_string(),
            Some(Value::from("A").unwrap()),
            Some(Value::from(42.0).unwrap())
        ).unwrap();
        let val = ts_pull_from_stack(&mut ts, "A".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_function_workbench1() {
        let mut ts = TS::new();
        ts.f("push".to_string(),
            Some(Value::from(42.0).unwrap()),
            None
        ).unwrap();
        ts.f("return".to_string(),
            None,
            None
        ).unwrap();
        let val = ts.pull_from_workbench().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_function_workbench2() {
        let mut ts = TS::new();
        ts.f("push".to_string(),
            Some(Value::from(42.0).unwrap()),
            None
        ).unwrap();
        ts.f("return".to_string(),
            None,
            None
        ).unwrap();
        ts.f("from_workbench".to_string(),
            None,
            None
        ).unwrap();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_function_workbench3() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.f("push_to".to_string(),
            Some(Value::from("A").unwrap()),
            Some(Value::from(42.0).unwrap())
        ).unwrap();
        ts.f("return_from".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        let val = ts.pull_from_workbench().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_function_workbench4() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.f("push_to".to_string(),
            Some(Value::from("A").unwrap()),
            Some(Value::from(42.0).unwrap())
        ).unwrap();
        ts.f("return_from".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        ts.f("return_to".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }
}
