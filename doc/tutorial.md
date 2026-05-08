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

These operators also can operate floating-point numbers. You can try how these operators work on the
floating-point numbers by enter the following lines to the interpreter:

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
