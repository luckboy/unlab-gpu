//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::doc::*;
use crate::mod_node::*;

pub fn add_getopts_doc(sig_root_mod: &mut ModNode<Sig, ()>, doc_root_mod: &mut ModNode<String, Option<String>>)
{
    let doc = r#"
# Getopts functions

This library contains the getopts functions which are:

- [`getopts`](#var.getopts)
- [`getoptsusage`](#var.getoptsusage)

The getopts functions takes the `opts` array that contains elements which are option arrays. The
option array contains the following elements:

- short option name that is one character
- long option name that can be empty string
- description
- hint that is used in place of the option argument in the usage
  (optional) (default: empty string)
- flag of the option argument can have the following values (optional):
  - `"yes"` - option argument is required (default for hint that isn't empty string)
  - `"no"` - no option argument (default for hint that is empty string)
  - `"maybe"` - option argument is optional
- flag of the option occurence can have the following values (optional):
  - `"req"` - option can occur once
  - `"optional"` - option can occur at most once (default)
  - `"multi"` - option can occur zero or more times
"#;
    match doc_root_mod.value() {
        Some(prev_doc) => doc_root_mod.set_value(Some(prev_doc.clone() + "\n" + &doc[1..])),
        None => doc_root_mod.set_value(Some(String::from(&doc[1..]))),
    }

    let doc = r#"
Parses the arguments for the `opts` options.

The `args` arguments are parsed if the `args` arguments are passed. This function returns a
structure with the following fields:

- `opts` - structure with option fields which have option names as field identifiers
- `free` - free arguments as array

The option name can be long option name if long option name isn't an empty string, otherwise short 
option name. All `-` characters in the identifiers of option fields of result are replaced by the
`_` characters. The option field of result is array of option arguments if the option is matched by
this function, otherwise `none`. This function can return an error with the `getopts` error kind.

# Examples

```
opts = .[.]
push(opts, .[ "a", "abc", "Abc option" .])
push(opts, .[ "d", "def", "Def option", "FILE" .])
push(opts, .[ "g", "ghi", "Ghi option" .])
println(getopts(opts, .[ "-a", "--def", "file.txt" .])?)
```
"#;
    sig_root_mod.add_var(String::from("getopts"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("opts")),
        BuiltinFunArg::OptArg(String::from("args"))
    ]));
    doc_root_mod.add_var(String::from("getopts"), String::from(&doc[1..]));

    let doc = r#"
Returns the usage from the `opts` options with the `brief` string.

# Examples

```
opts = .[.]
push(opts, .[ "a", "abc", "Abc option" .])
push(opts, .[ "d", "def", "Def option", "FILE" .])
push(opts, .[ "g", "ghi", "Ghi option" .])
println(getoptsusage(opts, "Usage: program [options]"))
```
"#;
    sig_root_mod.add_var(String::from("getoptsusage"), Sig::BuiltinFun(vec![
        BuiltinFunArg::Arg(String::from("opts")),
        BuiltinFunArg::Arg(String::from("brief"))
    ]));
    doc_root_mod.add_var(String::from("getoptsusage"), String::from(&doc[1..]));
}
