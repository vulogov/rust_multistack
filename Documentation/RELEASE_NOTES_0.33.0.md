# What's new in 0.33.0

* Function ```TS::add_named_fifo()``` that is support creating FIFO-policy stack.
* TS::pull() supports both LIFO and FIFO policies.
* Function ```TS::fold_all_in_current()``` folding all data in stack into list, including NODATA elements
* Functions TS::fold_current() and TS::fold_stack() do recognize NODATA and stop folding if NODATA is found in stack
* Function ```ts_move_to_current_stack()``` Moving elements from named datack to current stack
* Functions ```TS::[move_from_current()|move_to_current()|move_from_stack()]``` recognize NODATA and stop moving if found.
