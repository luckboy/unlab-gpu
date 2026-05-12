# Unlab tutorial

## Copyright and license

Copyright (c) 2026 Łukasz Szpakowski

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.

## Introduction

The Unlab scripting language is a simple scripting language for GPU that operates on metrices. This
scripting language can be used to create and train neural networks. This tutorial can help you learn
this scripting language.

## Standard documentation

The standard documentation is a documentation for the standard library. This documentation isn't
generated while installation of the Unlab interpreter. If you have access to this documentation, you
should invoke the following command:

```
unlab-pkg std-doc
```

If your browser hasn't access to hidden files as Firefox from Ubuntu, you should change the
documentation path by add the following line to shell configuration (`.bashrc` for bash) before the
generation of standard documentation to browse this documentation:

```
export UNLAB_GPU_DOC_PATH=$HOME/unlab-gpu-doc
```

## Interpreter

An interpreter is a program that interprets entered lines or a script code in the Unlab scripting
language. The interpreter can work in an interactive mode or a non-interactive mode.

### Interactive mode

The interactive mode allows you to enter and edit lines which are interpreted. Also, the interactive
mode allows you to access to the command history by press the up key or the down key. The interpreter
can be runned by invoke the following command in the interactive mode:

```
unlab-gpu
```

The sample interaction in the interactive mode is here:

```
unlab-gpu:1> println("Hello world!!!")
Hello world!!!
unlab-gpu:2> function f(x)
> x + 1
> end
unlab-gpu:5> println(f(2))
3
unlab-gpu:6> quit
```

You can leave from the interpreter by invoke the `quit` command. If you want to browse the standard
documentation, you can run the `doc()` command or the `help()` command to browse the standard
documentation.

### Non-interactive mode

The non-interactive mode allows you execute scripts in the Unlab scritping language. The interpreter
can be runned by invoke the following command in the non-interactive mode for the `script.un` file:

```
unlab-gpu script.un
```

## Basic values

Basic values in the Unlab scripting language are represented by numbers, matrices, and strings.
Operators in the Unlab scripting language operates on the basic values.

### Numbers

Numbers in this scripting language can be integer numbers or floating-point numbers. The sample
integer numbers are here:

```unlab
1234
-1234
0
```

Also, the integer numbers can be in hexadecimal system. The sample integer in hexadecimal system are
here:

```unlab
0x12ab
0XABCD
0xffff
```

The sample floating-point numbers are here:

```unlab
12.34
-12.34
0.56
1.234e-5
1.234e+5
2e10
0.0
```

### Matrices

Matrices in this scripting language are 2D arrays which contains floating-point numbers. The sample
matrix is here:

```unlab
[
    1, 2, 3
    4, 5, 6
]
```

The matrix also can be written in one line. The sample matrices in one line are here:

```unlab
[1, 2, 3; 4, 5, 6]
[1, 1.5; 2, 2.5; 3, 3.5]
[1, 2; 3, 4]
```

The matrices can have the filled rows with the filling floating-point numbers and be filled with the
filling rows by using the `fill` keyword. You can try how the matrix rows are filled with the filling
floating-point number by enter the following lines to the interpreter:

```unlab
A = [
    1 fill 3
    2, 3, 4
    5 fill 3
]
B = [1 fill 3;  2 fill 3; 3, 4, 5]
C = [1.5 fill 2; 2.5 fill 2; 3, 3.5]
println("A = ", A)
println("B = ", B)
println("C = ", C)
```

The output of the above lines is here:

```
A = [
              1           1           1
              2           3           4
              5           5           5
]
B = [
              1           1           1
              2           2           2
              3           4           5
]
C = [
         1.5000      1.5000
         2.5000      2.5000
              3      3.5000
]
```

You can try how the filled matrices are filled with the filling row by enter the following lines to
the interpreter:

```unlab
A = [
    1, 2, 3
    fill 3
]
B = [3, 2, 1; fill 3]
C = [1 fill 2; fill 3]
println("A = ", A)
println("B = ", B)
println("C = ", C)
```

The output of the above lines is here:

```
A = [
              1           2           3
              1           2           3
              1           2           3
]
B = [
              3           2           1
              3           2           1
              3           2           1
]
C = [
              1           1
              1           1
              1           1
]
```

The filling row or the filling expression is separately evaluated for each matrix row or each element.
You can use it for example generation of random matrix by enter the following line to the
interpreter:

```unlab
println([rand() fill 3; fill 2])
```

The output of the above line is here:

```
[
         0.8534      0.4617      0.9736
         0.7208      0.5610      0.1972
]
```

Some functions from the standard library can create some matrices. These functions are the `zeros`
function, the `ones` function, and the `eye` function. These function takes the number of rows and the number of columns except the `eye` function. The `eye` function takes one number for the rows and
the columns. You can create a matrix with zeros and then show it by enter the following line to the
interpreter:

```unlab
println(zeros(2, 3))
```

The output of the above line is here:

```
[
              0           0           0
              0           0           0
]
```

You can create a matrix with ones and then show it by enter the following line to the interpreter:

```unlab
println(ones(2, 3))
```

The output of the above line is here:

```
[
              1           1           1
              1           1           1
]
```

You can create an identity matrix and then show it by enter the following line to the interpreter:

```unlab
println(eye(3))
```

The output of the above line is here:

```
[
              1           0           0
              0           1           0
              0           0           1
]
```

### Strings

Strings are texts which can be shown by the `println` function. The sample strings are here:

```
"abcdef"
"abc123"
"Hello world!!!"
""
```

### Arithmentic operators

Arithmentic operators allows you to execute the arithmetic operations on basic values. The arithmetic
operators are the `-` negation operator, the `+` addition operator, the `-` subtraction operator, the
`*` multiplication operator, and the `/` division operator. You can try how these operators work on
the integer numbers by enter the following lines to the interpreter:

```unlab
println("-1 = ", -1)
println("2 + 3 = ", 2 + 3)
println("5 - 2 = ", 5 - 2)
println("2 * 3 = ", 2 * 3)
println("5 / 2 = ", 5 / 2)
```

The output of the above lines is here:

```
-1 = -1
2 + 3 = 5
5 - 2 = 3
2 * 3 = 6
5 / 2 = 2
```

These operators also can operate on floating-point numbers. You can try how these operators work on the floating-point numbers by enter the following lines to the interpreter:

```unlab
println("-1.0 = ", -1.0)
println("2.0 + 3.0 = ", 2.0 + 3.0)
println("5.0 - 2.0 = ", 5.0 - 2.0)
println("2.0 * 3.0 = ", 2.0 * 3.0)
println("5.0 / 2.0 = ", 5.0 / 2.0)
```

The output of the above lines is here:

```
-1.0 = -1
2.0 + 3.0 = 5
5.0 - 2.0 = 3
2.0 * 3.0 = 6
5.0 / 2.0 = 2.5000
```

These operators also can operates on matrices except the `/` division operator. The multiplication two
matrices works as in linear algebra. You can try how these operators work on the matrices by enter the
following lines to the interpreter:

```unlab
A = [1, 2, 3; 4, 5, 6]
B = [4, 5, 6; 7, 8, 9]
C = [9, 8, 7; 6, 5, 4]
D = [8, 7; 6, 5; 4, 3]
println("-A = ", -A)
println("A + B = ", A + B)
println("C - A = ", C - A)
println("A * D = ", A * D)
```

The output of the above lines is here:

```
-A = [
             -1          -2          -3
             -4          -5          -6
]
A + B = [
              5           7           9
             11          13          15
]
C - A = [
              8           6           4
              2           0          -2
]
A * D = [
             32          26
             86          71
]
```

You can used these operators for the matrices and numbers. You can try how these operators work on
the matrices and the numbers by enter the following lines to the interpreter:

```unlab
A = [1, 2, 3; 4, 5, 6]
println("A + 2 = ", A + 2)
println("A - 2 = ", A - 2)
println("A * 3 = ", A * 3)
println("A / 3 = ", A / 3)
println("3 + A = ", 3 + A)
println("3 - A = ", 3 - A)
println("2 * A = ", 2 * A)
```

The output of the above lines is here:

```
A + 2 = [
              3           4           5
              6           7           8
]
A - 2 = [
             -1           0           1
              2           3           4
]
A * 3 = [
              3           6           9
             12          15          18
]
A / 3 = [
         0.3333      0.6667           1
         1.3333      1.6667           2
]
3 + A = [
              4           5           6
              7           8           9
]
3 - A = [
              2           1           0
             -1          -2          -3
]
2 * A = [
              2           4           6
              8          10          12
]
```

Also, you can use the arithmetic operators with the dot characters which operates on elements of
matrices instead of the matrices. You can try how these operators work on the matrices by enter the
following lines to the interpreter:

```unlab
A = [1, 2, 3; 4, 5, 6]
B = [4, 5, 6; 7, 8, 9]
C = [9, 8, 7; 6, 5, 4]
println(".-A = ", .-A)
println("A .+ B = ", A .+ B)
println("C .- A = ", C .- A)
println("A .* B = ", A .* B)
println("A ./ B = ", A ./ B)
```

The output of the above lines is here:

```
.-A = [
             -1          -2          -3
             -4          -5          -6
]
A .+ B = [
              5           7           9
             11          13          15
]
C .- A = [
              8           6           4
              2           0          -2
]
A .* B = [
              4          10          18
             28          40          54
]
A ./ B = [
         0.2500      0.4000      0.5000
         0.5714      0.6250      0.6667
]
```

The arithmetic operators with the dot characters can operates the matrices and the numbers. You can
try how these operators work on the matrices and the numbers by enter the following lines to the
interpreter:

```unlab
A = [1, 2, 3; 4, 5, 6]
println("A .+ 2 = ", A .+ 2)
println("A .- 2 = ", A .- 2)
println("A .* 3 = ", A .* 3)
println("A ./ 3 = ", A ./ 3)
println("3 .+ A = ", 3 .+ A)
println("3 .- A = ", 3 .- A)
println("2 .* A = ", 2 .* A)
println("2 ./ A = ", 2 ./ A)
```

The output of the above lines is here:

```
A .+ 2 = [
              3           4           5
              6           7           8
]
A .- 2 = [
             -1           0           1
              2           3           4
]
A .* 3 = [
              3           6           9
             12          15          18
]
A ./ 3 = [
         0.3333      0.6667           1
         1.3333      1.6667           2
]
3 .+ A = [
              4           5           6
              7           8           9
]
3 .- A = [
              2           1           0
             -1          -2          -3
]
2 .* A = [
              2           4           6
              8          10          12
]
2 ./ A = [
              2           1      0.6667
         0.5000      0.4000      0.3333
]
```

Two strings can be concatated by using the `+` addition operator. You can concatenate two strings by enter the following line to the interpreter:

```unlab
println("abc" + "def")
```

The output of the above line is here:

```
abcdef
```

### Comparison operators

Comparison operators are used to compare two numbers. The integer numbers and the floating-point
numbers can be compared. The boolean values are returned by the comparison operators. The compareson
operators are:

- `==` - equal
- `!=` - not equal
- `<` - less
- `>=` - greater than or equal to
- `>` - greater
- `<=` - less than or equal to

The matrices isn't compared by these operators. You can try how these operators work on the integer
numbers by enter the following lines to the interpreter:

```unlab
println("2 == 3 = ", 2 == 3)
println("2 == 2 = ", 2 == 2)
println("2 != 3 = ", 2 != 3)
println("2 != 2 = ", 2 != 2)
println("2 < 3 = ", 2 < 3)
println("2 >= 3 = ", 2 >= 3)
println("2 > 3 = ", 2 > 3)
println("2 <= 3 = ", 2 <= 3)
```

The output of the above lines is here:

```
2 == 3 = false
2 == 2 = true
2 != 3 = true
2 != 2 = false
2 < 3 = true
2 >= 3 = false
2 > 3 = false
2 <= 3 = true
```

You can try how these operators work on the floating-point numbers by enter the following lines to the 
interpreter:

```unlab
println("2.0 == 3.0 = ", 2.0 == 3.0)
println("2.0 == 2.0 = ", 2.0 == 2.0)
println("2.0 != 3.0 = ", 2.0 != 3.0)
println("2.0 != 2.0 = ", 2.0 != 2.0)
println("2.0 < 3.0 = ", 2.0 < 3.0)
println("2.0 >= 3.0 = ", 2.0 >= 3.0)
println("2.0 > 3.0 = ", 2.0 > 3.0)
println("2.0 <= 3.0 = ", 2.0 <= 3.0)
```

The output of the above lines is here:

```
2.0 == 3.0 = false
2.0 == 2.0 = true
2.0 != 3.0 = true
2.0 != 2.0 = false
2.0 < 3.0 = true
2.0 >= 3.0 = false
2.0 > 3.0 = false
2.0 <= 3.0 = true
```

Also, two strings can be compared by these operators. You can try how these operators work on the strings by enter the following lines to the interpreter:

```unlab
println("abc == def = ", "abc" == "def")
println("abc == abc = ", "abc" == "abc")
println("abc != def = ", "abc" != "def")
println("abc != abc = ", "abc" != "abc")
println("abc < def = ", "abc" < "def")
println("abc >= def = ", "abc" >= "def")
println("abc > def = ", "abc" > "def")
println("abc <= def = ", "abc" <= "def")
```

The output of the above lines is here:

```
abc == def = false
abc == abc = true
abc != def = true
abc != abc = false
abc < def = true
abc >= def = false
abc > def = false
abc <= def = true
```

### Transpose operator

A transpose operator allows you to transpose the matrix. You can try how this operator work by enter
the following lines to the interpreter:

```unlab
A = [1, 2, 3; 4, 5, 6]
println("A' = ", A')
```

The output of the above lines is here:

```
A' = [
              1           4
              2           5
              3           6
]
```

## Control flow

A control flow specifies the execution order of statements. The statements can be for example
conditions or loops in the Unlab scriting language.

### Assignment statement

An assignment statement allows you to assign a value to a variable or other assignable expression by
using the `=` character. You can try how the assignment statement works by enter the following lines to the interpreter:

```unlab
x = 1234
println("x = ", x)
```

The output of the above lines is here:

```
x = 1234
```

### If statement

If you want some statements to be conditionally executed, you can use the if statement. You can try
how the if statement works by enter the following lines to the interpreter:

```unlab
x = 1
if x > 0
    y = x + 1
    println("y = ", y)
end
```

The output of the above lines is here:

```
y = 2
```

If you want the first statements or the second statements to be executed, you can use the if statement
with the `else` keyword. The first statements are executed if the condition is fulfilled, otherwise
the second statements are executed. You can try how the if statement with the `else` keyword works by
enter the following lines to the interpreter:

```unlab
x = 2
if x == 1
    println("x is one")
else
    println("x isn't one")
end
```

The output of the above lines is here:

```
x isn't one
```

You can use the if statement for more options by using the `else` keyword and the `if` keyword. The
condition statements are executed for the first fulfilled condition or the statements after the
`else` keyword are executed. You can try how the if statement works for more options by enter the
following lines to the interpreter:

```unlab
x = 3
if x == 1
    println("x is one")
else if x == 2
    println("x is two")
else if x == 3
    println("x is three")
else
    println("x has other value")
end
```

The output of the above lines is here:

```
x is three
```

### For statement

A for statement is a loop that executes the specified number of times. The number of iterations is
specified by for example the integer range. You can try how the for statement with the integer range
works by enter the following lines to the interpreter:

```unlab
for i in 1 to 5
    println("i = ", i)
    println("i * i = ", i * i)
end
```

The output of the above lines is here:

```
i = 1
i * i = 1
i = 2
i * i = 4
i = 3
i * i = 9
i = 4
i * i = 16
i = 5
i * i = 25
```

Also, this loop can iterate over the sequence of values. You can try how the for statement with the
sequence works by enter the following lines to the interpreter:

```unlab
for i in .[ 1, 2, 4, 6 .]
    println("i = ", i)
end
```

The output of the above lines is here:

```
i = 1
i = 2
i = 4
i = 6
```

This loop indeed iterates over an iterable value and executes the statements for each element of
iterable value.

### While statement

A while statement is a loop that executes the statements for iterations until the condition isn't
fulfilled. You can try the while statement by enter the following lines to the interpreter.

```unlab
i = 1
while i <= 5
    println("i = ", i)
    println("i * i = ", i * i)
    i = i + 1
end
```

The output of the above lines is here:

```
i = 1
i * i = 1
i = 2
i * i = 4
i = 3
i * i = 9
i = 4
i * i = 16
i = 5
i * i = 25
```

### Break statement

If you want the interpreter to leave from a loop, you can use the `break` keyword as a break
statement. You can try how the break statement works by enter the following lines to the interpreter:

```unlab
for i in 1 to 10
    if i == 5
        break
    end
    println("i = ", i)
end
```

The output of the above lines is here:

```
i = 1
i = 2
i = 3
i = 4
```

### Continue statement

If you want the interpreter to skip some iterations, you can use the `continue` keyword as a continue
statement. You can try how the continue statement works by enter the following lines to the interpreter:

```unlab
for i in 1 to 10
    if i == 2 or i == 5 or i == 7 or i == 10
        continue
    end
    println("i = ", i)
end
```

The output of the above lines is here:

```
i = 1
i = 3
i = 4
i = 6
i = 8
i = 9
```

### Functions

Functions allow you to uses same code in different places. The function can have the arguments which
have different values for the different function application. Local variables can defined in the function body by the assignment statements. You can try how the function definition with applications
works by enter the following lines to the interpreter:

```unlab
function f(x, y)
    a = x * y
    b = x / y
    z = a + b
    z
end
println("f(4, 2) = ", f(4, 2))
println("f(6, 3) = ", f(6, 3))
```

The output of the above lines is here:

```
f(4, 2) = 10
f(6, 3) = 20
```

The sample function with the loop in the body and the sample applications are here:

```unlab
function f(N)
    x = 1
    for i in 1 to N
        x = x * i
    end
    x
end
println("f(0) = ", f(0))
println("f(5) = ", f(5))
println("f(10) = ", f(10))
```

The output of the above lines is here:

```
f(0) = 1
f(5) = 120
f(10) = 3628800
```

Also, the functions can be recursively applied. You can try how the recursion works on the fibonacci
sequence example by enter the following lines to the interpreter:

```unlab
function fib(N)
    if N == 0
        0
    else if N == 1
        1
    else
        fib(N - 2) + fib(N - 1)
    end
end
println("fib(0) = ", fib(0))
println("fib(1) = ", fib(1))
println("fib(5) = ", fib(5))
println("fib(10) = ", fib(10))
```

The output of the above lines is here:

```
fib(0) = 0
fib(1) = 1
fib(5) = 5
fib(10) = 55
```
