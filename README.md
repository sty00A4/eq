# EQ
simple calculator language in progress for expansion.

# Guide
EQ is a simple calculator language with the grammar of basic maths.
It has vectors, variables and functions.

## Vectors
Vectors are defined like this: `[...]`
Binary as well as unary operations work on vectors like below.
```
[1 2 3] + 1         ->  [2 3 4]
[1 2 3] - 1         ->  [0 1 2]
[1 2 3] + [3 2 1]   ->  [4 4 4]

[6 4 5] # 0         ->  6
[6 4 5] # 1         ->  4
[6 4 5] # 2         ->  5
```

## Variables
Functions are defined like this: `x : 1 + 2`
`x` now contains the value `3`.

## Functions
Functions are defined like variables but with parameters: `f(x) : x * 2`
This function will double what ever value is put into it.