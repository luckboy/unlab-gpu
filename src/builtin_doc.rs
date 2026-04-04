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
Converts any value to a string.
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
}
