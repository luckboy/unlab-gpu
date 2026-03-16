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
character. Whitespace characters can be separators between tokens.

### Newline

Newline token is the `CRLF` character sequence, the `LF` character, or the `;` character. The lexical
definition of newline token is:

    newline = (cr, lf) | lf | ";";
    cr = ?CR character?;
    lf = ?LF character?;

### Comments

This scripting language only occurs the single line comments. The single line comments begin from
the `#` character or the `%` character and contains any characters without the LF character. The
lexical definition of comments is:

    comment = ('#' | '%), {?any character except LF character?};

### Punctuation

Punctuation tokens are operators and/or separators. The lexical definition of punctuation tokens is:

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
floating-point numbers. Integer numbers can be decimal system or hexadecimal system. The lexical
definitions of number literals is:

    number = integer | float;
    integer = "0", ("X" | "x"), hex digit, {hex digit}
            | digit, {digit};
    float = digit, {digit}, ".", digit, {digit}, [exponent]
          | digit, {digit}, [exponent];
    exponent = ("E" | "e"), [("+" | "-")], digit, {digit};
    hex digit = digit
              | "A" | "B" | "C" | "D" | "E" | "F"
              | "a" | "b" | "c" | "d" | "e" | "f";
    digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

### String literals

String literals are texts which are closed by the `"` characters. The lexical definitions of string
literals is:

    string = '"', {string char | escape}, '"';
    string char = ?any character except '"' character, '\' character, and LF character?;
    escape = ascii escape | unicode escape;
    ascii escape = "\", oct digit, oct digit, oct digit
                 | "\", oct digit, oct digit
                 | "\", oct digit
                 | "\a" | "\b" | "\t" | "\n" | "\v" | "\f" | "\r"
                 | "\", string char;
    escape char = ?any character except LF character?;
    unicode escape = "\U", hex digit, hex digit, hex digit, hex digit, hex digit, hex digit
                   | "\u", hex digit, hex digit, hex digit, hex digit;
    oct digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7";
    hex digit = digit
              | "A" | "B" | "C" | "D" | "E" | "F"
              | "a" | "b" | "c" | "d" | "e" | "f";
    digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

### Idendifiers

Identifiers begin from the `_` character or the UTF-8 character with the Alphabetic property. Other
character of identifiers can be the `_` character or the UTF-8 character that has the Alphabetic
property, the Nd category, the Nl category, or the No category. The lexical definitions of
identifiers literals is:

    ident = first ident char, {ident char};
    first ident char = "_" | alphabetic char;
    ident char = "_" | alphabetic char | numeric char;
    alphabetic char = ?character with Alphabetic property?;
    numeric char = ?character with Nd category, Nl category, or No category?;
