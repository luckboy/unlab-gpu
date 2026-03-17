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

### Whitespaces

Whitespace character is the UTF-8 character that has the White_Space property except newline
character. The whitespace character sequence can be separators between tokens.

### Newline

A newline token is the `CRLF` character sequence, the `LF` character, or the `;` character. The
lexical definition of newline token is:

    newline = (cr, lf) | lf | ";";
    cr = ?CR character?;
    lf = ?LF character?;

### Comments

This scripting language only occurs the single line comments. The single line comment begins from
the `#` character or the `%` character and contains any characters without the LF character. The
lexical definition of comment is:

    comment = ("#" | "%"), {?any character except LF character?};

### Punctuation

A punctuation token is operator or separator. The lexical definition of punctuation tokens is:

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

### Keywords

Keywords are reserved words which aren't identifiers. The lexical definitions of keywords is:

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

### Number literals

Number literals are used to directly represent number values which can be integer numbers or
floating-point numbers. The integer numbers can be decimal system or hexadecimal system. The lexical
definitions of number literal is:

    number = integer | float;
    integer = "0", ("X" | "x"), hex digit, {hex digit}
            | digit, {digit};
    float = digit, {digit}, ".", digit, {digit}, [exponent]
          | digit, {digit}, exponent;
    exponent = ("E" | "e"), [("+" | "-")], digit, {digit};
    hex digit = digit
              | "A" | "B" | "C" | "D" | "E" | "F"
              | "a" | "b" | "c" | "d" | "e" | "f";
    digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

### String literals

A string literal is a text that is closed by the `"` characters. The lexical definitions of string
literal is:

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
    hex digit = digit
              | "A" | "B" | "C" | "D" | "E" | "F"
              | "a" | "b" | "c" | "d" | "e" | "f";
    digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

### Idendifiers

An identifier begins from the `_` character or the UTF-8 character with the Alphabetic property. A
next character of identifier can be the `_` character or the UTF-8 character that has the Alphabetic
property, the Nd category, the Nl category, or the No category. The lexical definitions of
identifiers literals is:

    identifier = first identifier char, {identifier char};
    first identifier char = "_" | alphabetic char;
    identifier char = "_" | alphabetic char | numeric char;
    alphabetic char = ?character with Alphabetic property?;
    numeric char = ?character with Nd category, Nl category, or No category?;

## Nodes

A syntax tree contains nodes which can be definitions and/or statements. The syntax of nodes is:

    nodes = {neline}, [node, {newline, {newline}, node}, [newline, {newline}]]
    node = definition | statement;

## Definitions

A definition is a module definition or a function definition. The syntax of definition is:

    definition = module definition
               | function definition;

### Module definition

A module definition has identifier and contains the nodes which also can be the modules. The syntax of
module definition is:

    module definition = "module", identifier, newline,
                        nodes,
                        "end";

The module with same identifier can only be defined once in other module. The modules uses own
namespace that is separate from the variable namespace.

### Function definition

A function definition has identifier. Arguments and statements can be in function definition. The
syntax of function definition is:

    function definition = "function", identifier, "(", arguments, ")", newline,
                          statements,
                          "end";
    arguments = [argument, {",", argument}, [","]];
    argument = ident

The function is a variable which can be apply to the arguments. If some variable with identifier is
defined in the module, the function with same identifier can't be defined in the module. The function
arguments are checked by an interpreter whether they are repeated.

## Statements

A statement can be for example an expression, an assignment, or a loop. The syntax of statements is:

    statements = {newline}, [statement, {newline, {newline}, statement}, [newline, {newline}]]
    statement = if statement
              | for statement
              | while statement
              | break statement
              | continue statement
              | return statement
              | quit statement
              | assignment statement
              | expression;

### If statement

An if statement contains conditional expressions and conditional statements. The syntax of if
statement is:

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

                    
