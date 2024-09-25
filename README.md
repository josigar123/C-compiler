# C-compiler
A primitive C-compiler for an extremely small subset of the language written in Rust ( first time use of the language ).
It compiles to ARM64 assembly, no other architecture is currently supported.

## Compiles:
Supports integer binary operations such as: +, -, &&, ||, * /, <=, >= ==, !=, <, >
Supports integer unary operators such as: !, ^, ˝
These operations work recursively and can be applied a variable amount of times.
Complex binary operator chains also gets computed correctly such as: !1 + 2 && 3 < 4; is valid.
It compiles and correcly labels function definitions (no arguments) but does not check for the existence of a
main function to execute from and will execute from the first instruction met during execution. You can define as
many functions as you like.

Variable declaration and assignment is also functional ( and by extension booleans ). Operations on variables is
still quite buggy and and does not work as expected ( the exception is an expression containing only two variables e.g. a + b; ).

It also compiles garbage lines such as 1 + 3 - b < z; without being stored or returned by a function.

The return statement works as expected for the valid compilations mentioned above, a function can return single values or expressions.

## Memory
The compiler only deals in the process's stack memory using a simple symbol-table to keep track of allocated bytes
aswell as maintaining the proper offsets and boundaries.
Heap allocation and retrieval is not supported.
