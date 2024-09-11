# History

rust_multistack crate is a deeply reviewed and re-designed version of another Rust crate called rust_twostack. All further development will continue with rust_multistack.

# What's on the Stack ?

What is a two-dimensional stack? The rust_multistack crate offers the functionality of a two-dimensional stack structure for the Rust programming language. This data structure can contain dynamically typed values provided by the rust_dynamic crate. While the concepts of Last-In-First-Out (LIFO) and First-In-First-Out (FIFO) stacks are likely familiar to you, serving as data structures designed to store and extract values based on specific application logic, it is important to consider the potential limitations of this well-established concept.

rust_multistack offers a data structure that holds a stack of stacks of values. A single stack may not provide the necessary protections when performing complex stack-based computations:

Data isolation: When consolidating data from multiple computational steps onto a single stack, it is imperative to carefully consider the data boundaries to mitigate the risk of costly computation errors.

When setting up data processing pipelines, it is standard practice for an application to acquire and preprocess data before analyzing it. This process requires accessing diverse data sources and applying various preprocessing methods. Combining dissimilar data in the same pipeline is ill-advised and indicative of subpar design.

Hence arises the concept of the Stack-of-the-stacks, allowing you to organize data segments in individual stack spaces and process and merge them in a controlled manner.

## What are the properties of the Stack-of-the-stacks ?

* rust_multistack provides a structure called TS that controls the deque-based buffer of the stack structures
* You can rotate, create new stacks, access the current stack, and remove stacks from the Stack-of-the-stacks
* Stacks could be either anonymous or named. You can rotate Stack-of-the-stacks and position a stack based on its name to become a current stack

## How you create Stack-of-the-stacks ?

```rust
// You call a ::new() function like this
let ts = TS::new();

```
This operation will create a multistack with initial "anonymous" stack. But what if you do want to initialize your multistack with a named stack?

```rust
// You call a ::new_with_named() function like this
let ts = TS::new_with_named("Z".to_string());;

```


## How you can control Stack-of-the-stacks ?

Here is the list of TS object methods that control the Stack-of-the-stacks.

| Function name | Description |
|---|---|
| TS::new() | Creates a new instance of the Stack-of-the-stacks. Adds one anonymous stack to serve as initial default stack |
| TS::new_with_named(name) | Creates a new instance of the Stack-of-the-stacks. Adds one named stack to serve as initial default stack |
| TS.clear() | First, removes all created stacks and then adds one anonymous stack to serve as initial default stack |
| TS.ensure() | If stack-of-stacks is empty, add a new anonymous stack |
| TS.ensure_stack(name) | If stack-of-stacks is empty, add a new named stack, if stack-of-stacks is not empty, adds named stack if this stack is not existing |
| TS.stack_exists() | Returns true if stack with that name exists. Otherwise returns false |
| TS.len() | Return a number of stacks |
| TS.is_empty() | Return "true" if Stack-of-stacks is empty, "false" otherwise. Note, if you got "false", likely it is due to internal error and you have a corrupt structure. |
| TS.current_stack_len() | Return a number of values in current stack |
| TS.stack_len(name) | Return a number of values in the named stack |
| TS.current() | Return a reference to a current stack that holds the data, or None. If this function return None, it is likely due to internal error and you are dealing with corrupted structure |
| TS.stack(name) | Return a reference to a named stack that holds the data, or None. |
| TS.push() | Push data into current stack |
| TS.pull() | Pull and remove data from the current stack, returns data element or None if stack is empty |
| TS.add_stack() | Add new anonymous data stack and make it current |
| TS.add_named_stack() | Add new named data stack and make it current |
| TS.swap() | For the current stack that have to be at least 2 values deep, this method will swap last two elements of the stack |
| TS.dup() | Duplicate data value located on top of the stack |
| TS.drop() | Drop the top stack |


## How you can control data stack ?

You can access a data stack by calling TS.current(). After that, you can control it directly by calling object methods.

| Function name | Description |
|---|---|
| Stack.len() | Return the number of data elements in the stack |
| Stack.is_empty()) | Return "true" if data stack is empty |
| Stack.peek() | Return the reference on current element in the data stack without removing it from the stack. If stack is empty, returns None |
| Stack.pull() | Remove and return the current element from the stack |
| Stack.push() | Push new data element to the stack |
| Stack.clear() | Remove all elements from the stack |
| Stack.pull() | Remove and return the current element from the stack |
| Stack.left() | Rotate data stack one position to the left |
| Stack.right() | Rotate data stack one position to the right |

## What is the best way to exchange data between stacks ?

rust_multistack provides a mechanism called "Workbench." The Workbench is a single stack of values, and you can push and pull data between stacks and the Workbench.

| Function name | Description |
|---|---|
| TS.pull_from_workbench() | Return the Value element on top of the Workbench |
| TS.push_to_workbench(value) | Push arbitrary value to a Workbench |
| TS.return_from_current_to_workbench() | Pull the data from current stack and push it to a workbench |
| TS.return_from_stack_to_workbench() | Pull the data from named stack and push it to a workbench |
| TS.return_from_workbench_to_current() | Pull the data from workbench and push it to a current stack |
| TS.return_from_workbench_to_stack() | Pull the data from workbench and push it to a named stack |

```rust
    let mut ts = TS::new();
    ts.ensure_stack("A".to_string());
    ts.ensure_stack("B".to_string());
    ts.push_to_stack("A".to_string(), Value::from(41.0).unwrap())
      .push_to_stack("A".to_string(), Value::from(42.0).unwrap())
      .push_to_stack("A".to_string(), Value::from(43.0).unwrap());
    ts.return_from_stack_to_workbench("A".to_string());
    ts.return_from_workbench_to_stack("B".to_string());
    let val = ts.pull_from_stack("B".to_string()).expect("No pull() happens");
    assert_eq!(val.cast_float().unwrap(), 43.0 as f64);
```

In this example, we create two named stacks and push three elements to stack "A." Then, we take the last element from stack "A" and push it to the Workbench. After that, we pull data from the Workbench and push it to stack "B." Typically, additional computation and processing would be added to this code.

Another approach for transferring data between different stacks is to transfer data from the current stack to a named stack or between two named stacks:

| Function name | Description |
|---|---|
| Stack.move_from_current(dst) | Move all elements from current stack to named stack |
| Stack.move_from_stack(src, dst) | Move all elements from named stack to another named stack |

Here is an example of cross-stack move operation:

```rust
    let mut ts = TS::new();
    ts.ensure_stack("A".to_string());
    ts.push(Value::from(42.0).unwrap());
    ts.ensure_stack("B".to_string());
    ts.move_from_stack("A".to_string(), "B".to_string());
    let val = ts.pull().expect("No pull() happens");
    assert_eq!(val.cast_float().unwrap(), 42.0 as f64);
```

In this example we creating two naed stacks "A" and "B", pushing data to stack "A" and them move it to stack "B".


## Support for a "stack functions"

You can register a stack function to create a generic interface for performing operations with a multi-stack. First, let's define a simple function, for example, one that pushes a value to the stack. This function refers to a multi-stack as the first parameter, followed by two ``` Some()|None``` values. Stack functions can only accept none, one, or two parameters.

```rust
pub fn stdlib_push_value_to_current(ts: &mut TS, value1: Option<Value>, _value2: Option<Value>) -> Result<&mut TS, Error> {
    match value1 {
        Some(val) => {
            ts.push(val);
        }
        None => {
            bail!("Value is missed for a push() operation");
        }
    }
    Ok(ts)
}
```

Next, you have to register your function under some name, for example - "push":

```rust
let _ = ts.register_function("push".to_string(), stdlib_push_value_to_current);
```

Then using function TS::f() you can call stack function. This function expects only one parameter - the value that will be pushed to the current stack.

```rust
    let mut ts = TS::new();
    ts.f("push".to_string(),
        Some(Value::from(42.0).unwrap()),
        None
    ).unwrap();
    let val = ts.pull().expect("No pull() happens");
    assert_eq!(val.cast_float().unwrap(), 42.0 as f64);

```

### Show me the list of the stack functions

| Function name | Description | 1 | 2 |
|---|---|---|---|
| push | Pushing value to current stack | Value | None |
| push_to | Pushing value to named stack | Name | Value |
| return | Push value from the current stack to Workbench | None | None |
| from_workbench | Push value from the Workbench to current stack | None | None |
| return_to | Push value from the Workbench to named stack | Name | None |
| return_from | Push value from the named stack to Workbench | Name | None |
| dup | Duplicate value in current stack | Number of duplicates | None |
| dup_in | Duplicate value in named stack | Name | Number of duplicates |
| move | Move all values from current to named stack | Dst | None |
| move_from | Move all values from named to another named stack | Src | Dst |


## Support for "inline functions"

An inline function in a multi-stack retrieves its parameters directly from the stack. Allow me to elucidate this concept using a pseudo-assembler.

```
ensure_stack "A"    // First, we creating named stack
push_to "A" 42.0    // Then pushing a number to stack "A"
push "A"            // Now, we are pushing name of the stack from which we will read value to a current stack
call "return_from"  // This inline function will read the name of the stack from which it will read value from current stack to Workbench
push "A"            // Push the name of the stack again
call "return_to"    // Pushing value from Workbench to named stack
// Now we shall expect number 42.0 on the top of the named stack "A"
```

Which Rust code correspond to this pseudocode?

```rust
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
```

### Show me the list of inline functions

| Function name | Description | Stack |
|---|---|---|
| return | Push value from the current stack to Workbench |  |
| return_from | Push value from the named stack to Workbench | Name |
| return_to | Push value from Workbench to the named stack | Name |
