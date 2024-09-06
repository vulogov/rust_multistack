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
    fn test_ts_pull_from_current() {
        let mut ts = TS::new();
        ts.push(Value::from(42.0).unwrap());
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }

    #[test]
    fn test_ts_pull_from_stack() {
        let mut ts = TS::new();
        ts_push_to_stack(&mut ts, "A".to_string(), Value::from(42.0).unwrap());
        ts.ensure_stack("B".to_string());
        let val = ts_pull_from_stack(&mut ts, "A".to_string()).expect("No pull() happens");
        assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
    }


}
