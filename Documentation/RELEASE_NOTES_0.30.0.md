# What's new in 0.30.0

* Add support for maximum stack capacity. If capacity is set for stack and it is reached, older data will be pulled from the stack.
* TS::stack_capacity(NAME) - returns stack capacity as an option or None if there is no capacity set
* TS::ensure_stack_with_capacity(NAME, capacity) - ensure that the stack exists and capacity is set.
* TS::push() is respecting stack capacity.
 
