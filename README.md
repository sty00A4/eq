# EQ
simple calculator language in progress for expansion

# Guide
EQ works just like a normal calculator with the additional features of vectors, variables
and functions.

## Vectors
Vectors are defined like this: `[1 2 3]`
This vector contains the numbers `1`, `2`, `3`.
Binary and unary operations work like this:
```
[1 2 3] + 1         ->  [2 3 4]
[1 2 3] - 1         ->  [0 1 2]
[1 2 3] + [3 2 1]   ->  [4 4 4]

[1 2 3] # 0         ->  1
[1 2 3] # 1         ->  2
[1 2 3] # 2         ->  3
```

## Variables
Variables are defined like this: `x : 1 + 2`
`x` now contains the number `3`.

## Functions
Functions are defined like variables but with parameters: `f(x) : x * 2`
This function will double what ever value is put into it.