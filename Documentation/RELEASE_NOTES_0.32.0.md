# What's new in 0.32.0

* Function ```TS::add_named_fifo()``` that is support creating FIFO-policy stack.
* TS::pull() supports both LIFO and FIFO policies.
* Function ```TS::fold_all_in_current()``` folding all data in stack into list, including NODATA elements
* Functions TS::fold_current() and TS::fold_stack() do recognize NODATA and stop folding if NODATA is found in stack
* Function ```ts_move_to_current_stack()``` Moving last data from named datack to current stack
