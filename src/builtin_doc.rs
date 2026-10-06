//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! A module of documentation of built-in functions.
use crate::doc::*;
use crate::getopts_doc::*;
use crate::mod_node::*;
#[cfg(feature = "plot")]
use crate::plot_doc::*;

/// Adds the documentation of built-in functions to the root module of signature and the root
/// module of documentation.
pub fn add_std_builtin_fun_doc(sig_root_mod: &mut ModNode<Sig, ()>, doc_root_mod: &mut ModNode<String, Option<String>>)
{
    let doc = r#"
Standard library that is a basic library for the Unlab scripting language.

This library contains basic functions to operate on numbers, matrices, arrays, structures, and
other objects. Also, this library allows access to system operations by providing system
functions. Functions to running scripts, loading libraries, and testing also are provided by this
library.

# Values and objects

Value types are:

- none
- boolean
- integer number
- floating-point number
- reference to immutable object
- strong reference to mutable object
- weak reference to mutable object
- floating-point number box (since 0.2.0)
- immutable user object (since 0.2.0)
- mutable user object (since 0.2.0)

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
- barrier (since 0.2.0)
- mutex (since 0.2.0)
- monitor (since 0.2.0)
- reader-writer lock (since 0.2.0)
- channel (since 0.2.0)
- join handle (since 0.2.0)

Mutable object types are:

- array
- structure

Indexable object types are:

- string
- matrix array
- matrix row slice
- array
- structure

Iterable object types are:

- string
- integer range
- floating-point range
- matrix array
- matrix row slice
- array

# Mathematical functions

A mathematical function with one argument recursivaly performs an operation on floating-point
number and/or matrices. One element or one field is ignored if it isn't floating-point number,
matrix, or mutable object and one argument is a mutable object.

A mathematical function with two arguments recursively performs an operation on floating-point
number and/or matrices. Two elements or two fields are compares with types if they aren't 
floating-point numbers, matrices, or mutable object and two arguments are mutable objects. If two
elements or two fields aren't equal, an error occurs. One element or one field is ignored if it
isn't a floating-point number, a matrix, or a mutable object; one argument is a mutable object;
and other argument is a number.

# TOML format and JSON format

This library contains the following functions to converting from/to values, loading values, and
saving values for the [TOML](https://en.wikipedia.org/wiki/TOML) format and the
[JSON](https://en.wikipedia.org/wiki/JSON) format:

- [`str2toml`](#var.str2toml) (since 0.2.0)
- [`toml2str`](#var.toml2str) (since 0.2.0)
- [`str2json`](#var.str2json) (since 0.2.0)
- [`json2str`](#var.json2str) (since 0.2.0)
- [`loadtoml`](#var.loadtoml)
- [`savetoml`](#var.savetoml)
- [`loadjson`](#var.loadjson)
- [`savejson`](#var.savejson)

These functions to converting from/to, loading, and saving the following values:

- none
- boolean
- integer number
- floating-point number
- string
- array
- structure

# CSV format

This library has the following functions to converting from/to arrays, loading arrays, and saving
arrays for the [CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format:

- [`str2csv`](#var.str2csv) (since 0.2.0)
- [`str2csvwithouthdr`](#var.str2csvwithouthdr) (since 0.2.0)
- [`csv2str`](#var.csv2str) (since 0.2.0)
- [`csv2strwithouthdr`](#var.csv2strwithouthdr) (since 0.2.0)
- [`loadcsv`](#var.loadcsv) (since 0.2.0)
- [`loadcsvwithouthdr`](#var.loadcsvwithouthdr) (since 0.2.0)
- [`savecsv`](#var.savecsv) (since 0.2.0)
- [`savecsvwithouthdr`](#var.savecsvwithouthdr) (since 0.2.0)

These functions to converting from/to, loading, and saving the following values as record fields:

- boolean
- integer number
- floating-point number
- string

# Time format

The following specifiers can be used in the format by the [`strftime`](#var.strftime) function
and the  [`strptime`](#var.strptime) function:

- `%Y` - year with zero padding
- `%C` - century with zero padding
- `%y` - year in century with zero padding
- `%q` - quartar of year
- `%m` - month number with zero padding
- `%b` - abbreviated month name
- `%B` - full month name
- `%h` - same as `%b`
- `%d` - day of month with zero padding
- `%e` - day of month with spece padding
- `%a` - abbreviated weekday name
- `%A` - full weekday name
- `%w` - Sunday = 0, Monday = 1, ..., Saturday = 6
- `%u` - Monday = 1, Tuesday = 2, ..., Sunday = 7
- `%U` - week number starting with Sunday (00-53) with zero padding
- `%W` - week number starting with Monday (00-53) witn zero padding
- `%G` - year corresponding to the ISO week number with zero padding
- `%g` - year in century corresponding to the ISO week number with zero padding
- `%V` - ISO week number (01-53) with zero padding
- `%j` - day of year with zero padding
- `%D` - same as `%m/%d/%y`
- `%x` - locale's date
- `%F` - same as `%Y-%b-%d`
- `%v` - same as `%e-%b-%Y`
- `%H` - hour number (00-23) with zero padding
- `%k` - hour number ( 0-23) with space padding
- `%I` - hour number in 12-hour clocks (01-12) with zero padding
- `%l` - hour number in 12-hour clocks ( 1-12) with space padding
- `%P` - `am` or `pm` in 12-hour clocks
- `%p` - `AM` or `PM` in 12-hour clocks
- `%M` - minute number with zero padding
- `%S` - second number with zero padding
- `%f` - number of nanoseconds since of last second
- `%.f` - decimal faction of second with dot
- `%.3f` - decimal faction of second with dot and 3 digits
- `%.6f` - decimal faction of second with dot and 6 digits
- `%.9f` - decimal faction of second with dot and 9 digits
- `%3f` - decimal faction of second without dot with 3 digits
- `%6f` - decimal faction of second without dot with 6 digits
- `%9f` - decimal faction of second without dot with 9 digits
- `%R` - same as `%H:%M`
- `%T` - same as `%H:%M:%S`
- `%X` - locale's time
- `%r` - locale's 12-hour closks time
- `%Z` - local tiem zone name
- `%z` - offset from the local time to UTC
- `%:z` - offset from the local time to UTC with colon
- `%::z` - offset from the local time to UTC with seconds
- `%:::z` - offset from the local time to UTC without minutes
- `%#z` - offset from the local time to UTC with minutes or without minutes (parsing only)
- `%c` - locale's date and time
- `%+` - date and time in ISO 8601/RFC 3339 format
- `%s` - UNIX timestamp
- `%t` - literal tab
- `%n` - literal newline
- `%%` - literal percent sign

The above specifiers can have the following modifiers:

- `-` - specifier without padding
- `_` - specifier with space padding
- `0` - specifier with zero padding
"#;
    match doc_root_mod.value() {
        Some(prev_doc) => doc_root_mod.set_value(Some(prev_doc.clone() + "\n" + &doc[1..])),
        None => doc_root_mod.set_value(Some(String::from(&doc[1..]))),
    }
    
    let doc = r#"
A $\pi$ number.
"#;
    sig_root_mod.add_var(String::from("pi"), Sig::VarSinceFirst);
    doc_root_mod.add_var(String::from("pi"), String::from(&doc[1..]));

    let doc = r#"
An $e$ number.
"#;
    sig_root_mod.add_var(String::from("e"), Sig::VarSinceFirst);
    doc_root_mod.add_var(String::from("e"), String::from(&doc[1..]));

    let doc = r#"
A machine epsilon number.
"#;
    sig_root_mod.add_var(String::from("eps"), Sig::VarSinceFirst);
    doc_root_mod.add_var(String::from("eps"), String::from(&doc[1..]));

    let doc = r#"
A path separator that can be `"/"` for Unix or `"\\"` for Windows.
"#;
    sig_root_mod.add_var(String::from("pathsep"), Sig::VarSinceFirst);
    doc_root_mod.add_var(String::from("pathsep"), String::from(&doc[1..]));
    
    let doc = r#"
Returns a string corresponding to the type of the `X` value.

The strings corresponding to the value types and the object types are:

- `"none"` - none value
- `"bool"` - boolean value
- `"int"` - integer number
- `"float"` - floating-point number
- `"string"` - string
- `"intrange"` - integer range
- `"floatrange"` - floating-point range
- `"matrix"` - matrix
- `"function"` - function
- `"matrixarray"` matrix array
- `"matrixrowslice"` - matrix row slice
- `"error"` - error
- `"windowid"` - window identifier
- `"barrier"` - barrier (since 0.2.0)
- `"mutex"` - mutex (since 0.2.0)
- `"monitor"` - monitor (since 0.2.0)
- `"rwlock"` - reader-writer lock (since 0.2.0)
- `"channel"` - channel (since 0.2.0)
- `"joinhandle"` - join handle (since 0.2.0)
- `"array"` - array
- `"struct"` - structure
- `"weak"` - weak reference
- `"floatbox"` - floating-point number box (since 0.2.0)
- `"userobject"` - immutable user object (since 0.2.0)
- `"mutuserobject"` - mutable user object (since 0.2.0)
"#;
    sig_root_mod.add_var(String::from("type"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("type"), String::from(&doc[1..]));

    let doc = r#"
Returns a copy of the `X` object.

If the `X` object isn't a mutable object, this function returns the `X` object.
"#;
    sig_root_mod.add_var(String::from("clone"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("clone"), String::from(&doc[1..]));
        
    let doc = r#"
Converts the `X` value to a boolean value.

This function returns `true` if the `X` value isn't `none`, `false`, zero, or an error; otherwise
`false`.
"#;
    sig_root_mod.add_var(String::from("bool"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("bool"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `X` value to an integer number.

The `X` number is converted to an integer number by this function. This function returns `1` for a
non-numeric value if the `X` value isn't `none`, `false`, or an error; otherwise `0`.
"#;
    sig_root_mod.add_var(String::from("int"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("int"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `X` value to a float-point number.

The `X` number is converted to a float-point number by this function. This function returns `1.0`
for a non-numeric value if the `X` value isn't `none`, `false`, or an error; otherwise `0.0`.
"#;
    sig_root_mod.add_var(String::from("float"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("float"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `X` value to a string.
"#;
    sig_root_mod.add_var(String::from("string"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("string"), String::from(&doc[1..]));
    let doc = r#"
Returns a matrix with zeros that has the `N` number of rows and the `M` number of columns.

The returned matrix is:

$$ \begin{bmatrix} 0 & 0 & \ldots & 0 \\ 0 & 0 & \ldots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \ldots & 0 \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("zeros"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("N")),
        BuiltinFunArg::Arg(String::from("M"))
    ]));
    doc_root_mod.add_var(String::from("zeros"), String::from(&doc[1..]));
    
    let doc = r#"
Returns a matrix with ones that has the `N` number of rows and the `M` number of columns.

The returned matrix is:

$$ \begin{bmatrix} 1 & 1 & \ldots & 1 \\ 1 & 1 & \ldots & 1 \\ \vdots & \vdots & \ddots & \vdots \\ 1 & 1 & \ldots & 1 \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("ones"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("N")),
        BuiltinFunArg::Arg(String::from("M"))
    ]));
    doc_root_mod.add_var(String::from("ones"), String::from(&doc[1..]));
    
    let doc = r#"
Returns an identity matrix that has the `N` number of rows and columns.

The identity matrix is:

$$ \begin{bmatrix} 1 & 0 & \ldots & 0 \\ 0 & 1 & \ldots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \ldots & 1 \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("eye"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("N"))
    ]));
    doc_root_mod.add_var(String::from("eye"), String::from(&doc[1..]));
    let doc = r#"
Returns an initialized matrix that has the `N` number of rows and the `M` number of columns.

This function applies the `f` function to the `D` value and the element indices
($f(\mathbf{D}, i, j)$) for each element of initialized matrix. The initialized matrix is:

$$ \begin{bmatrix} f(\mathbf{D}, 1, 1) & f(\mathbf{D}, 1, 2) & \ldots & f(\mathbf{D}, 1, M) \\ f(\mathbf{D}, 2, 1) & f(\mathbf{D}, 2, 2) & \ldots & f(\mathbf{D}, 2, M) \\ \vdots & \vdots & \ddots & \vdots \\ f(\mathbf{D}, N, 1) & f(\mathbf{D}, N, 2) & \ldots & f(\mathbf{D}, N, M) \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("init"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("N")),
        BuiltinFunArg::Arg(String::from("M")),
        BuiltinFunArg::Arg(String::from("D")),
        BuiltinFunArg::Arg(String::from("f"))
    ]));
    doc_root_mod.add_var(String::from("init"), String::from(&doc[1..]));
    
    let doc = r#"
Returns an initialized diagonal matrix that has the `N` number of rows and columns.

This function applies the `f` function to the `D` value and the element index
($f(\mathbf{D}, i)$) for each element of main diagonal of initialized diagonal matrix. The
initialized diagonal matrix is:

$$ \begin{bmatrix} f(\mathbf{D}, 1) & 0 & \ldots & 0 \\ 0  & f(\mathbf{D}, 2) & \ldots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \ldots & f(\mathbf{D}, N) \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("initdiag"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("N")),
        BuiltinFunArg::Arg(String::from("D")),
        BuiltinFunArg::Arg(String::from("f"))
    ]));
    doc_root_mod.add_var(String::from("initdiag"), String::from(&doc[1..]));
    
    let doc = r#"
Creates a matrix from the `X` iterable object that contains the iterable objects which contains
the numbers.

If the `X` object is a matrix, this function returns the `X` object. The created matrix is:

$$ \begin{bmatrix} x_{1 1} & x_{1 2} & \ldots & x_{1M} \\ x_{2 1} & x_{2 2} & \ldots & x_{2M} \\ \vdots & \vdots & \ddots & \vdots \\ x_{N1} & x_{N2} & \ldots & x_{NM} \end{bmatrix} $$

"#;
    sig_root_mod.add_var(String::from("matrix"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("matrix"), String::from(&doc[1..]));
    
    let doc = r#"
Creates a matrix with one row from the `x` iterable object that contains the numbers.

If the `x` object is a matrix with one row, this function returns the `x` object. The created matrix
with one row is:

$$ \begin{bmatrix} x_1 & x_2 & \ldots & x_N \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("rowvector"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("rowvector"), String::from(&doc[1..]));
    
    let doc = r#"
Creates a matrix with one column vector from the `x` iterable object that contains the numbers.

If the `x` object is a matrix with one column, this function returns the `x` object. The created
matrix with one column is:

$$ \begin{bmatrix} x_1 \\ x_2 \\ \vdots \\ x_N \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("colvector"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("colvector"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `X` matrix to a matrix array.

If the `X` object is a matrix array, this function returns the `X` object.
"#;
    sig_root_mod.add_var(String::from("matrixarray"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("matrixarray"), String::from(&doc[1..]));
    
    let doc = r#"
Creates an error with the `kind` error kind and the `msg` message which are strings.
"#;
    sig_root_mod.add_var(String::from("error"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("kind")),
        BuiltinFunArg::Arg(String::from("msg"))
    ]));
    doc_root_mod.add_var(String::from("error"), String::from(&doc[1..]));

    let doc = r#"
Creates an array from the `X` iterable object.

If the `X` value is an array, this function returns the `X` value.
"#;
    sig_root_mod.add_var(String::from("array"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("array"), String::from(&doc[1..]));

    let doc = r#"
Converts the `R` reference to the strong reference.

If the `R` reference is strong, this function returns the `R` reference. 
"#;
    sig_root_mod.add_var(String::from("strong"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("R"))
    ]));
    doc_root_mod.add_var(String::from("strong"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `R` reference to the weak reference.

If the `R` reference is weak, this function returns the `R` reference. 
"#;
    sig_root_mod.add_var(String::from("weak"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("R"))
    ]));
    doc_root_mod.add_var(String::from("weak"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` object is empty, otherwise `false`.

The `X` object can be a string, a matrix array, a matrix row slice, or an array. 
"#;
    sig_root_mod.add_var(String::from("isempty"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("isempty"), String::from(&doc[1..]));

    let doc = r#"
Returns the number of elements in the `X` object.

The `X` object can be a string, a matrix array, a matrix row slice, or an array. This function
returns the number of UTF-8 characters for a string, the number of rows for a matrix array, or the
number of columns for a matrix row slice. 
"#;
    sig_root_mod.add_var(String::from("length"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("length"), String::from(&doc[1..]));

    let doc = r#"
Returns the number of rows in the `X` object.

The `X` object can be a matrix or a matrix array.
"#;
    sig_root_mod.add_var(String::from("rows"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("rows"), String::from(&doc[1..]));

    let doc = r#"
Returns the number of columns in the `X` object.

The `X` object can be a matrix or a matrix array.
"#;
    sig_root_mod.add_var(String::from("columns"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("columns"), String::from(&doc[1..]));

    let doc = r#"
Returns the element with one index or two indices in the `X` indexable object if the `X`
indexable object contains the element, otherwise `none`.

If the `j` index is passed and the `X` value is a matrix array, this function returns the element
with the `i` row index and the `j`  column index in the `X` matrix array. This function returns
the string with one UTF-8 character for a string, the matrix row slice for a matrix array, or the
element of matrix for a matrix row slice if the `j` index isn't passed. The field with the `i` 
identifier in the `X` structure is returned if the `j` index isn't passed and the `X` object is
structure.
"#;
    sig_root_mod.add_var(String::from("get"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("i")),
        BuiltinFunArg::OptArg(String::from("j"))
    ]));
    doc_root_mod.add_var(String::from("get"), String::from(&doc[1..]));

    let doc = r#"
Returns the element with the `i` index in the diagonal of the `X` matrix array if the diagonal of
`X` matrix array contains the element, otherwise `none`.
"#;
    sig_root_mod.add_var(String::from("getdiag"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("i"))
    ]));
    doc_root_mod.add_var(String::from("getdiag"), String::from(&doc[1..]));

    let doc = r#"
Returns the substrings of the `s` string which are separated by the `t` string.

If the `t` string isn't passed, this function uses whitespaces as a separator.
"#;
    sig_root_mod.add_var(String::from("split"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::OptArg(String::from("t"))
    ]));
    doc_root_mod.add_var(String::from("split"), String::from(&doc[1..]));

    let doc = r#"
Returns the `s` string without the start whitespaces and the end whitespaces.
"#;
    sig_root_mod.add_var(String::from("trim"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("trim"), String::from(&doc[1..]));

    let doc = r#"
Returns the `true` if the `s` string contains the `t` string, otherwise `false`.
"#;
    sig_root_mod.add_var(String::from("contains"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t"))
    ]));
    doc_root_mod.add_var(String::from("contains"), String::from(&doc[1..]));

    let doc = r#"
Returns the `true` if the `t` is the prefix of the `s` string, otherwise `false`.
"#;
    sig_root_mod.add_var(String::from("startswith"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t"))
    ]));
    doc_root_mod.add_var(String::from("startswith"), String::from(&doc[1..]));

    let doc = r#"
Returns the `true` if the `t` is the suffix of the `s` string, otherwise `false`.
"#;
    sig_root_mod.add_var(String::from("endswith"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t"))
    ]));
    doc_root_mod.add_var(String::from("endswith"), String::from(&doc[1..]));

    let doc = r#"
Replaces all occurrences of the `t` string in the `s` string with the `u` string.

This function returns a new string with replaced occurrences of the `t` string to the `u` string.
"#;
    sig_root_mod.add_var(String::from("replace"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t")),
        BuiltinFunArg::Arg(String::from("u"))
    ]));
    doc_root_mod.add_var(String::from("replace"), String::from(&doc[1..]));

    let doc = r#"
Returns an uppercase string corresponding the `s` string.
"#;
    sig_root_mod.add_var(String::from("upper"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("upper"), String::from(&doc[1..]));

    let doc = r#"
Returns a lowercase string corresponding the `s` string.
"#;
    sig_root_mod.add_var(String::from("lower"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("lower"), String::from(&doc[1..]));

    let doc = r#"
Sorts boolean values, numbers, or strings in the `x` array.

This function uses ascending sort order to sorting. Each element in the `x` array must have same
sorting value type that can be the boolean type, the number type, or the string type. If two or
more elements in the `x` array have the different sorting value types, an error occurs. The
integer numbers and the the floating-point numbers have same sorting value type. An error occurs
if any element in the `x` array is `nan`.
"#;
    sig_root_mod.add_var(String::from("sort"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("sort"), String::from(&doc[1..]));

    let doc = r#"
Reverses the order of elements in the `x` array.
"#;
    sig_root_mod.add_var(String::from("reverse"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("reverse"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `f` function with the passed `D` value returns a convertible value to `true`
for any element in the `X` iterable object ($f(\mathbf{D}, {\mathbf{x}}_i)$), otherwise `false`.
"#;
    sig_root_mod.add_var(String::from("any"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("D")),
        BuiltinFunArg::Arg(String::from("f"))
    ]));
    doc_root_mod.add_var(String::from("any"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `f` function with the passed `D` value returns a convertible value to `true`
for all elements in the `X` iterable object ($f(\mathbf{D}, {\mathbf{x}}_i)$), otherwise
`false`.
"#;
    sig_root_mod.add_var(String::from("all"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("D")),
        BuiltinFunArg::Arg(String::from("f"))
    ]));
    doc_root_mod.add_var(String::from("all"), String::from(&doc[1..]));

    let doc = r#"
Finds the element in the `X` iterable object.

This function applies the `f` function to the `D` value and each element in the `X` iterable
object ($f(\mathbf{D}, {\mathbf{x}}_i)$) until the `f` function returns a convertible value to 
`true` and then returns the index of this element. If the `f` function doesn't return the 
convertible value to `true` for any element, this function returns `none`.
"#;
    sig_root_mod.add_var(String::from("find"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("D")),
        BuiltinFunArg::Arg(String::from("f"))
    ]));
    doc_root_mod.add_var(String::from("find"), String::from(&doc[1..]));

    let doc = r#"
Filters the elements in the `X` iterable object.

This function applies the `f` function to the `D` value and each element in the `X` iterable
object ($f(\mathbf{D}, {\mathbf{x}}_i)$) and then returns the indices of elements for which the
`f` function returns a convertible value to `true`.
"#;
    sig_root_mod.add_var(String::from("filter"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("D")),
        BuiltinFunArg::Arg(String::from("f"))
    ]));
    doc_root_mod.add_var(String::from("filter"), String::from(&doc[1..]));

    let doc = r#"
Finds maximum element in the `X` iterable object or maximum value between the `X` value and the
`Y` value ($\max(x, y)$, $\max(x_{ij}, y)$, $\max(x, y_{ij})$, or $\max(x_{ij}, y_{ij})$).

This function with two arguments is a mathematical function that takes two arguments. This
argument can be a number, a matrix, or a mutable object. These arguments can't be a matrix and a
mutable object. If the `X` value and the `Y` value are integer numbers, this function also
returns an integer number. This function returns `none` if the `X` iterable object is empty and
the `Y` value isn't passed.
"#;
    sig_root_mod.add_var(String::from("max"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::OptArg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("max"), String::from(&doc[1..]));

    let doc = r#"
Finds minimum element in the `X` iterable object or minimum value between the `X` value and the
`Y` value ($\min(x, y)$, $\min(x_{ij}, y)$, $\min(x, y_{ij})$, or $\min(x_{ij}, y_{ij})$).

This function with two arguments is a mathematical function that takes two arguments. This
argument can be a number, a matrix, or a mutable object. These arguments can't be a matrix and a
mutable object. If the `X` value and the `Y` value are integer numbers, this function also
returns an integer number. This function returns `none` if the `X` iterable object is empty and
the `Y` value isn't passed.
"#;
    sig_root_mod.add_var(String::from("min"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::OptArg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("min"), String::from(&doc[1..]));

    let doc = r#"
Finds maximum element in the `X` iterable object and returns its index.

This function returns `none` if the `X` iterable object is empty.
"#;
    sig_root_mod.add_var(String::from("imax"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("imax"), String::from(&doc[1..]));

    let doc = r#"
Finds minumum element in the `X` iterable object and returns its index.

This function returns `none` if the `X` iterable object is empty.
"#;
    sig_root_mod.add_var(String::from("imin"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("imin"), String::from(&doc[1..]));

    let doc = r#"
Pushes the `y` value to the back of the`X` array.
"#;
    sig_root_mod.add_var(String::from("push"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("y"))
    ]));
    doc_root_mod.add_var(String::from("push"), String::from(&doc[1..]));

    let doc = r#"
Removes the last element from the `X` array and returns the last element.

If the `X` array is empty, this function returns `none`.
"#;
    sig_root_mod.add_var(String::from("pop"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("pop"), String::from(&doc[1..]));

    let doc = r#"
Appends the `Y` mutable object to the `X` mutable object.

The `X` mutable object and the `Y` mutable object must be arrays or structures. If two fields in
two structures have same field identifier, the field in the first structure is overwritten by a
value from the field in the second structure.
"#;
    sig_root_mod.add_var(String::from("append"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("append"), String::from(&doc[1..]));

    let doc = r#"
Inserts the `y` value to the `X` mutable object.

If the `X` mutable object is an array, this function inserts the `y` value as an element with the
`i` index to the `X` array, moves all elements after the inserted element to right, and returns
`none`.  If the `X` mutable object is a structure, this function inserts the `X` value as a field
with the `i` identifier to the `X` structure and then returns the replaced field. This function
returns `none` if the `X` structure doesn't contain the field with the `i` identifier. 
"#;
    sig_root_mod.add_var(String::from("insert"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("i")),
        BuiltinFunArg::Arg(String::from("y"))
    ]));
    doc_root_mod.add_var(String::from("insert"), String::from(&doc[1..]));

    let doc = r#"
Removes the element from the `X` mutable object.

If the `X` mutable object is an array, this function removes an element with the `i` index from
the `X` array and moves all elements after the removed element to left. If the `X` mutable object
is a structure, this function removes a field with the `i` identifier from the `X` structure.
This finction returns the removed element or the removed field if the `X` mutable object contains
the element with the `i` index or the field with the `i` identifier, otherwise `none`.
"#;
    sig_root_mod.add_var(String::from("remove"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("i"))
    ]));
    doc_root_mod.add_var(String::from("remove"), String::from(&doc[1..]));

    let doc = r#"
Returns the error kind for the `e` error.
"#;
    sig_root_mod.add_var(String::from("errorkind"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("e"))
    ]));
    doc_root_mod.add_var(String::from("errorkind"), String::from(&doc[1..]));

    let doc = r#"
Returns the error message for the `e` error.
"#;
    sig_root_mod.add_var(String::from("errormsg"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("e"))
    ]));
    doc_root_mod.add_var(String::from("errormsg"), String::from(&doc[1..]));
    
    let doc = r#"
Returns `true` if the `X` value is equal to the `Y` value, otherwise `false`.

This function doesn't compare matrices. The result of this function is `false` if two values are
matrices. This function doesn't compare value types for integer numbers and floating-point
numbers.
"#;
    sig_root_mod.add_var(String::from("isequal"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("isequal"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` value isn't equal to the `Y` value, otherwise `false`.

This function doesn't compare matrices. The result of this function is `true` if two values are
matrices. This function doesn't compare value types for integer numbers and floating-point
numbers.
"#;
    sig_root_mod.add_var(String::from("isnotequal"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("isnotequal"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` value is less than the `Y` value, otherwise `false`.

This function compares two boolean values, two numbers, or two strings. The result of this
function is `false` for two other values.
"#;
    sig_root_mod.add_var(String::from("isless"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("isless"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` value is greater than or equal to the `Y` value, otherwise `false`.

This function compares two boolean values, two numbers, or two strings. The result of this
function is `false` for two other values.
"#;
    sig_root_mod.add_var(String::from("isgreaterequal"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("isgreaterequal"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` value is greater than the `Y` value, otherwise `false`.

This function compares two boolean values, two numbers, or two strings. The result of this
function is `false` for two other values.
"#;
    sig_root_mod.add_var(String::from("isgreater"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("isgreater"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` value is less than or equal to the `Y` value, otherwise `false`.

This function compares two boolean values, two numbers, or two strings. The result of this
function is `false` for two other values.
"#;
    sig_root_mod.add_var(String::from("islessequal"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("islessequal"), String::from(&doc[1..]));

    let doc = r#"
Calculates sigmoid function for the `X` value ($\operatorname{sigmoid}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("sigmoid"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("sigmoid"), String::from(&doc[1..]));

    let doc = r#"
Calculates hyperbolic tangent for the `X` value ($\tanh(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("tanh"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("tanh"), String::from(&doc[1..]));

    let doc = r#"
Calculates swish function for the `X` value ($\operatorname{swish}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("swish"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("swish"), String::from(&doc[1..]));

    let doc = r#"
Calculates softmax function for the `X` value ($\operatorname{softmax}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("softmax"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("softmax"), String::from(&doc[1..]));

    let doc = r#"
Calculates square root of the `X` value ($\sqrt{x}$ or $\sqrt{x_{ij}}$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("sqrt"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("sqrt"), String::from(&doc[1..]));

    let doc = r#"
Indeed transposes the `X` matrix (${\mathbf{X}}^\top$).
"#;
    sig_root_mod.add_var(String::from("reallytranspose"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("reallytranspose"), String::from(&doc[1..]));

    let doc = r#"
This function is alias to the [`reallytranspose`](#var.reallytranspose) function.
"#;
    sig_root_mod.add_var(String::from("rt"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("rt"), String::from(&doc[1..]));

    let doc = r#"
Repeats the `x` vector as column or row.
"#;
    sig_root_mod.add_var(String::from("repeat"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("repeat"), String::from(&doc[1..]));

    let doc = r#"
Calculates remainder of division the `x` value by the `y` value ($\operatorname{mod}(x, y)$).

If the `x` value and the `y` value are integer numbers, this function also returns an integer
number.
"#;
    sig_root_mod.add_var(String::from("mod"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("x")),
        BuiltinFunArg::Arg(String::from("y"))
    ]));
    doc_root_mod.add_var(String::from("mod"), String::from(&doc[1..]));
    
    let doc = r#"
Calculates absolute value of the `X` value ($|x|$ or $|x_{ij}|$).

This function is a mathematical function that takes a number, a matrix, or a mutable object. If
the `X` value is an integer number, this function also returns an integer value.
"#;
    sig_root_mod.add_var(String::from("abs"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("abs"), String::from(&doc[1..]));

    let doc = r#"
Raises the `X` value to the power of the `Y` value ($x^y$, ${x_{ij}}^y$, $x^{y_{ij}}$, or
${x_{ij}}^{y_{ij}}$).

This function is a mathematical function that takes two arguments. This argument can be a number,
a matrix, or a mutable object. These arguments can't be a matrix and a mutable object.
"#;
    sig_root_mod.add_var(String::from("pow"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("pow"), String::from(&doc[1..]));

    let doc = r#"
Calculates exponentional function of the `X` value ($e^x$ or $e^{x_{ij}}$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("exp"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("exp"), String::from(&doc[1..]));

    let doc = r#"
Calculates natural logarithm of the `X` value ($\ln{x}$ or $\ln{x_{ij}}$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("log"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("log"), String::from(&doc[1..]));

    let doc = r#"
Calculates base 2 logarithm of the `X` value ($\log_2{x}$ or $\log_2{x_{ij}}$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("log2"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("log2"), String::from(&doc[1..]));

    let doc = r#"
Calculates base 10 logarithm of the `X` value ($\log_10{x}$ or $\log_10{x_{ij}}$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("log10"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("log10"), String::from(&doc[1..]));

    let doc = r#"
Calculates sine function for the `X` value ($\sin(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("sin"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("sin"), String::from(&doc[1..]));

    let doc = r#"
Calculates cosine function for the `X` value ($\cos(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("cos"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("cos"), String::from(&doc[1..]));

    let doc = r#"
Calculates tangent function for the `X` value ($\tan(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("tan"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("tan"), String::from(&doc[1..]));

    let doc = r#"
Calculates arcsine function for the `X` value ($\arcsin(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("asin"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("asin"), String::from(&doc[1..]));

    let doc = r#"
Calculates arccosine function for the `X` value ($\arccos(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("acos"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("acos"), String::from(&doc[1..]));

    let doc = r#"
Calculates arctangent function for the `X` value ($\arctan(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("atan"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("atan"), String::from(&doc[1..]));

    let doc = r#"
Calculates arctangent function for the `X` value and the `Y` value ($\arctan(\frac{x}{y})$,
$\arctan(\frac{x_{ij}}{y})$, $\arctan(\frac{x}{y_{ij}})$, or $\arctan(\frac{x_{ij}}{y_{ij}})$).

This function is a mathematical function that takes two arguments. This argument can be a number,
a matrix, or a mutable object. These arguments can't be a matrix and a mutable object.
"#;
    sig_root_mod.add_var(String::from("atan2"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("atan2"), String::from(&doc[1..]));

    let doc = r#"
Calculates hyperbolic sine function for the `X` value ($\sinh(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("sinh"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("sinh"), String::from(&doc[1..]));

    let doc = r#"
Calculates hyperbolic cosine function for the `X` value ($\cosh(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("cosh"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("cosh"), String::from(&doc[1..]));

    let doc = r#"
Calculates inverse hyperbolic sine function for the `X` value
($\operatorname{arsinh}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("asinh"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("asinh"), String::from(&doc[1..]));

    let doc = r#"
Calculates inverse hyperbolic cosine function for the `X` value
($\operatorname{arcosh}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("acosh"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("acosh"), String::from(&doc[1..]));

    let doc = r#"
Calculates inverse hyperbolic tangent function for the `X` value
($\operatorname{artanh}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("atanh"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("atanh"), String::from(&doc[1..]));

    let doc = r#"
Calculates signum function for the `X` value ($\operatorname{sgn}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("sign"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("sign"), String::from(&doc[1..]));

    let doc = r#"
Calculates ceil function for the `X` value ($\operatorname{ceil}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("ceil"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("ceil"), String::from(&doc[1..]));

    let doc = r#"
Calculates floor function for the `X` value ($\operatorname{floor}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("floor"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("floor"), String::from(&doc[1..]));

    let doc = r#"
Calculates round function for the `X` value ($\operatorname{round}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("round"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("round"), String::from(&doc[1..]));

    let doc = r#"
Calculates trunc function for the `X` value ($\operatorname{trunc}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("trunc"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("trunc"), String::from(&doc[1..]));

    let doc = r#"
Generates a random floating-point number in range $[0, 1)$.
"#;
    sig_root_mod.add_var(String::from("rand"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("rand"), String::from(&doc[1..]));

    let doc = r#"
Generates a random integer number in range $[1, N]$ or range $[N, M]$.
"#;
    sig_root_mod.add_var(String::from("randi"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("N")),
        BuiltinFunArg::OptArg(String::from("M"))
    ]));
    doc_root_mod.add_var(String::from("randi"), String::from(&doc[1..]));

    let doc = r#"
Converts the `s` string to an integer number.

If the `s` string can be converted to the integer number, this function returns an error with the
`"parseint"` error kind.
"#;
    sig_root_mod.add_var(String::from("str2int"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("str2int"), String::from(&doc[1..]));

    let doc = r#"
Converts the `s` string to a floating-point number.

If the `s` string can be converted to the floating-point number, this function returns an error
with the `"parsefloat"` error kind.
"#;
    sig_root_mod.add_var(String::from("str2float"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("str2float"), String::from(&doc[1..]));    
    
    let doc = r#"
Converts the hexadecimal number as the `s` string to a decimal integer number.

If the `s` string can be converted to the decimal integer number, this function returns an error 
with the error kind `"parseint"`.
"#;
    sig_root_mod.add_var(String::from("hex2dec"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("hex2dec"), String::from(&doc[1..]));

    let doc = r#"
Converts the first character of the `s` string to a character code as an integer number.

If the `s` string is empty, this function returns `none`.
"#;
    sig_root_mod.add_var(String::from("char2code"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("char2code"), String::from(&doc[1..]));

    let doc = r#"
Converts the character code as the `x` integer number to the string with the character.

If the `x` character code is invalid, this function returns `none`.
"#;
    sig_root_mod.add_var(String::from("code2char"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("code2char"), String::from(&doc[1..]));

    let doc = r#"
Formats the `millis` number of milliseconds according to the `fmt` format.

The formats with examples are:

- `"s"` - seconds for exmaple `"1234.567s"`
- `"ms"` - minutes and seconds for example `"12m34.567s"`
- `"hms"` - hours, minutes, and seconds for example `"12h34m56.789s"`

If the `fmt` format is invalid, this function returns an error with the `"format"` error kind.
"#;
    sig_root_mod.add_var(String::from("formatmillis"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("fmt")),
        BuiltinFunArg::Arg(String::from("millis"))
    ]));
    doc_root_mod.add_var(String::from("formatmillis"), String::from(&doc[1..]));

    let doc = r#"
Formats the `X` value according to the `width` width and the `align` alignment.

If the string of the `X` value has a number of characters less than the `width` width, the string
of `X` value is padded with spaces according to the `align` alignment. The alignments are:

- `"left"` - left alignment
- `"center"` - center alignment
- `"right"` - right alignment

If the `align` alignment isn't passed, this function uses the left alignment for the string of
the `X` value by default. If the `width` width and/or the `align` alignment are/is invalid, this 
function returns an error with the `"format"` error kind.
"#;
    sig_root_mod.add_var(String::from("withwidth"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("width")),
        BuiltinFunArg::OptArg(String::from("align"))
    ]));
    doc_root_mod.add_var(String::from("withwidth"), String::from(&doc[1..]));

    let doc = r#"
Formats the `X` value with the zero padding according to the `width` width.

If the string of the `X` value has a number of characters less than the `width` width, the string
of `X` value is padded with zeros according to the right alignment. If the `width` width is 
invalid, this function returns an error with the `"format"` error kind.
"#;
    sig_root_mod.add_var(String::from("withzeros"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("width"))
    ]));
    doc_root_mod.add_var(String::from("withzeros"), String::from(&doc[1..]));

    let doc = r#"
Reads a line from the standard input.

If an I/O error occurs while this operation, this function returns an error with the `"io"` error 
kind.
"#;
    sig_root_mod.add_var(String::from("readline"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("readline"), String::from(&doc[1..]));

    let doc = r#"
Formats the values and then returns the formatted values as a string.
"#;
    sig_root_mod.add_var(String::from("format"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("format"), String::from(&doc[1..]));

    let doc = r#"
Prints the values to the standard output.
"#;
    sig_root_mod.add_var(String::from("print"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("print"), String::from(&doc[1..]));

    let doc = r#"
Prints the values with the newline character to the standard output.
"#;
    sig_root_mod.add_var(String::from("println"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("println"), String::from(&doc[1..]));

    let doc = r#"
Prints the values to the standard error.
"#;
    sig_root_mod.add_var(String::from("eprint"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("eprint"), String::from(&doc[1..]));

    let doc = r#"
Prints the values with the newline character to the standard error.
"#;
    sig_root_mod.add_var(String::from("eprintln"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("eprintln"), String::from(&doc[1..]));

    let doc = r#"
Flushes the stream of standard output.

This function writes all unwritten buffered data in the stream of standard output to the standard 
output. Also, this function returns `true` if an I/O error doesn't occur while this operation,
otherwise an error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("flush"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("flush"), String::from(&doc[1..]));

    let doc = r#"
Flushes the stream of standard error.

This function writes all unwritten buffered data in the stream of standard error to the standard
error. Also, this function returns `true` if an I/O error doesn't occur while this operation,
otherwise an error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("eflush"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("eflush"), String::from(&doc[1..]));

    let doc = r#"
Changes the current working directory to the `path` directory.

This function returns `true` if an I/O error doesn't occur while this operation, otherwise an
error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("cd"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("cd"), String::from(&doc[1..]));

    let doc = r#"
Returns the path of current working directory.

If an I/O error occur while this operation, this function returns an error with the `"io"` error
kind.
"#;
    sig_root_mod.add_var(String::from("pwd"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("pwd"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `path` file exists, otherwise `false`.

If an I/O error occur while this operation, this function returns an error with the `"io"` error
kind.
"#;
    sig_root_mod.add_var(String::from("exist"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("exist"), String::from(&doc[1..]));

    let doc = r#"
Returns a file type as a string for the `path` file.

The file types are:

- `"dir"` - directory
- `"file"` - any file except directory

If an I/O error occur while this operation, this function returns an error with the `"io"` error
kind.
"#;
    sig_root_mod.add_var(String::from("filetype"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("filetype"), String::from(&doc[1..]));

    let doc = r#"
Returns file names in the `path` directory.

If an I/O error occur while this operation, this function returns an error with the `"io"` error
kind.
"#;
    sig_root_mod.add_var(String::from("dir"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("dir"), String::from(&doc[1..]));
    
    let doc = r#"
This function is alias to the [`dir`](#var.dir) function.
"#;
    sig_root_mod.add_var(String::from("ls"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("ls"), String::from(&doc[1..]));

    let doc = r#"
Creates a `path` directory.

This function returns `true` if an I/O error doesn't occur while this operation, otherwise an
error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("mkdir"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("mkdir"), String::from(&doc[1..]));

    let doc = r#"
Removes the `path` directory.

This function returns `true` if an I/O error doesn't occur while this operation, otherwise an
error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("rmdir"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("rmdir"), String::from(&doc[1..]));

    let doc = r#"
Removes the `path` file.

This function returns `true` if an I/O error doesn't occur while this operation, otherwise an
error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("rmfile"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("rmfile"), String::from(&doc[1..]));

    let doc = r#"
Copies the content of the `srcpath` file with the permissions to the `dstpath` file.

This function overwrites the content of the `dstpath` file if the `dstpath` file already exists.
Also, this function returns `true` if an I/O error doesn't occur while this operation, otherwise
an error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("copy"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("srcpath")),
        BuiltinFunArg::Arg(String::from("dstpath"))
    ]));
    doc_root_mod.add_var(String::from("copy"), String::from(&doc[1..]));

    let doc = r#"
Renames the `oldpath` file to the `newpath` name.

This function replaces the `newpath` file if the `newpath` file already exists. Also, this
function returns `true` if an I/O error doesn't occur while this operation, otherwise an error
with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("rename"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("oldpath")),
        BuiltinFunArg::Arg(String::from("newpath"))
    ]));
    doc_root_mod.add_var(String::from("rename"), String::from(&doc[1..]));

    let doc = r#"
Executes the command with the `cmdname` command name and the arguments as a child process.

This function returns the exit code if an I/O error doesn't occur while this operation, otherwise
an error with the `"io"` error kind. Also, this function returns an error with the `"exitstatus"`
error kind if child process terminated by signal.
"#;
    sig_root_mod.add_var(String::from("spawn"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("cmdname")),
        BuiltinFunArg::OptArg(String::from("arg")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("spawn"), String::from(&doc[1..]));

    let doc = r#"
Terminates the current process with the `exitcode` exit code.
"#;
    sig_root_mod.add_var(String::from("exit"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("exitcode"))
    ]));
    doc_root_mod.add_var(String::from("exit"), String::from(&doc[1..]));

    let doc = r#"
Loads values from the `path` file in the binary format.

This function returns the loaded values if an I/O error doesn't occur while this operation, 
otherwise an error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("load"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("load"), String::from(&doc[1..]));

    let doc = r#"
Saves the values to the `path` file in the binary format.

This function returns `true` if an I/O error doesn't occur while this operation, otherwise an
error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("save"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path")),
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("save"), String::from(&doc[1..]));

    let doc = r#"
Loads a string from the `path` text file.

This function returns the loaded string if an I/O error doesn't occur while this operation,
otherwise an error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("loadstr"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("loadstr"), String::from(&doc[1..]));

    let doc = r#"
Saves the `s` string to the `path` text file.

This function returns `true` if an I/O error doesn't occur while this operation, otherwise an
error with the `"io"` error kind.
"#;
    sig_root_mod.add_var(String::from("savestr"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path")),
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("savestr"), String::from(&doc[1..]));

    let doc = r#"
Loads a value from the `path` file in the [TOML](https://en.wikipedia.org/wiki/TOML) format.

This function returns the loaded value if an error doesn't occur while this operation, otherwise
an error with the `"io"` error kind or the `"toml"` error kind.
"#;
    sig_root_mod.add_var(String::from("loadtoml"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("loadtoml"), String::from(&doc[1..]));

    let doc = r#"
Saves the `X` value to the `path` file in the [TOML](https://en.wikipedia.org/wiki/TOML) format.

This function returns `true` if an error doesn't occur while this operation, otherwise an error
with the `"io"` error kind or the `"toml"` error kind.
"#;
    sig_root_mod.add_var(String::from("savetoml"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path")),
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("savetoml"), String::from(&doc[1..]));

    let doc = r#"
Loads a value from the `path` file in the [JSON](https://en.wikipedia.org/wiki/JSON) format.

This function returns the loaded value if an error doesn't occur while this operation, otherwise
an error with the `"io"` error kind or the `"json"` error kind.
"#;
    sig_root_mod.add_var(String::from("loadjson"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("loadjson"), String::from(&doc[1..]));

    let doc = r#"
Saves the `X` value to the `path` file in the [JSON](https://en.wikipedia.org/wiki/JSON) format.

This function returns `true` if an error doesn't occur while this operation, otherwise an error
with the `"io"` error kind or the `"json"` error kind.
"#;
    sig_root_mod.add_var(String::from("savejson"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path")),
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("savejson"), String::from(&doc[1..]));

    let doc = r#"
Returns the arguments which are passed for this script.
"#;
    sig_root_mod.add_var(String::from("args"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("args"), String::from(&doc[1..]));

    let doc = r#"
Returns the environment variables of current process as strings.
"#;
    sig_root_mod.add_var(String::from("env"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("env"), String::from(&doc[1..]));

    let doc = r#"
Returns the path to the script directory.
"#;
    sig_root_mod.add_var(String::from("scriptdir"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("scriptdir"), String::from(&doc[1..]));

    let doc = r#"
Returns the library paths as the string.
"#;
    sig_root_mod.add_var(String::from("libpath"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("libpath"), String::from(&doc[1..]));

    let doc = r#"
Returns the domain of current library if interpreter is in the libary, otherwise `None`.
"#;
    sig_root_mod.add_var(String::from("domain"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("domain"), String::from(&doc[1..]));

    let doc = r#"
Loads the library with the `libname` library name if the library isn't already loaded, otherwise
this function doesn't load the library.

The library with the `libname` library name is loaded in the root module. The `libname` library
name should contain the domain and the name which are separeted by the `/` character. If the
`libname` library name hasn't the domain and the interpreter is in the library, this function
loads the library with the domain of current library.
"#;
    sig_root_mod.add_var(String::from("uselib"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("libname"))
    ]));
    doc_root_mod.add_var(String::from("uselib"), String::from(&doc[1..]));

    let doc = r#"
Loads the library with the `libname` library name even if the library is already loaded.

The library with the `libname` library name is loaded in the root module. The `libname` library
name should contain the domain and the name which are separeted by the `/` character. If the
`libname` library name hasn't the domain and the interpreter is in the library, this function
loads the library with the domain of current library.
"#;
    sig_root_mod.add_var(String::from("reuselib"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("libname"))
    ]));
    doc_root_mod.add_var(String::from("reuselib"), String::from(&doc[1..]));

    let doc = r#"
Runs the script that is refers by the `path` path.

If the `path` path is relative, the script is runned from the script directory. The script is
runned in the current module. The `/` path separators can be used in the `path` path regardless
of the operating system because  the `/` path separators are replaced to the system path
separators.
"#;
    sig_root_mod.add_var(String::from("run"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("run"), String::from(&doc[1..]));

    let doc = r#"
This function is alias to the [`run`](#var.run) function.

This alias adds the documetation comments from the `path` file while documentation generation. 
The `path` path should be a string literal so that documentation comment are added.
"#;
    sig_root_mod.add_var(String::from("runwithdoc"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("path"))
    ]));
    doc_root_mod.add_var(String::from("runwithdoc"), String::from(&doc[1..]));

    let doc = r#"
Returns the elapsed time in milliseconds since an interpreter start.
"#;
    sig_root_mod.add_var(String::from("clock"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("clock"), String::from(&doc[1..]));

    let doc = r#"
Imports the module with the `modname` name in the current module.

If the `newident` identifier is passed, the module is imported as module with the `newident`
identifier. The `modname` name should contain the module identifiers which are separated by the
`::` character sequence. The first module identifier in the `modname` name can be the `root`
keyword that refers to the root module. The `modname` name can have the `::` prefix.
"#;
    sig_root_mod.add_var(String::from("usemod"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("modname")),
        BuiltinFunArg::OptArg(String::from("newident"))
    ]));
    doc_root_mod.add_var(String::from("usemod"), String::from(&doc[1..]));

    let doc = r#"
Imports all modules from the module with the `modname` name in the current module.

The `modname` name should contain the module identifiers which are separated by the `::` 
character sequence.  The first module identifier in the `modname` name can be the `root` keyword
that refers to the root module. The `modname` name can have the `::` prefix.
"#;
    sig_root_mod.add_var(String::from("usemods"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("modname"))
    ]));
    doc_root_mod.add_var(String::from("usemods"), String::from(&doc[1..]));

    let doc = r#"
Imports the module with the `varname` name in the current module.

If the `newident` identifier is passed, the variable is imported as module with the `newident`
identifier. The `varname` name should contain the module identifiers and/or the variable 
identifier which are separated by the `::` character sequence. The first module identifier in the
`varname` name can be the `root` keyword that refers to the root module. The `varname` name can
have the `::` prefix.
"#;
    sig_root_mod.add_var(String::from("usevar"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("varname")),
        BuiltinFunArg::OptArg(String::from("newident"))
    ]));
    doc_root_mod.add_var(String::from("usevar"), String::from(&doc[1..]));

    let doc = r#"
Imports all variables from the module with the `modname` name in the current module.

The `modname` name should contain the module identifiers which are separated by the `::` 
character sequence.  The first module identifier in the `modname` name can be the `root`
keyword that refers to the root module. The `modname` name can have the `::` prefix.
"#;
    sig_root_mod.add_var(String::from("usevars"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("modname"))
    ]));
    doc_root_mod.add_var(String::from("usevars"), String::from(&doc[1..]));

    let doc = r#"
Removes the module import with the `ident` identifier from the current module.
"#;
    sig_root_mod.add_var(String::from("removeusemod"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("ident"))
    ]));
    doc_root_mod.add_var(String::from("removeusemod"), String::from(&doc[1..]));

    let doc = r#"
Removes the variable import with the `ident` identifier from the current module.
"#;
    sig_root_mod.add_var(String::from("removeusevar"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("ident"))
    ]));
    doc_root_mod.add_var(String::from("removeusevar"), String::from(&doc[1..]));

    let doc = r#"
Removes the module with the `ident` identifier from the current module.
"#;
    sig_root_mod.add_var(String::from("removemod"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("ident"))
    ]));
    doc_root_mod.add_var(String::from("removemod"), String::from(&doc[1..]));

    let doc = r#"
Removes the variable with the `ident` identifier from the current module.
"#;
    sig_root_mod.add_var(String::from("removevar"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("ident"))
    ]));
    doc_root_mod.add_var(String::from("removevar"), String::from(&doc[1..]));

    let doc = r#"
Removes the local variable with the `ident` identifier.
"#;
    sig_root_mod.add_var(String::from("removelocalvar"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("ident"))
    ]));
    doc_root_mod.add_var(String::from("removelocalvar"), String::from(&doc[1..]));

    let doc = r#"
An interruption error occurs if an interruption is occurred.
"#;
    sig_root_mod.add_var(String::from("checkintr"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("checkintr"), String::from(&doc[1..]));

    let doc = r#"
Returns the backend name as a string.
"#;
    sig_root_mod.add_var(String::from("backend"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("backend"), String::from(&doc[1..]));

    let doc = r#"
Returns the Unlab-gpu version.
"#;
    sig_root_mod.add_var(String::from("version"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("version"), String::from(&doc[1..]));

    let doc = r#"
An version error if the Unlab-gpu version isn't matched to the `s` version requirement.
"#;
    sig_root_mod.add_var(String::from("reqver"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("reqver"), String::from(&doc[1..]));

    let doc = r#"
Returns the documentation paths as the string.
"#;
    sig_root_mod.add_var(String::from("docpath"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("docpath"), String::from(&doc[1..]));

    let doc = r#"
Opens the documentation.

If the `libname` library name isn't passed, this function opens this documentation. The `libname`
library name should contain the domain and the name which are separeted by the `/` character. 
"#;
    sig_root_mod.add_var(String::from("doc"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::OptArg(String::from("libname"))
    ]));
    doc_root_mod.add_var(String::from("doc"), String::from(&doc[1..]));

    let doc = r#"
This function is alias to the [`doc`](#var.doc) function.
"#;
    sig_root_mod.add_var(String::from("help"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::OptArg(String::from("libname"))
    ]));
    doc_root_mod.add_var(String::from("help"), String::from(&doc[1..]));

    let doc = r#"
Asserts that the `V` value is `true`.

An assertion error occurs if the `V` value isn't `true`. If next arguments are passed, an
assertion error has the message that is the formatted next arguments.
"#;
    sig_root_mod.add_var(String::from("assert"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("V")),
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("assert"), String::from(&doc[1..]));

    let doc = r#"
Asserts that the `L` value is equal to the `R` value.

An assertion error occurs if the `L` value isn't equal to the `R` value. If next arguments are
passed, an assertion error has the message that is the formatted next arguments.
"#;
    sig_root_mod.add_var(String::from("asserteq"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("L")),
        BuiltinFunArg::Arg(String::from("R")),
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("asserteq"), String::from(&doc[1..]));

    let doc = r#"
Asserts that the `L` value isn't equal to the `R` value.

An assertion error occurs if the `L` value is equal to the `R` value. If next arguments are
passed, an assertion error has the message that is the formatted next arguments.
"#;
    sig_root_mod.add_var(String::from("assertne"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("L")),
        BuiltinFunArg::Arg(String::from("R")),
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("assertne"), String::from(&doc[1..]));

    let doc = r#"
Asserts that the difference between the `L` value and the `R` value is greater than the `eps`
value.

An assertion error occurs if the difference between the `L` value and the `R` value isn't greater
than the `eps` value. If next arguments are passed, an assertion error has the message that is
the formatted next arguments. The differences are recursively checked like comparison without
types for matrix arrays, matrix row slices, arrays, or structures. The difference between two
elements or two fields are checked if two elements or two fields are numbers, otherwise this
function compares two element or two fields.
"#;
    sig_root_mod.add_var(String::from("assertnearlyeq"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("L")),
        BuiltinFunArg::Arg(String::from("R")),
        BuiltinFunArg::Arg(String::from("eps")),
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("assertnearlyeq"), String::from(&doc[1..]));

    let doc = r#"
Asserts that the difference between the `L` value and the `R` value isn't greater than the `eps`
value.

An assertion error occurs if the difference between the `L` value and the `R` value is greater
than the `eps` value. If next arguments are passed, an assertion error has the message that is
the formatted next arguments. The differences are recursively checked like comparison without
types for matrix arrays, matrix row slices, arrays, or structures. The difference between two
elements or two fields are checked if two elements or two fields are numbers, otherwise this
function compares two element or two fields.
"#;
    sig_root_mod.add_var(String::from("assertnearlyne"), Sig::BuiltinFunSinceFirst(vec![
        BuiltinFunArg::Arg(String::from("L")),
        BuiltinFunArg::Arg(String::from("R")),
        BuiltinFunArg::Arg(String::from("eps")),
        BuiltinFunArg::OptArg(String::from("X")),
        BuiltinFunArg::DotDotDot
    ]));
    doc_root_mod.add_var(String::from("assertnearlyne"), String::from(&doc[1..]));

    let doc = r#"
Adds the current module to the test suites.
"#;
    sig_root_mod.add_var(String::from("tests"), Sig::BuiltinFunSinceFirst(vec![]));
    doc_root_mod.add_var(String::from("tests"), String::from(&doc[1..]));

    //
    // Documentation of built-in functions since version 0.2.0.
    //

    let doc = r#"
Returns a string corresponding to the function kind of the `f` value if the `f` value is a
function, otherwise `none`.

The strings corresponding to the function kinds:

- `"named"` - named function
- `"builtin"` - built-in function
- `"unnamed"` - unnamed function
- `"user"` - user function
"#;
    sig_root_mod.add_var(String::from("functionkind"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("f"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("functionkind"), String::from(&doc[1..]));

    let doc = r#"
Converts the `X` value to a float-point number and creates a floating-point number box with the
converted `X` value.

The `X` number is converted to a float-point number by this function. This function converts the
`X` value to `1.0` for a non-numeric value if the `X` value isn't `none`, `false`, or an error;
otherwise `0.0`.
"#;
    sig_root_mod.add_var(String::from("floatbox"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("floatbox"), String::from(&doc[1..]));
    
    let doc = r#"
Returns the number of bytes in the `s` string.
"#;
    sig_root_mod.add_var(String::from("bytes"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("bytes"), String::from(&doc[1..]));
    
    let doc = r#"
Returns the substrings of the `s` string which are separated by the `t` regular expression.
"#;
    sig_root_mod.add_var(String::from("splitre"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("splitre"), String::from(&doc[1..]));

    let doc = r#"
Returns the `true` if the `s` string contains one or more matches of the `t` regular expression,
otherwise `false`.
"#;
    sig_root_mod.add_var(String::from("containsre"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("containsre"), String::from(&doc[1..]));    
    
    let doc = r#"
Replaces all matches of the `t` regular expression in the `s` string with the `u` a regular
string.

This function returns a new string with replaced matches of the `t` regular expression to the `u`
regular string. The regular string can have references to groups in the regular expression.
"#;
    sig_root_mod.add_var(String::from("replacere"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t")),
        BuiltinFunArg::Arg(String::from("u"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("replacere"), String::from(&doc[1..]));

    let doc = r#"
Folds the elements in the `X` iterable object.

This function applies the `f` function with the `D` value and the `z` value for the first
application or the previous result of the `f` function for each element in the `X` iterable
object
($f(\mathbf{D}, f(\ldots f(\mathbf{D}, f(\mathbf{D}, \mathbf{z}, {\mathbf{x}}_1), {\mathbf{x}}_2) \ldots), {\mathbf{x}}_N)$).
A result of this function is a last result of the `f` function if the `X` iterable object isn't
empty, otherwise the `z` value.
"#;
    sig_root_mod.add_var(String::from("fold"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("z")),
        BuiltinFunArg::Arg(String::from("D")),
        BuiltinFunArg::Arg(String::from("f"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("fold"), String::from(&doc[1..]));

    let doc = r#"
Maps the elements in the `X` iterable object.

This function creates a new array with the mapped elements by the `f` function with the `D` value
($f(\mathbf{D}, {\mathbf{x}}_i)$).
"#;
    sig_root_mod.add_var(String::from("map"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("D")),
        BuiltinFunArg::Arg(String::from("f"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("map"), String::from(&doc[1..]));    

    let doc = r#"
Reduce the elements in the `X` iterable object.

This function applies the `f` function with the `D` value and the first element in the `X`
iterable object for the first application or the previous result of the `f` function for each
element except the first element in the `X` iterable object
($f(\mathbf{D}, f(\ldots f(\mathbf{D}, f(\mathbf{D}, {\mathbf{x}}_1, {\mathbf{x}}_2), {\mathbf{x}}_3) \ldots), {\mathbf{x}}_N)$).
A result of this function is a last result of the `f` function for more elements than one or one
element for one element if the `X` iterable object isn't empty, otherwise `none`.
"#;
    sig_root_mod.add_var(String::from("reduce"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("D")),
        BuiltinFunArg::Arg(String::from("f"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("reduce"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `s` string with the content in the [TOML](https://en.wikipedia.org/wiki/TOML) format
to a value.

This function returns the value if an error doesn't occur while this conversion, otherwise an
error with the `"toml"` error kind.
"#;
    sig_root_mod.add_var(String::from("str2toml"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("str2toml"), String::from(&doc[1..]));

    let doc = r#"
Converts the `X` value to a string with a content in the
[TOML](https://en.wikipedia.org/wiki/TOML) format.

This function returns the string if an error doesn't occur while this conversion, otherwise an
error with the `"toml"` error kind.
"#;
    sig_root_mod.add_var(String::from("toml2str"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("toml2str"), String::from(&doc[1..]));

    let doc = r#"
Converts the `s` string with the content in the [JSON](https://en.wikipedia.org/wiki/JSON) format
to a value.

This function returns the value if an error doesn't occur while this conversion, otherwise an
error with the `"json"` error kind.
"#;
    sig_root_mod.add_var(String::from("str2json"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("str2json"), String::from(&doc[1..]));

    let doc = r#"
Converts the `X` value to a string with a content in the
[JSON](https://en.wikipedia.org/wiki/JSON) format.

This function returns the string if an error doesn't occur while this conversion, otherwise an
error with the `"json"` error kind.
"#;
    sig_root_mod.add_var(String::from("json2str"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("json2str"), String::from(&doc[1..]));    

    let doc = r#"
Converts the `s` string with the content in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format to an array.

The array has elements which are structures with fields. The structure contains the field
identifiers which are the header in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format. If the `issemicolon` boolean
is `true`, this function uses semicolon as the field separator instead comma. This function
returns the array if an error doesn't occur while this conversion, otherwise an error with the
`"csv"` error kind. 
"#;
    sig_root_mod.add_var(String::from("str2csv"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::OptArg(String::from("issemicolon")),
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("str2csv"), String::from(&doc[1..]));    

    let doc = r#"
Converts the `s` string with the content without the header in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format to an array.

The array has elements which are arrays with elements as record fields. If the `issemicolon`
boolean is `true`, this function uses semicolon as the field separator instead comma. This
function returns the array if an error doesn't occur while this conversion, otherwise an error
with the `"csv"` error kind. 
"#;
    sig_root_mod.add_var(String::from("str2csvwithouthdr"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::OptArg(String::from("issemicolon"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("str2csvwithouthdr"), String::from(&doc[1..]));    
    
    let doc = r#"
Converts the `X` array to a string with a content in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format.

The array has elements which are structures with fields. The structure contains the field
identifiers which are the header in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format. This function also requires
the `fieldnames` array that contains the field names which specify which record fields are
written. If some structure field doesn't exist in the array element, the record field is empty.
If the `issemicolon` boolean is `true`, this function uses semicolon as the field separator
instead comma. This function returns the string if an error doesn't occur while this conversion,
otherwise an error with the `"csv"` error kind. 
"#;
    sig_root_mod.add_var(String::from("csv2str"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("fieldnames")),
        BuiltinFunArg::OptArg(String::from("issemicolon"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("csv2str"), String::from(&doc[1..]));    
    
    let doc = r#"
Converts the `X` array to a string with a content without the header in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format.

The array has elements which are arrays with elements as record fields. This function also
requires the `fields` number that determines how many record the fields are written. If the
`issemicolon` boolean is `true`, this function uses semicolon as the field separator instead
comma. This function returns the string if an error doesn't occur while this conversion,
otherwise an error with the `"csv"` error kind. 
"#;
    sig_root_mod.add_var(String::from("csv2strwithouthdr"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("fields")),
        BuiltinFunArg::OptArg(String::from("issemicolon"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("csv2strwithouthdr"), String::from(&doc[1..]));    

    let doc = r#"
Formats the `millis` number of milliseconds as the UNIX timestamp according the `fmt` format.

If the `isutc` is `true`, this function uses the UTC time zone instead the local time zone. This
function returns a string if the `millis` number of milliseconds isn't too small or too large,
otherwise an error with the `"datetime"` error kind.
"#;
    sig_root_mod.add_var(String::from("strftime"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("fmt")),
        BuiltinFunArg::Arg(String::from("millis")),
        BuiltinFunArg::OptArg(String::from("isutc"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("strftime"), String::from(&doc[1..]));

    let doc = r#"
Converts the `s` string to the UNIX timestamp in milliseconds according the `fmt` format.

This function returns an integer number if a parsing error occurs for a time, otherwise an error
with the `"parsetime"` error kind.
"#;
    sig_root_mod.add_var(String::from("strptime"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("fmt"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("strptime"), String::from(&doc[1..]));    
    
    let doc = r#"
Loads an array from the `path` file in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format.

The array has elements which are structures with fields. The structure contains the field
identifiers which are the header in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format. If the `issemicolon` boolean
is `true`, this function uses semicolon as the field separator instead comma. This function
returns the array if an error doesn't occur while this operation, otherwise an error with the
`"io"` error kind or the `"csv"` error kind. 
"#;
    sig_root_mod.add_var(String::from("loadcsv"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("path")),
        BuiltinFunArg::OptArg(String::from("issemicolon"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("loadcsv"), String::from(&doc[1..]));

    let doc = r#"
Loads an array from the `path` file in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format.

The array has elements which are arrays with elements as record fields. If the `issemicolon`
boolean is `true`, this function uses semicolon as the field separator instead comma. This
function returns the array if an error doesn't occur while this operation, otherwise an error
with the `"io"` error kind or the `"csv"` error kind. 
"#;
    sig_root_mod.add_var(String::from("loadcsvwithouthdr"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("path")),
        BuiltinFunArg::OptArg(String::from("issemicolon"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("loadcsvwithouthdr"), String::from(&doc[1..]));    
    
    let doc = r#"
Saves the `X` array to the `path` file in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format.

The array has elements which are structures with fields. The structure contains the field
identifiers which are the header in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format. This function also requires
the `fieldnames` array that contains the field names which specify which record fields are
written. If some structure field doesn't exist in the array element, the record field is empty.
If the `issemicolon` boolean is `true`, this function uses semicolon as the field separator
instead comma. This function returns the string if an error doesn't occur while this operation,
otherwise an error with the `"io"` error kind or the `"csv"` error kind. 
"#;
    sig_root_mod.add_var(String::from("savecsv"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("path")),
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("fieldnames")),
        BuiltinFunArg::OptArg(String::from("issemicolon"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("savecsv"), String::from(&doc[1..]));

    let doc = r#"
Saves the `X` array to the `path` file in the
[CSV](https://en.wikipedia.org/wiki/Comma-separated_values) format.

The array has elements which are arrays with elements as record fields. This function also
requires the `fields` number that determines how many record the fields are written. If the
`issemicolon` boolean is `true`, this function uses semicolon as the field separator instead
comma. This function returns the string if an error doesn't occur while this operation,
otherwise an error with the `"io"` error kind or the `"csv"` error kind. 
"#;
    sig_root_mod.add_var(String::from("savecsvwithouthdr"), Sig::BuiltinFunSince(vec![
        BuiltinFunArg::Arg(String::from("path")),
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("fields")),
        BuiltinFunArg::OptArg(String::from("issemicolon"))
    ], String::from("0.2.0")));
    doc_root_mod.add_var(String::from("savecsvwithouthdr"), String::from(&doc[1..]));
    
    //
    // Documentation of built-in functions from other modules.
    //

    add_getopts_doc(sig_root_mod, doc_root_mod);
    #[cfg(feature = "plot")]
    add_plot_doc(sig_root_mod, doc_root_mod);
}
