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
    fn test_fold_current() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .push(Value::from(43.0).unwrap());
        ts.fold_current().unwrap();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.len(), 3);
    }

    #[test]
    fn test_fold_stack() {
        let mut ts = TS::new();
        ts.ensure_stack("A".to_string());
        ts.push_to_stack("A".to_string(), Value::from(41.0).unwrap())
          .push_to_stack("A".to_string(), Value::from(42.0).unwrap())
          .push_to_stack("A".to_string(), Value::from(43.0).unwrap());
        ts.fold_stack("A".to_string()).unwrap();
        let val = ts.pull_from_stack("A".to_string()).expect("No pull() happens");
        assert_eq!(val.len(), 3);
    }

    #[test]
    fn test_fold_current_with_none() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::nodata())
          .push(Value::from(42.0).unwrap())
          .push(Value::from(43.0).unwrap());
        ts.fold_current().unwrap();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.len(), 2);
    }

    #[test]
    fn test_fold_all_current() {
        let mut ts = TS::new();
        ts.push(Value::from(41.0).unwrap())
          .push(Value::from(42.0).unwrap())
          .push(Value::nodata())
          .push(Value::from(43.0).unwrap());
        ts.fold_all_in_current().unwrap();
        let val = ts.pull().expect("No pull() happens");
        assert_eq!(val.len(), 4);
    }

}
