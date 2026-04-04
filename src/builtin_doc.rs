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
Returns a string corresponding to the value type.

The stings corresponding to the value types are:

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
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("type"), String::from(&doc[1..]));
    let doc = r#"
Converts the `x` value to a boolean value.

This function returns `true` if the `x` value isn't `none`, `false`, zero, or an error, otherwise
`false`.
"#;
    sig_root_mod.add_var(String::from("bool"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("bool"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `x` value to an integer number.

The `x` number is converted to an integer number by this function. This function returns `1` for a
non-numeric value if the `x` value isn't `none`, `false`, or an error, otherwise `0`.
"#;
    sig_root_mod.add_var(String::from("int"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("int"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `x` value to a float-point number.

The `x` number is converted to a float-point number by this function. This function returns `1.0`
for a non-numeric value if the `x` value isn't `none`, `false`, or an error, otherwise `0.0`.
"#;
    sig_root_mod.add_var(String::from("float"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("float"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `x` value to a string.
"#;
    sig_root_mod.add_var(String::from("string"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
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

This function applies the `f` function to the `d` value and the element indices for each element of
initialized matrix. The initialized matrix is:

$$ \begin{bmatrix} f(d, 1, 1) & f(d, 1, 2) & \cdots & f(d, 1, M) \\ f(d, 2, 1) & f(d, 2, 2) & \cdots & f(d, 2, M) \\ \vdots & \vdots & \ddots & \vdots \\ f(d, N, 1) & f(d, N, 2) & \cdots & f(d, N, M) \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("init"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("N")),
        BuiltinFunArg::Arg(String::from("M")),
        BuiltinFunArg::Arg(String::from("d")),
        BuiltinFunArg::Arg(String::from("f"))
    ]));
    doc_root_mod.add_var(String::from("init"), String::from(&doc[1..]));
    
    let doc = r#"
Returns an initialized diagonal matrix that has the `N` number of rows and columns.

This function applies the `f` function to the `d` value and the element index for each element of
main diagonal of initialized diagonal matrix. The initialized diagonal matrix is:

$$ \begin{bmatrix} f(d, 1) & 0 & \cdots & 0 \\ 0  & f(d, 2) & \cdots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \cdots & f(d, N) \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("initdiag"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("N")),
        BuiltinFunArg::Arg(String::from("d")),
        BuiltinFunArg::Arg(String::from("f"))
    ]));
    doc_root_mod.add_var(String::from("initdiag"), String::from(&doc[1..]));
    
    let doc = r#"
Creates a matrix from the `X` iterable value that contains the iterable values which contains
the numbers.

If the `X` value is a matrix, this function returns the `X` value. The created matrix is:

$$ \begin{bmatrix} x_{1 1} & x_{1 2} & \cdots & x_{1M} \\ x_{2 1} & x_{2 2} & \cdots & x_{2M} \\ \vdots & \vdots & \ddots & \vdots \\ x_{N1} & x_{N2} & \cdots & x_{NM} \end{bmatrix} $$

"#;
    sig_root_mod.add_var(String::from("matrix"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("matrix"), String::from(&doc[1..]));
    
    let doc = r#"
Creates a matrix with one row from the `x` iterable value that contains the numbers.

If the `x` value is a matrix with one row, this function returns the `x` value. The created matrix
with one row is:

$$ \begin{bmatrix} x_1 & x_2 & \cdots & x_N \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("rowvector"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("rowvector"), String::from(&doc[1..]));
    
    let doc = r#"
Creates a matrix with one column vector from the `x` iterable value that contains the numbers.

If the `x` value is a matrix with one column, this function returns the `x` value. The created
matrix with one column is:

$$ \begin{bmatrix} x_1 \\ x_2 \\ \vdots \\ x_N \end{bmatrix} $$
"#;
    sig_root_mod.add_var(String::from("colvector"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("colvector"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `X` matrix to a matrix array.

If the `X` value is the matrix array, this function returns the `X` value.
"#;
    sig_root_mod.add_var(String::from("matrixarray"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("matrixarray"), String::from(&doc[1..]));
    
    let doc = r#"
Creates an error from the `kind` string and the `msg` string.

The `kind` string is an error kind and the `msg` string is a message.
"#;
    sig_root_mod.add_var(String::from("error"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("kind")),
        BuiltinFunArg::Arg(String::from("msg"))
    ]));
    doc_root_mod.add_var(String::from("error"), String::from(&doc[1..]));

    let doc = r#"
Creates an array from the `x` iterable value.

If the `x` value is an array, this function returns the `x` value.
"#;
    sig_root_mod.add_var(String::from("array"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("array"), String::from(&doc[1..]));

    let doc = r#"
Converts the `x` reference to the strong reference.

If the `x` reference is strong, this function returns the `x` reference. 
"#;
    sig_root_mod.add_var(String::from("strong"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("strong"), String::from(&doc[1..]));
    
    let doc = r#"
Converts the `x` reference to the weak reference.

If the `x` reference is weak, this function returns the `x` reference. 
"#;
    sig_root_mod.add_var(String::from("weak"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("x"))
    ]));
    doc_root_mod.add_var(String::from("weak"), String::from(&doc[1..]));

    let doc = r#"
Returns `true` if the `X` value is empty, otherwise `false`. 

The `X` value can be a string, a matrix array, a matrix row slice, or an array. 
"#;
    sig_root_mod.add_var(String::from("isempty"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("isempty"), String::from(&doc[1..]));

    let doc = r#"
Returns the length of `X` value. 

The `X` value can be a string, a matrix array, a matrix row slice, or an array. This function
returns the number of rows for a matrix array and the number of columns for a matrix row slice. 
"#;
    sig_root_mod.add_var(String::from("length"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("length"), String::from(&doc[1..]));

    let doc = r#"
Returns the number of rows of `X` value.

The `X` value can be a matrix or a matrix array.
"#;
    sig_root_mod.add_var(String::from("rows"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("rows"), String::from(&doc[1..]));

    let doc = r#"
Returns the number of columns of `X` value.

The `X` value can be a matrix or a matrix array.
"#;
    sig_root_mod.add_var(String::from("columns"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X"))
    ]));
    doc_root_mod.add_var(String::from("columns"), String::from(&doc[1..]));

    let doc = r#"
Returns the element of `X` indexable value with the index or the indices if the `X` indexable 
value contains the element, otherwise `none`.

If the `j` index is passed to this function and the `X` value is matrix array, this function
returns the element of the `X` matrix array with the `i` row index and the `j`  column index. This
function returns the field of the `X` structure with the `i` identifier if the `X` value is
structure.
"#;
    sig_root_mod.add_var(String::from("get"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("i")),
        BuiltinFunArg::OptArg(String::from("j"))
    ]));
    doc_root_mod.add_var(String::from("get"), String::from(&doc[1..]));

    let doc = r#"
Returns the element of diagonal of `X` matrix array with the `i` index if the diagonal of `X`
matrix array contains the element, otherwise `none`.
"#;
    sig_root_mod.add_var(String::from("getdiag"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("X")),
        BuiltinFunArg::Arg(String::from("i"))
    ]));
    doc_root_mod.add_var(String::from("getdiag"), String::from(&doc[1..]));
}
