# Unlab reference

## Copyright and license

Copyright (c) 2026 Łukasz Szpakowski

This Source Code Form is subject to the terms of the Mozilla Public  
License, v. 2.0. If a copy of the MPL was not distributed with this  
file, You can obtain one at https://mozilla.org/MPL/2.0/.

## Introduction

This document is the reference for the Unlab scripting language. The Unlab scripting language is micro
neural scriting language for GPU. This reference describes the syntax of this scripting language and
semantics of this scriping language.

## Notation

This document uses EBNF to describes the lexical definitions of this scriping language and syntax of
this scripting language.

## Lexical conventions

### Whitespace

The lexical definition of whitespace is:

    whitespace = whitespace, {whitespace};
    whitespace char = ?UTF-8 character with White_Space property?

The whitespace can be separators between tokens.

### Newline

The lexical definition of newline token is:

    newline = (cr, lf) | lf | ";";
    cr = ?CR character?;
    lf = ?LF character?;

### Comments

The lexical definition of comment is:

    comment = ("#" | "%"), {?any character except LF character?};

### Punctuation

The lexical definition of punctuation tokens is:

    punctuation = "("
                | ")"
                | "["
                | "]"
                | "{"
                | "}"
                | ".["
                | ".]"
                | "?"
                | "*"
                | "/"
                | "+"
                | "-"
                | ".*"
                | "./"
                | ".+"
                | ".-"
                | "<"
                | ">="
                | ">"
                | "<="
                | "="
                | "=="
                | "!="
                | "'"
                | "."
                | ":"
                | "::"
                | ",";

A punctuation token is operator or separator.

### Keywords

The lexical definition of keywords is:

    keyword = "and"
            | "break"
            | "by"
            | "continue"
            | "else"
            | "end"
            | "false"
            | "fill"
            | "for"
            | "function"
            | "if"
            | "in"
            | "inf"
            | "module"
            | "nan"
            | "none"
            | "not"
            | "or"
            | "quit"
            | "return"
            | "root"
            | "to"
            | "true"
            | "while";

Keywords are reserved words which aren't identifiers.

### Number literals

The lexical definition of number literal is:

    number = integer | float;
    integer = "0", ("X" | "x"), hex digit, {hex digit}
            | digit, {digit};
    float = digit, {digit}, ".", digit, {digit}, [exponent]
          | digit, {digit}, exponent;
    exponent = ("E" | "e"), [("+" | "-")], digit, {digit};
    digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
    hex digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
              | "A" | "B" | "C" | "D" | "E" | "F"
              | "a" | "b" | "c" | "d" | "e" | "f";

Number literals are used to directly represent number values which can be integer numbers or
floating-point numbers. The integer numbers can be decimal system or hexadecimal system.

### String literals

The lexical definition of string literal is:

    string = '"', {string char | escape}, '"';
    string char = ?any character except '"' character, '\' character, and LF character?;
    escape = ascii escape | unicode escape;
    ascii escape = "\", oct digit, oct digit, oct digit
                 | "\", oct digit, oct digit
                 | "\", oct digit
                 | "\a" | "\b" | "\t" | "\n" | "\v" | "\f" | "\r"
                 | "\", escape char;
    escape char = ?any character except LF character?;
    unicode escape = "\U", hex digit, hex digit, hex digit, hex digit, hex digit, hex digit
                   | "\u", hex digit, hex digit, hex digit, hex digit;
    oct digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7";
    hex digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
              | "A" | "B" | "C" | "D" | "E" | "F"
              | "a" | "b" | "c" | "d" | "e" | "f";

A string literal is a text that is closed by the `"` characters. Ascii escapes can be used in the
string literals as the C programming language. The string literal also can have unicode escapes which
begin the `U` character for 24-bit code or the `u` character for 16-bit code.

### Idendifiers

The lexical definition of identifiers literals is:

    identifier = first identifier char, {identifier char};
    first identifier char = "_" | alphabetic char;
    identifier char = "_" | alphabetic char | numeric char;
    alphabetic char = ?UTF-8 character with Alphabetic property?;
    numeric char = ?UTF-8 character with Nd category, Nl category, or No category?;

## Values

Value types are:

- none
- boolean
- integer number
- floating-point number
- reference to immutable object
- strong reference to mutable object
- weak reference to mutable object

The integer numbers are 64-bit and floating-point numbers are 32-bit.

### Immutable objects

Immutable object types are:

- string
- integer range
- floating-point range
- matrix
- function
- matrix array
- matrix row slice
- error
- window identifier

### Mutable objects

Mutable object types are:

- array
- structure

### Value properties

Value types and object types with properties are:

| Value type or object type | Properties          | Index type | Element type          | Boolean    |
| ------------------------- | ------------------- | ---------- | --------------------- | ---------- |
| none                      |                     |            |                       | `false`    |
| boolean                   |                     |            |                       |            |
| integer number            |                     |            |                       | `a != 0`   |
| floating-point number     |                     |            |                       | `a != 0.0` |
| string                    | iterable, indexable | number     | string                | `true`     |
| integer range             | iterable            |            | integer number        | `true`     |
| floating-point range      | iterable            |            | floating-point number | `true`     |
| function                  | applicable          |            |                       | `true`     |
| matrix                    |                     |            |                       | `true`     |
| matrix array              | iterable, indexable | number     | matrix row slice      | `true`     |
| matrix row slice          | iterable, indexable | number     | floating-point number | `true`     |
| error                     |                     |            |                       | `false`    |
| window identifier         |                     |            |                       | `true`     |
| array                     | iterable, indexable | number     | any value             | `true`     |
| structure                 | indexable           | string     | any value             | `true`     |

Strong references only has the properties for the mutable objects.

## Nodes

The syntax of nodes is:

    nodes = {neline}, [node, {newline, {newline}, node}, [newline, {newline}]]
    node = definition | statement;

A syntax tree contains the nodes which can be definitions and/or statements.

## Definitions

The syntax of definition is:

    definition = module definition
               | function definition;

The definition is a module definition or a function definition.

### Module definitions

The syntax of module definition is:

    module definition = "module", identifier, newline,
                        nodes,
                        "end";

The module with same identifier can only be defined once in other module. The modules uses own
namespace that is separate from the variable namespace.

### Function definitions

The syntax of function definition is:

    function definition = "function", identifier, "(", arguments, ")", newline,
                          statements,
                          "end";
    arguments = [argument, {",", argument}, [","]];
    argument = identifier

The function is a variable that can be applied to arguments. If some variable with identifier is 
defined in a module, the function with same identifier can't be defined in the module. The function
arguments are checked whether they are repeated.

## Statements

The syntax of statements is:

    statements = {newline}, [statement, {newline, {newline}, statement}, [newline, {newline}]]
    statement = if statement
              | for statement
              | while statement
              | break statement
              | continue statement
              | return statement
              | quit statement
              | assign statement
              | expression;

### Assignment statements

The syntax of assignment statement is:

    assign statement = expression , "=", expression;

The assignment stament is a statement that assigns the second expression value to a variable, an 
element, or a field. The error occurs if the first expression isn't assignable.

### If statements

The syntax of if statement is:

    if statement = "if", expression, newline, statements,
                   {
                       "else", "if" expression, newline,
                       statements
                   },
                   [
                       "else", newline,
                       statements
                   ],
                   "end";

The if statement is a statement that executes the condition statement for the first fulfilled
condition that and skips other statements with conditions. If any condition isn't fulfilled, the
statements after the `else` keyword is executed. The condition is fulfilled if the condition
expression can be converted to `true`.

### For statements

The syntax of for statement is:

    for statement = "for", identifier, "in", expression, newline,
                    statements,
                    "end";

The for stamement is a loop that iterates over an iterable value and executes the statements for
elements of iterable value. The element of iterable value is stored a variable with identifier.

### While statements

The syntax of while statement is:

    while statement = "while", expression, newline
                      statements,
                      "end";

The while statement is a loop that executes the statements for iterations until expression can be
converted to `true` if the break statements doesn't occur.

### Break statements

The syntax of break statement is:

    break statement = "break";

The break statement stops a loop. If the break statement is used outside the loop, the error occurs.

### Continue statements

The syntax of continue statement is:

    continue statement = "continue";

The continue statement skips the rest statements of loop to the next iteration. If the continue
statement also is used outside the loop, the error occurs.

### Return statements

The syntax of return statement is:

    return statement = "return", [expression];

The return statement leaves from a function and then the function returns a value of expresion or the
`None` value if the expression isn't passed. If the return statement is used outside a function, the
error occurs.

### Quit statements

The syntax of quit statement is:

    quit statement = "quit";

The quit statement leaves from a script or an interpreter.

## Exressions

The syntax of expression is:

    expresion = "(", expression, ")"
              | literal
              | name
              | function application
              | unary op expression
              | binary op expression
              | logical expression
              | field access expression
              | range expression
              | error propagation expression;

### Function application

The syntax of function application is:

    function application = expression, "(", expressions, ")";
    expressions = [expression, {",", expression}, [","]];

The function application applies the function expression to the arguments. If a value of function
expression isn't function, the error occurs.

### Expressions of unary operators

The syntax of expression of unary operator is:

    unary op expression = "-", expression
                        | ".-", expression
                        | "not", expression
                        | expression, "'";

The `-` operator negates the number or the matrix.

The `.-` operator recursively negates the loating-point numbers and/or the matrices. One element or
one field is ignored if it isn't a floating-point number, a matrix, an array, or a structure. If the
expression value is an integer number, the expresion value is converted to a floating-point number and
then is negated.

The `not` operator converts the expression value to a boolean value and then negates the boolean
value.

The `'` operator transposes matrix.

### Expressions of binary operators

The syntax of expression of binary operator is:

    binary op expression = expression, "[", expression, "]"
                         | expression, "*", expression
                         | expression, ".*", expression
                         | expression, "/", expression
                         | expression, "./", expression
                         | expression, "+", expression
                         | expression, ".+", expression
                         | expression, "-", expression
                         | expression, ".-", expression
                         | expression, "<", expression
                         | expression, ">=", expression
                         | expression, ">", expression
                         | expression, "<=", expression
                         | expression, "==", expression
                         | expression, "!=", expression;

The `[]` operator is an index operator that allows to access to elements or fields. An indexing
for numbers begins from one. An expression created by this operator is assignable if the first
expression value is an array or a structure.

The `*` operator multiplies the number or matrix by the number or the matrix.

The `.*` operator multiplies the number or the elements of matrix by the number or the elements of
matrix.

The `/` operator divides the number or the matrix by the number.

The `.*` operator divides the number or the elements of matrix by the number or the elements of
matrix.

The `+` operator adds the number or matrix to the number or the matrix. Also, two strings, two arrays,
two structures can be added by this operator. If two fields in two structures have same a field
idendifier, the field from the first expression assigns to a field of result structure.

The `.+` operator adds the number or the elements of matrix by the number or the elements of matrix.

The `-` operator subtracts the number or matrix from the number or the matrix.

The `.-` operator subtracts the number or the elements of matrix from the number or the elements of
matrix.

The arithmetic binary operator without dot converts one value to a floating-point number and then
performs operation if one value is an integer number.

The arithmetic binary operator with dot recursively performs an operation on numbers and/or the
matrices. Two elements or two fields are compares with types if they aren't floating-point numbers,
matrices, arrays, or a structures. If two elements or two fields aren't equal, the error occurs. If
the expression value is an integer number, the expression value is converted to a floating-point
number and then there performs the operation.

The comparison operator except the `==` operator and the `!=` operator comperes the boolean value to
the boolean value, the number to the number, the string to the string.

The `==` operator comperes two values. The result of this operator is `true` if two values are equal,
otherwise `false`.

The `!=` operator comperes two values. The result of this operator is `false` if two values are equal,
otherwise `true`.

The `==` operator and the `!=` operator don't compare two matrices. The result of these operators is
`false` for the `==` operator or `true` for the `!=` operator if two values are matrices. These
operator doesn't compare value types for integer numbers and floating-point numbers.

### Expressions of logical operators

The syntax of expression of logical operator is:

    logical op expression = expression, "and", expression
                          | expression, "or", expression;

The `and` operator performs the logical-AND operation. The result of this operator is the second
expression value if the first expression value is `true` after conversion to the boolean value,
otherwise the first expression value.

The `or` operator performs the logical-OR operation. The result of this operator is the first
expression value if the first expression value is `true` after conversion to the boolean value,
otherwise the second expression value.

These operators evaluate the second expression if it is necessary.

### Expressions of field access

The syntax of expression of field access is:

    field access expression = expression, ".", identifier;

The expression of field access allows to access to the structure fields by the identifier. The
expression of field access is assignable.

### Range expressions

The syntax of range expression is:

    range expression = expression, "to", expression, ["by", expression];

The range expression creates a range. The expression values in the range expression must be numbers.
The range is a floating-point range if at least one expression value is floating-point number,
otherwise an integer range.

### Expressions of propagation error

The syntax of expression of propagation error is:

    propagation error expression = expression, "?";

The expression of error propagation allows to propagate error. If the expression value is the `none`
value or an error value, there leaves from a function with the result that is this value or prints the
error outside the function.

### Expression precedence

Expresions and operators with arities and priorities are:

| Expression or operator | Arity          | Priority |
| ---------------------- | -------------- | -------- |
| parenthesis            |                | 12       |
| literal                |                | 12       |
| name                   |                | 12       |
| function appliaction   |                | 11       |
| `[]`                   | binary         | 11       |
| `.`                    | binary         | 11       |
| `?`                    | unary          | 10       |
| `'`                    | unary          | 9        |
| `-`                    | unary          | 8        |
| `.-`                   | unary          | 8        |
| `not`                  | unary          | 8        |
| `*`                    | binary         | 7        |
| `.*`                   | binary         | 7        |
| `/`                    | binary         | 7        |
| `./`                   | binary         | 7        |
| `+`                    | binary         | 6        |
| `.+`                   | binary         | 6        |
| `-`                    | binary         | 6        |
| `.-`                   | binary         | 6        |
| `to` `by`              | binary/ternary | 5        |
| `<`                    | binary         | 4        |
| `>=`                   | binary         | 4        |
| `>`                    | binary         | 4        |
| `<=`                   | binary         | 4        |
| `==`                   | binary         | 3        |
| `!=`                   | binary         | 3        |
| `and`                  | binary         | 2        |
| `or`                   | binary         | 1        |

Associative of all expression and all operators is left to right.

## Literals

The syntax of literal is:

    literal = none literal
            | boolean literal
            | integer number literal
            | float number literal
            | string literal
            | matrix literal
            | array literal
            | structure literal;

### None literals

The syntax of none literal is:

    none literal = "none";

The none literal represents the none value.

### Boolean literals

The syntax of none literal is:

    none literal = "false"
                 | "true";

The boolean literal represents the boolean value that is `false` or `true`.

### Literals of interger numbers

The syntax of literal of integer number is:

    integer number literal = integer;

The literal of integer number represents the value of integer number.

### Literals of floating-point numbers

The syntax of literal of floating-point number is:

    float number literal = float
                         | "inf"
                         | "nan";

The literal of floating-point number represents the value of floating-point number. The literal of
floating number can also be infinity or NaN.

### String literal

The syntax of string literal is:

    string literal = string;

The string literal represents the string value.

### Matrix literal

The syntax of matrix literal is:

    matrix literal = "[", {newline},
                     (
                         matrix row, newline, {newline}, "fill", expression, {newline}
                       | matrix rows
                     ),
                     "]";
    matrix rows = {newline}, [matrix row, {newline, {newline}, matrix row}, [newline, {newline}]];
    matrix row = expression, "fill", expression
               | expressions;
    expressions = [expression, {",", expression}, [","]];

The matrix literal represents the matrix value. The matrix or the matrix row can be filled with the
filling matrix or the filling expression by using the `fill` keyword. The filling matrix row or the
filling expression is separately evaluated for each matrix row and each element. The expression after
the `fill` keyword specifies the number of rows for the filled matrix or the number of columns for the
filled matrix row.

### Array literal

The syntax of array literal is:

    array literal = ".[", {newline},
                    (
                        expression, "fill", expression
                      | expressions
                    ),
                    {newline}, ".]";
    expressions = [expression, {",", expression}, [","]];

The array literal represents the array value. The array can also be filled with the filling expression
by the `fill` keyword. The filling expresson is separately evaluated for each element. The expression
after the `fill` keyword specifies the number of elements.
