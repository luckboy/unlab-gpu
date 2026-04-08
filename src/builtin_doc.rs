//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::doc::*;
use crate::mod_node::*;

pub fn add_std_builtin_fun_doc(sig_root_mod: &mut ModNode<Sig, ()>, doc_root_mod: &mut ModNode<String, Option<String>>)
{
    let doc = r#"
Returns a string corresponding to the type of the `X` value.

The stings corresponding to the value types or the object types are:

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
- `"array"` - array
- `"struct"` - structure
- `"weak"` - weak reference
"#;
    sig_root_mod.add_var(String::from("type"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("type"), String::from(&doc[1..]));

    let doc = r#"
Returns a copy of the `X` object.

If the `X` object isn't a mutable object, this function returns the `X` object.
"#;
    sig_root_mod.add_var(String::from("bool"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("bool"), String::from(&doc[1..]));
    
    
    let doc = r#"
Converts the `X` value to a boolean value.

This function returns `true` if the `X` value isn't `none`, `false`, zero, or an error; otherwise
`false`.
"#;
    sig_root_mod.add_var(String::from("bool"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("bool"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `X` value to an integer number.

The `X` number is converted to an integer number by this function. This function returns `1` for a
non-numeric value if the `X` value isn't `none`, `false`, or an error; otherwise `0`.
"#;
    sig_root_mod.add_var(String::from("int"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("int"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `X` value to a float-point number.

The `X` number is converted to a float-point number by this function. This function returns `1.0`
for a non-numeric value if the `X` value isn't `none`, `false`, or an error; otherwise `0.0`.
"#;
    sig_root_mod.add_var(String::from("float"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("float"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `X` value to a string.
"#;
    sig_root_mod.add_var(String::from("string"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("string"), String::from(&doc[1..]));
    let doc = r#"
Returns a matrix with zeros that has the `N` number of rows and the `M` number of columns.

The returned matrix is:

$$ \begin{bmatrix} 0 & 0 & \cdots & 0 \\ 0 & 0 & \cdots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \cdots & 0 \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("zeros"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("N")),
        BuiltinFunArg::Arg(String::from("M"))
    ]));
    doc_root_mod.add_var(String::from("zeros"), String::from(&doc[1..]));
    
    let doc = r#"
Returns a matrix with ones that has the `N` number of rows and the `M` number of columns.

The returned matrix is:

$$ \begin{bmatrix} 1 & 1 & \cdots & 1 \\ 1 & 1 & \cdots & 1 \\ \vdots & \vdots & \ddots & \vdots \\ 1 & 1 & \cdots & 1 \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("ones"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("N")),
        BuiltinFunArg::Arg(String::from("M"))
    ]));
    doc_root_mod.add_var(String::from("ones"), String::from(&doc[1..]));
    
    let doc = r#"
Returns an identity matrix that has the `N` number of rows and columns.

The identity matrix is:

$$ \begin{bmatrix} 1 & 0 & \cdots & 0 \\ 0 & 1 & \cdots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \cdots & 1 \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("eye"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("N"))
    ]));
    doc_root_mod.add_var(String::from("eye"), String::from(&doc[1..]));
    let doc = r#"
Returns an initialized matrix that has the `N` number of rows and the `M` number of columns.

This function applies the `f` function to the `D` value and the element indices
($f(\mathbf{D}, i, j)$) for each element of initialized matrix. The initialized matrix is:

$$ \begin{bmatrix} f(\mathbf{D}, 1, 1) & f(\mathbf{D}, 1, 2) & \cdots & f(\mathbf{D}, 1, M) \\ f(\mathbf{D}, 2, 1) & f(\mathbf{D}, 2, 2) & \cdots & f(\mathbf{D}, 2, M) \\ \vdots & \vdots & \ddots & \vdots \\ f(\mathbf{D}, N, 1) & f(\mathbf{D}, N, 2) & \cdots & f(\mathbf{D}, N, M) \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("init"), Sig::BuiltinFun(vec![
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

$$ \begin{bmatrix} f(\mathbf{D}, 1) & 0 & \cdots & 0 \\ 0  & f(\mathbf{D}, 2) & \cdots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \cdots & f(\mathbf{D}, N) \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("initdiag"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("N")),
        BuiltinFunArg::Arg(String::from("D")),
        BuiltinFunArg::Arg(String::from("f"))
    ]));
    doc_root_mod.add_var(String::from("initdiag"), String::from(&doc[1..]));
    
    let doc = r#"
Creates a matrix from the `X` iterable object that contains the iterable objects which contains
the numbers.

If the `X` object is a matrix, this function returns the `X` object. The created matrix is:

$$ \begin{bmatrix} x_{1 1} & x_{1 2} & \cdots & x_{1M} \\ x_{2 1} & x_{2 2} & \cdots & x_{2M} \\ \vdots & \vdots & \ddots & \vdots \\ x_{N1} & x_{N2} & \cdots & x_{NM} \end{bmatrix} $$

"#;
    sig_root_mod.add_var(String::from("matrix"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("matrix"), String::from(&doc[1..]));
    
    let doc = r#"
Creates a matrix with one row from the `x` iterable object that contains the numbers.

If the `x` object is a matrix with one row, this function returns the `x` object. The created matrix
with one row is:

$$ \begin{bmatrix} x_1 & x_2 & \cdots & x_N \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("rowvector"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("rowvector"), String::from(&doc[1..]));
    
    let doc = r#"
Creates a matrix with one column vector from the `x` iterable object that contains the numbers.

If the `x` object is a matrix with one column, this function returns the `x` object. The created
matrix with one column is:

$$ \begin{bmatrix} x_1 \\ x_2 \\ \vdots \\ x_N \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("colvector"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("colvector"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `X` matrix to a matrix array.

If the `X` object is a matrix array, this function returns the `X` object.
"#;
    sig_root_mod.add_var(String::from("matrixarray"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("matrixarray"), String::from(&doc[1..]));
    
    let doc = r#"
Creates an error with the `kind` error kind and the `msg` message which are strings.
"#;
    sig_root_mod.add_var(String::from("error"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("kind")),
        BuiltinFunArg::Arg(String::from("msg"))
    ]));
    doc_root_mod.add_var(String::from("error"), String::from(&doc[1..]));

    let doc = r#"
Creates an array from the `X` iterable object.

If the `X` value is an array, this function returns the `X` value.
"#;
    sig_root_mod.add_var(String::from("array"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("array"), String::from(&doc[1..]));

    let doc = r#"
Converts the `r` reference to the strong reference.

If the `r` reference is strong, this function returns the `r` reference. 
"#;
    sig_root_mod.add_var(String::from("strong"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("r"))
    ]));
    doc_root_mod.add_var(String::from("strong"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `r` reference to the weak reference.

If the `r` reference is weak, this function returns the `r` reference. 
"#;
    sig_root_mod.add_var(String::from("weak"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("r"))
    ]));
    doc_root_mod.add_var(String::from("weak"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` object is empty, otherwise `false`.

The `X` object can be a string, a matrix array, a matrix row slice, or an array. 
"#;
    sig_root_mod.add_var(String::from("isempty"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("isempty"), String::from(&doc[1..]));

    let doc = r#"
Returns the number of elements in the`X` object.

The `X` object can be a string, a matrix array, a matrix row slice, or an array. This function
returns the number of UTF-8 characters for a string, the number of rows for a matrix array, or the
number of columns for a matrix row slice. 
"#;
    sig_root_mod.add_var(String::from("length"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("length"), String::from(&doc[1..]));

    let doc = r#"
Returns the number of rows in the `X` object.

The `X` object can be a matrix or a matrix array.
"#;
    sig_root_mod.add_var(String::from("rows"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("rows"), String::from(&doc[1..]));

    let doc = r#"
Returns the number of columns in the `X` object.

The `X` object can be a matrix or a matrix array.
"#;
    sig_root_mod.add_var(String::from("columns"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("columns"), String::from(&doc[1..]));

    let doc = r#"
Returns the element with one index or two indices in the `X` indexable object if the `X`
indexable object contains the element, otherwise `none`.

If the `j` index is passed and the `X` value is a matrix array, this function returns the element
with the `i` row index and the `j`  column index in the `X` matrix array. This function returns
the string with one UTF-8 character for a string, the matrix row slice for a matrix array, or the
element of matrix for a matrix row slice if the `j` index isn't passed. This function returns the
field with the `i` identifier in the `X` structure if the `j` index isn't passed and the `X`
object is structure.
"#;
    sig_root_mod.add_var(String::from("get"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("i")),
        BuiltinFunArg::OptArg(String::from("j"))
    ]));
    doc_root_mod.add_var(String::from("get"), String::from(&doc[1..]));

    let doc = r#"
Returns the element with the `i` index in the diagonal of the `X` matrix array if the diagonal of
`X` matrix array contains the element, otherwise `none`.
"#;
    sig_root_mod.add_var(String::from("getdiag"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("i"))
    ]));
    doc_root_mod.add_var(String::from("getdiag"), String::from(&doc[1..]));

    let doc = r#"
Returns the substrings of the `s` string which are separated by the `t` string.

If the `t` string isn't passed, this function uses whitespaces as a separator.
"#;
    sig_root_mod.add_var(String::from("split"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::OptArg(String::from("t"))
    ]));
    doc_root_mod.add_var(String::from("split"), String::from(&doc[1..]));

    let doc = r#"
Returns the `s` string without the start whitespaces and the end whitespaces.
"#;
    sig_root_mod.add_var(String::from("trim"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("trim"), String::from(&doc[1..]));

    let doc = r#"
Returns the `s` string without the start whitespaces and the end whitespaces.
"#;
    sig_root_mod.add_var(String::from("trim"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("trim"), String::from(&doc[1..]));

    let doc = r#"
Returns the `true` if the `s` string contains the `t`, otherwise `false`.
"#;
    sig_root_mod.add_var(String::from("contains"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t"))
    ]));
    doc_root_mod.add_var(String::from("contains"), String::from(&doc[1..]));

    let doc = r#"
Returns the `true` if the `t` is the prefix of the `s` string, otherwise `false`.
"#;
    sig_root_mod.add_var(String::from("startswith"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t"))
    ]));
    doc_root_mod.add_var(String::from("startswith"), String::from(&doc[1..]));

    let doc = r#"
Returns the `true` if the `t` is the suffix of the `s` string, otherwise `false`.
"#;
    sig_root_mod.add_var(String::from("endswith"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t"))
    ]));
    doc_root_mod.add_var(String::from("endswith"), String::from(&doc[1..]));

    let doc = r#"
Replaces all occurrences of the `t` string in the `s` string with the `u` string.

This function returns a new string with replaced occurrences of the `t` string to the `u` string.
"#;
    sig_root_mod.add_var(String::from("replace"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("s")),
        BuiltinFunArg::Arg(String::from("t")),
        BuiltinFunArg::Arg(String::from("u"))
    ]));
    doc_root_mod.add_var(String::from("replace"), String::from(&doc[1..]));

    let doc = r#"
Returns an uppercase string corresponding the `s` string.
"#;
    sig_root_mod.add_var(String::from("upper"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("s"))
    ]));
    doc_root_mod.add_var(String::from("upper"), String::from(&doc[1..]));

    let doc = r#"
Returns a lowercase string corresponding the `s` string.
"#;
    sig_root_mod.add_var(String::from("lower"), Sig::BuiltinFun(vec![
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
    sig_root_mod.add_var(String::from("sort"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("sort"), String::from(&doc[1..]));

    let doc = r#"
Reverses the order of elements in the `x` array.
"#;
    sig_root_mod.add_var(String::from("reverse"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("reverse"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `f` function with the passed `D` value returns a convertible value to `true`
for any element in the `X` iterable object ($f(\mathbf{D}, {\mathbf{x}}_i)$), otherwise `false`.
"#;
    sig_root_mod.add_var(String::from("any"), Sig::BuiltinFun(vec![
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
    sig_root_mod.add_var(String::from("all"), Sig::BuiltinFun(vec![
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
    sig_root_mod.add_var(String::from("find"), Sig::BuiltinFun(vec![
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
    sig_root_mod.add_var(String::from("filter"), Sig::BuiltinFun(vec![
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
    sig_root_mod.add_var(String::from("max"), Sig::BuiltinFun(vec![
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
    sig_root_mod.add_var(String::from("min"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::OptArg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("min"), String::from(&doc[1..]));

    let doc = r#"
Finds maximum element in the `X` iterable object and returns its index.

This function returns `none` if the `X` iterable object is empty.
"#;
    sig_root_mod.add_var(String::from("imax"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("imax"), String::from(&doc[1..]));

    let doc = r#"
Finds minumum element in the `X` iterable object and returns its index.

This function returns `none` if the `X` iterable object is empty.
"#;
    sig_root_mod.add_var(String::from("imin"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("imin"), String::from(&doc[1..]));

    let doc = r#"
Pushes the `y` value to the back of the`X` array.
"#;
    sig_root_mod.add_var(String::from("push"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("y"))
    ]));
    doc_root_mod.add_var(String::from("push"), String::from(&doc[1..]));

    let doc = r#"
Removes the last element from the `X` array and returns the last element.

If the `X` array is empty, this function returns `none`.
"#;
    sig_root_mod.add_var(String::from("pop"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("pop"), String::from(&doc[1..]));

    let doc = r#"
Appends the `Y` mutable object to the `X` mutable object.

The `X` mutable object and the `Y` mutable object must be arrays or structures. If two fields in
two structures have same field identifier, the field in the first structure is overwritten by a
value from the field in the second structure.
"#;
    sig_root_mod.add_var(String::from("append"), Sig::BuiltinFun(vec![
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
    sig_root_mod.add_var(String::from("insert"), Sig::BuiltinFun(vec![
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
    sig_root_mod.add_var(String::from("remove"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("i"))
    ]));
    doc_root_mod.add_var(String::from("remove"), String::from(&doc[1..]));

    let doc = r#"
Returns the error kind for the `e` error.
"#;
    sig_root_mod.add_var(String::from("errorkind"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("e"))
    ]));
    doc_root_mod.add_var(String::from("errorkind"), String::from(&doc[1..]));

    let doc = r#"
Returns the error message for the `e` error.
"#;
    sig_root_mod.add_var(String::from("errormsg"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("e"))
    ]));
    doc_root_mod.add_var(String::from("errormsg"), String::from(&doc[1..]));
    
    let doc = r#"
Returns `true` if the `X` value is equal to the `Y` value, otherwise `false`.

This function doesn't compare matrices. The result of this function is `false` if two values are
matrices. This function doesn't compare value types for integer numbers and floating-point
numbers.
"#;
    sig_root_mod.add_var(String::from("isequal"), Sig::BuiltinFun(vec![
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
    sig_root_mod.add_var(String::from("isnotequal"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("isnotequal"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` value is less than the `Y` value, otherwise `false`.

This function compares two boolean values, two numbers, or two strings. The result of this
function is `false` for two other values.
"#;
    sig_root_mod.add_var(String::from("isless"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("isless"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` value is greater than or equal to the `Y` value, otherwise `false`.

This function compares two boolean values, two numbers, or two strings. The result of this
function is `false` for two other values.
"#;
    sig_root_mod.add_var(String::from("isgreaterequal"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("isgreaterequal"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` value is greater than the `Y` value, otherwise `false`.

This function compares two boolean values, two numbers, or two strings. The result of this
function is `false` for two other values.
"#;
    sig_root_mod.add_var(String::from("isgreater"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("isgreater"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` value is less than or equal to the `Y` value, otherwise `false`.

This function compares two boolean values, two numbers, or two strings. The result of this
function is `false` for two other values.
"#;
    sig_root_mod.add_var(String::from("islessequal"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("islessequal"), String::from(&doc[1..]));

    let doc = r#"
Calculates sigmoid function for the `X` value ($\operatorname{sigmoid}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("sigmoid"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("sigmoid"), String::from(&doc[1..]));

    let doc = r#"
Calculates hyperbolic tangent for the `X` value ($\tanh(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("tanh"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("tanh"), String::from(&doc[1..]));

    let doc = r#"
Calculates swish function for the `X` value ($\operatorname{swish}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("swish"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("swish"), String::from(&doc[1..]));

    let doc = r#"
Calculates softmax function for the `X` value ($\operatorname{softmax}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("softmax"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("softmax"), String::from(&doc[1..]));

    let doc = r#"
Calculates square root of the `X` value ($\sqrt{x}$ or $\sqrt{x_{ij}}$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("sqrt"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("sqrt"), String::from(&doc[1..]));

    let doc = r#"
Indeed transposes the `X` matrix (${\mathbf{X}}^\top$).
"#;
    sig_root_mod.add_var(String::from("reallytranspose"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("reallytranspose"), String::from(&doc[1..]));

    let doc = r#"
This function is alias to the [`reallytranspose`](#var.reallytranspose) function.
"#;
    sig_root_mod.add_var(String::from("rt"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("rt"), String::from(&doc[1..]));

    let doc = r#"
Repeats the `x` vector as column or row.
"#;
    sig_root_mod.add_var(String::from("repeat"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("repeat"), String::from(&doc[1..]));

    let doc = r#"
Calculates remainder of division the `x` value by the `y` value ($\operatorname{mod}(x, y)$).

If the `x` value and the `y` value are integer numbers, this function also returns an integer
number.
"#;
    sig_root_mod.add_var(String::from("mod"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x")),
        BuiltinFunArg::Arg(String::from("y"))
    ]));
    doc_root_mod.add_var(String::from("mod"), String::from(&doc[1..]));
    
    let doc = r#"
Calculates absolute value of the `X` value ($|x|$ or $|x_{ij}|$).

This function is a mathematical function that takes a number, a matrix, or a mutable object. If
the `X` value is an integer number, this function also returns an integer value.
"#;
    sig_root_mod.add_var(String::from("abs"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("abs"), String::from(&doc[1..]));

    let doc = r#"
Raises the `X` value to the power of the `Y` value ($x^y$, ${x_{ij}}^y$, $x^{y_{ij}}$, or
${x_{ij}}^{y_{ij}}$).

This function is a mathematical function that takes two arguments. This argument can be a number,
a matrix, or a mutable object. These arguments can't be a matrix and a mutable object.
"#;
    sig_root_mod.add_var(String::from("pow"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("Y"))
    ]));
    doc_root_mod.add_var(String::from("pow"), String::from(&doc[1..]));

    let doc = r#"
Calculates exponentional function of the `X` value ($e^x$ or $e^{x_{ij}}$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("exp"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("exp"), String::from(&doc[1..]));

    let doc = r#"
Calculates natural logarithm of the `X` value ($\ln{x}$ or $\ln{x_{ij}}$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("log"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("log"), String::from(&doc[1..]));

    let doc = r#"
Calculates base 2 logarithm of the `X` value ($\log_2{x}$ or $\log_2{x_{ij}}$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("log2"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("log2"), String::from(&doc[1..]));

    let doc = r#"
Calculates base 10 logarithm of the `X` value ($\log_10{x}$ or $\log_10{x_{ij}}$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("log10"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("log10"), String::from(&doc[1..]));

    let doc = r#"
Calculates sine function for the `X` value ($\sin(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("sin"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("sin"), String::from(&doc[1..]));

    let doc = r#"
Calculates cosine function for the `X` value ($\cos(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("cos"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("cos"), String::from(&doc[1..]));

    let doc = r#"
Calculates tangent function for the `X` value ($\tan(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("tan"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("tan"), String::from(&doc[1..]));

    let doc = r#"
Calculates arcsine function for the `X` value ($\arcsin(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("asin"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("asin"), String::from(&doc[1..]));

    let doc = r#"
Calculates arccosine function for the `X` value ($\arccos(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("acos"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("acos"), String::from(&doc[1..]));

    let doc = r#"
Calculates arctangent function for the `X` value ($\arctan(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("atan"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("atan"), String::from(&doc[1..]));

    let doc = r#"
Calculates arctangent function for the `X` value and the `Y` value ($\arctan(\frac{x}{y})$,
$\arctan(\frac{x_{ij}}{y})$, $\arctan(\frac{x}{y_{ij}})$, or $\arctan(\frac{x_{ij}}{y_{ij}})$).

This function is a mathematical function that takes two arguments. This argument can be a number,
a matrix, or a mutable object. These arguments can't be a matrix and a mutable object.
"#;
    sig_root_mod.add_var(String::from("atan2"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("atan2"), String::from(&doc[1..]));

    let doc = r#"
Calculates hyperbolic sine function for the `X` value ($\sinh(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("sinh"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("sinh"), String::from(&doc[1..]));

    let doc = r#"
Calculates hyperbolic cosine function for the `X` value ($\cosh(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("cosh"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("cosh"), String::from(&doc[1..]));

    let doc = r#"
Calculates inverse hyperbolic sine function for the `X` value
($\operatorname{arsinh}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("asinh"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("asinh"), String::from(&doc[1..]));

    let doc = r#"
Calculates inverse hyperbolic cosine function for the `X` value
($\operatorname{arcosh}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("acosh"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("acosh"), String::from(&doc[1..]));

    let doc = r#"
Calculates inverse hyperbolic tangent function for the `X` value
($\operatorname{artanh}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("atanh"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("atanh"), String::from(&doc[1..]));

    let doc = r#"
Calculates signum function for the `X` value ($\operatorname{sgn}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("sign"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("sign"), String::from(&doc[1..]));

    let doc = r#"
Calculates ceil function for the `X` value ($\operatorname{ceil}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("ceil"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("ceil"), String::from(&doc[1..]));

    let doc = r#"
Calculates floor function for the `X` value ($\operatorname{floor}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("floor"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("floor"), String::from(&doc[1..]));

    let doc = r#"
Calculates round function for the `X` value ($\operatorname{round}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("round"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("round"), String::from(&doc[1..]));

    let doc = r#"
Calculates trunc function for the `X` value ($\operatorname{trunc}(\mathbf{X})$).

This function is a mathematical function that takes a number, a matrix, or a mutable object.
"#;
    sig_root_mod.add_var(String::from("trunc"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("trunc"), String::from(&doc[1..]));
}
