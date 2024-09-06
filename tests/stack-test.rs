#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use rust_multistack::stack::Stack;

    #[test]
    fn test_stack_new() {
        let s: Stack<Value> = Stack::new();
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_stack_is_empty() {
        let s: Stack<Value> = Stack::new();
        assert!(s.is_empty());
    }

    #[test]
    fn test_stack_clear() {
        let mut s: Stack<Value> = Stack::new();
        s.push(Value::from(42.0).unwrap())
         .push(Value::from(41.0).unwrap())
         .clear();
        assert!(s.is_empty());
    }

    #[test]
    fn test_stack_push() {
        let mut s: Stack<Value> = Stack::new();
        s.push(Value::from(42.0).unwrap())
         .push(Value::from(41.0).unwrap());
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_stack_pull() {
        let mut s: Stack<Value> = Stack::new();
        s.push(Value::from(41.0).unwrap())
         .push(Value::from(42.0).unwrap());
        let val = s.pull().expect("No value has been pulled");
        assert_eq!(val.cast_float().unwrap(), 42.0);
    }

    #[test]
    fn test_stack_rotate_left() {
        let mut s: Stack<Value> = Stack::new();
        s.push(Value::from(42.0).unwrap())
         .push(Value::from(41.0).unwrap())
         .left();
        let val = s.pull().expect("No value has been pulled");
        assert_eq!(val.cast_float().unwrap(), 42.0);
    }

    #[test]
    fn test_stack_rotate_left_single_element() {
        let mut s: Stack<Value> = Stack::new();
        s.push(Value::from(42.0).unwrap())
         .left();
        let val = s.pull().expect("No value has been pulled");
        assert_eq!(val.cast_float().unwrap(), 42.0);
    }

    #[test]
    fn test_stack_peek() {
        let mut s: Stack<Value> = Stack::new();
        s.push(Value::from(41.0).unwrap())
         .push(Value::from(42.0).unwrap());
        let val = s.peek().expect("No value has been pulled");
        assert_eq!(val.cast_float().unwrap(), 42.0);
    }

    #[test]
    fn test_stack_peek_check_len() {
        let mut s: Stack<Value> = Stack::new();
        s.push(Value::from(41.0).unwrap())
         .push(Value::from(42.0).unwrap());
        let _ = s.peek().expect("No value has been pulled");
        assert_eq!(s.len(), 2);
    }
}
