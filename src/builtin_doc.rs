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

$$ \begin{bmatrix} f(d, 1, 1) & f(d, 1, 2) & \cdots & f(d, 1, M) \\ f(d, 2, 1)  & f(d, 2, 2) & \cdots & f(d, 2, M) \\ \vdots & \vdots & \ddots & \vdots \\ f(d, N, 1) & f(d, N, 2) & \cdots & f(d, N, M) \end{bmatrix} $$
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
}
