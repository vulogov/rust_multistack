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
    fn test_inline_workbench1() {
        let mut ts = TS::new();
        ts.f("push".to_string(),
            Some(Value::from(42.0).unwrap()),
            None
        ).unwrap();
        ts.i("return".to_string()).unwrap();
        let val = ts.pull_from_workbench().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_inline_workbench2() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.f("push_to".to_string(),
            Some(Value::from("A").unwrap()),
            Some(Value::from(42.0).unwrap())
        ).unwrap();
        ts.f("push".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();

        ts.i("return_from".to_string()).unwrap();
        let val = ts.pull_from_workbench().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_inline_workbench3() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.f("push_to".to_string(),
            Some(Value::from("A").unwrap()),
            Some(Value::from(42.0).unwrap())
        ).unwrap();
        ts.f("push".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        ts.i("return_from".to_string()).unwrap();

        ts.f("push".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        ts.i("return_to".to_string()).unwrap();
        let val = ts.pull_from_stack("A".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_stack_exists() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.f("push".to_string(),
            Some(Value::from("A").unwrap()),
            None
        ).unwrap();
        ts.i("stack_exists".to_string()).unwrap();
        let val = ts.pull_from_stack("A".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_bool().unwrap(), true);
    }

    #[test]
    fn test_inline_workbench4() {
        let mut ts = TS::new();
        ts.f("push".to_string(),
            Some(Value::from(42.0).unwrap()),
            None
        ).unwrap();
        ts.i("return".to_string()).unwrap();
        ts.i("take".to_string()).unwrap();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

}
