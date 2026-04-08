Walk through the IR while counting stack size and do the following substitutions

Function symbol has a HashMap of Symbol Name and Relative Position inside

# __MARK_BASE () -> ()
-push current height to base vector with depth 0 being 0

# __MARK_END () -> ()
-pop base vector

# __START_SCOPE : i () -> ()
-push current scope index to vec of index
-HashMap of symbol name and relative position is pushed into vector at the time of declaraction
-the index to used to access the symbols inside it

# __END_SCOPE () -> ()
-pop the scope index

# __LOAD_SYMBOL () -> (any)
-Get symbol position
-Calculate offset with (stack_height - (symbol_position + base_stack_position))
-push offset
-Fisherman's Gambit II 

# __LOAD_SYMBOL_PIE () -> (any)
-Position independent load that evaluates the offset ingame
-This replaces __LOAD_SYMBOL for hermes-based functions because you can't hardcode offset since you don't know where it's going to be called
-Push stack size before pushing args
-calculate and push offset of stack size at fn call 
-Fisherman's Gambit II 
-Check Symbol Table and push (internal_stack_height - symbol_position)
-Additive Distillation 
-Fisherman's Gambit II 

# Overhead of hermes function approach over inline

-Push stack size: 1
-Getting and executing fn: 5
-External symbol call: 5 per unique external symbol 
-Use inline if pattern count in body is less than overhead 