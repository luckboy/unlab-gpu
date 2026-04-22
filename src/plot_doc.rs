//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::doc::*;
use crate::mod_node::*;

pub fn add_plot_doc(sig_root_mod: &mut ModNode<Sig, ()>, doc_root_mod: &mut ModNode<String, Option<String>>)
{
    let doc = r#"
# Plotting functions

The plotting function allows to draw charts and/or histograms. This library contains the plotting
functions which are:

- [`plot`](#var.plot)
- [`plot3`](#var.plot3)
- [`histogram`](#var.histogram)
- [`hist`](#var.hist)

The series represents date that is related to each other. Also, series can have series string that
consists of series kind, color, and label. The syntax of series string is:

```
series string = [series kind], [series color], [",", label]
```

The series kind can be:

- `-` - line (default)
- `--` - dashed line
- `:` - dotted line
- `o` - circle
- `x` - cross
- `.` - point
- `^` - triangle
- `sxy` - surface on a $X$ axis and an $Y$ axis.
- `sxz` - surface on a $X$ axis and a $Z$ axis.
- `syz` - surface on an $Y$ axis and a $Z$ axis.

The series color can be:

- `r` - red
- `g` - green
- `b` - blue
- `c` - cyan
- `m` - magenta
- `y` - yellow
- `k` - black
- `w` - white

The label is separated from the series kind and the series color by comma. 
"#;
    match doc_root_mod.value() {
        Some(prev_doc) => doc_root_mod.set_value(Some(prev_doc.clone() + "\n" + &doc[1..])),
        None => doc_root_mod.set_value(Some(String::from(&doc[1..]))),
    }
    
    let doc = r#"
Draws the 2D chart on the window and/or saves to the file.

The series consists of the `Xi` values, the `Yi` values, and the `si` series string. One function
with one argument can be one of arguments for series. This function can return an error with the
`plot` error kind.

# Examples

```
chart = {
    x: .[ -1.0, 1.0 .]
    y: .[ -0.1, 1.0 .]
}
function f(x)
    x * x
end
plot(chart, -1.0 to 1.0 by 0.02, f, ",x^2")?
```
"#;
    sig_root_mod.add_var(String::from("plot"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("chart")),
        BuiltinFunArg::OptArg(String::from("X1")),
        BuiltinFunArg::OptArg(String::from("Y1")),
        BuiltinFunArg::OptArg(String::from("s1")),
        BuiltinFunArg::DotDotDot,
        BuiltinFunArg::OptArg(String::from("XN")),
        BuiltinFunArg::OptArg(String::from("YN")),
        BuiltinFunArg::OptArg(String::from("sN"))
    ]));
    doc_root_mod.add_var(String::from("plot"), String::from(&doc[1..]));

    let doc = r#"
Draws the 3D chart on the window and/or saves to the file.

The series consists of the `Xi` values, the `Yi` values, the `Zi` values, and the `si` series
string. Two functions with one argument can be two of arguments for line series. The surface
object with the rows and the columns can be:

- the `Z` object for the surface on a $X$ axis and an $Y$ axis
- the `Y` object for the surface on a $X$ axis and a $Z$ axis
- the `X` object for the surface on an $Y$ axis and a $Z$ axis

The columns and the rows of surface object are:

- the `X` values and the `Y` values for the surface on a $X$ axis and an $Y$ axis
- the `X` values and the `Z` values for the surface on a $X$ axis and a $Z$ axis
- the `Y` values and the `Z` values for the surface on an $Y$ axis and a $Z$ axis

One function with two arguments can be one argument for surface series. The surface function can
be:

- the `Z` function for the surface on a $X$ axis and an $Y$ axis
- the `Y` function for the surface on a $X$ axis and a $Z$ axis
- the `X` function for the surface on an $Y$ axis and a $Z$ axis

Two arguments of surface function are:

- the `X` values and the `Y` values for the surface on a $X$ axis and an $Y$ axis
- the `X` values and the `Z` values for the surface on a $X$ axis and a $Z$ axis
- the `Y` values and the `Z` values for the surface on an $Y$ axis and a $Z$ axis

This function can return an error with the `plot` error kind.

# Examples

```
chart = {
    x: .[ -3.0, 3.0 .]
    y: .[ -3.0, 3.0 .]
    z: .[ -3.0, 3.0 .]
}
function sin10(x)
    sin(x * 10.0)
end
function cos10(x)
    cos(x * 10.0)
end
plot3(chart, sin10, -2.5 to 2.5 by 0.025, cos10, ",line")?
```

```
chart = {
    x: .[ -3.0, 3.0 .]
    y: .[ -3.0, 3.0 .]
    z: .[ -3.0, 3.0 .]
}
function f(x, y)
    cos(x * x + y * y)
end
plot3(chart, -3.0 to 3.0 by 0.1, f, -3.0 to 3.0 by 0.1, "sxz,surface")?
```
"#;
    sig_root_mod.add_var(String::from("plot3"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("chart")),
        BuiltinFunArg::OptArg(String::from("X1")),
        BuiltinFunArg::OptArg(String::from("Y1")),
        BuiltinFunArg::OptArg(String::from("Z1")),
        BuiltinFunArg::OptArg(String::from("s1")),
        BuiltinFunArg::DotDotDot,
        BuiltinFunArg::OptArg(String::from("XN")),
        BuiltinFunArg::OptArg(String::from("YN")),
        BuiltinFunArg::OptArg(String::from("ZN")),
        BuiltinFunArg::OptArg(String::from("sN"))
    ]));
    doc_root_mod.add_var(String::from("plot3"), String::from(&doc[1..]));

    let doc = r#"
Draws the histogram on the window and/or saves to the file.

The series consists of the `di` data and the `si` series string. The `di` data should be an array.
The element in the array can be:

- boolean value
- integer number 
- floating-point number
- string

This function can return an error with the `plot` error kind.

# Examples

```
chart = {
    x: 1 to 3
    y: .[ 0, 9 .]
}
d = .[ 1, 1, 2, 2, 1, 3, 3, 2, 2, 1, 1, 2, 2, 2, 3, 3, 1, 2, 3 .]
histogram(chart, d, "")?
```
"#;
    sig_root_mod.add_var(String::from("histogram"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("chart")),
        BuiltinFunArg::OptArg(String::from("d1")),
        BuiltinFunArg::OptArg(String::from("s1")),
        BuiltinFunArg::DotDotDot,
        BuiltinFunArg::OptArg(String::from("dN")),
        BuiltinFunArg::OptArg(String::from("sN"))
    ]));
    doc_root_mod.add_var(String::from("histogram"), String::from(&doc[1..]));

    let doc = r#"
This function is alias to the [`histogram`](#var.histogram) function.
"#;
    sig_root_mod.add_var(String::from("hist"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("chart")),
        BuiltinFunArg::OptArg(String::from("d1")),
        BuiltinFunArg::OptArg(String::from("s1")),
        BuiltinFunArg::DotDotDot,
        BuiltinFunArg::OptArg(String::from("dN")),
        BuiltinFunArg::OptArg(String::from("sN"))
    ]));
    doc_root_mod.add_var(String::from("hist"), String::from(&doc[1..]));
}
