# Caffeinated Beverage Integrated Language 
The Caffeinated Bevarage Integrated Language (or CBIL) is the GRID's primary 
programming language.

## Language Requirements
The below holds the primary requirements of CBIL

1. Must be a strongly-typed language
2. Must be a hybrid (compiled to bytecode) language
3. Must be formed around a trait-based object oriented model
4. Must support Unicode initally
5. Must be able to support default parameters for functions
6. Must support overloading functions by arguments and return types
7. Must support variable-based error handling on-use


## Primitives
The following are all of the supported primitives in the language

1. Byte
2. Number 
3. Unicode character
4. Decimal (pending, 64-bit precision)
5. Structures/traits
6. Constant variables
7. NO mutable globals.
8. Arrays
9. Strings
10. Error types

## Function Definition Format
* Return values are provided at the beginning of the function definition
* The function is defined using one, optionally two, pairs of parenthesis
  * If there is one pair, then all arguments are pass by value
  * If there is two pairs, arguments in the first are pass by reference, the rest are pass by value

```
// Function taking only pass by value arguments
num my_pass_func(num passed_by_value) {
    ...


// Function taking references
num my_func(num reference)(num passed_by_value) {
    ...
```

## Error Handling
All types provide an "error" value associated with them. So for example, a 
number variable can either be valid, with its error code signaling such, or it
can be in a state called "errored", where if it is used before it is set causes
an error to be signaled. 

This check can be ignored by using the `nocheck` scope around a portion of code.
However, `nocheck` only works for the lines included in its scope. The bodies of 
functions included in `nocheck` are *not* subject to its exception so if a 
variable in a function errors out and it gets used the error is not ignored, 
even though its called from a `nocheck` scope.

Accessing a variable's error state does not constitute using the variable.
