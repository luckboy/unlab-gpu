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
expression can be converted to the `true` value.

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
converted to the `true` value if the break statements doesn't occur.

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

    expresion = literal
              | name
              | function application
              | unary op expression
              | binary op expression
              | logical expression
              | field access
              | range expression
              | error propagation expression;

### Function application

The syntax of function application is:

    function application = expression, "(", expressions, ")";
    expressions = [expression, {",", expression}, [","]];

The function application applies the function expression to the arguments. If a value of function
expression isn't function, the error occurs.

### Expressions of unary operator

The syntax of expression of unary operator is:

    unary op expression = "-", expression
                        | ".-", expression
                        | "not", expression
                        | expression, "'";

The `-` operator negates the number or the matrix.

The `.-` operator recursively negates the loating-point numbers and/or the matrices. The element or
the field are ignored if it isn't a floating-point number, a matrix, an array, or a structure. If the
expression value is an integer number, the expresion value is converted to a floating-point number and
then is negated.

The `not` operator converts the expression value to a boolean value and then negates the boolean
value.

The `'` operator transposes matrix.

### Expressions of binary operator

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

The `[` `]` operator is an index operator that allows to access to an element or a field. An
expression created by this operator is assignable.

The `*` operator multiplies the number or matrix by the number or the matrix.

The `.*` operator multiplies the number or the elements of matrix by the number or the elements of
matrix.

The `/` operator divides the number or the matrix by the number.

The `.*` operator divides the number or the elements of matrix by the number or the elements of
matrix.

The `+` operator adds the number or matrix to the number or the matrix.

The `.+` operator adds the number or the elements of matrix by the number or the elements of matrix.

The `-` operator subtracts the number or matrix to the number or the matrix.

The `.-` operator subtracts the number or the elements of matrix by the number or the elements of
matrix.

The arithmetic binary operator with dot recursively performs an operation on numbers and/or the 
matrices. Two elements or two fields are compares with types if they aren't floating-point numbers,
matrices, arrays, or a structures. If the expression value is an integer number, the expression value
is converted to a floating-point number and then there performs the operation.
