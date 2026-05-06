# Unlab tutorial

## Copyright and license

Copyright (c) 2026 Łukasz Szpakowski

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.

## Introduction

The Unlab scripting language is a simple scripting language for GPU that operates on metrices. This
scripting language can be used to create and train neural networks. This tutorial can help you learn
this scripting language.

## Standard documentation

The standard documentation is a documentation for the standard library. This documentation isn't
generated while installation of the Unlab interpreter. If you have access to this documentation, you
should invoke the following command:

```
unlab-pkg std-doc
```

If your browser hasn't access to hidden files as Firefox from Ubuntu, you should change the
documentation path by add the following line to shell configuration (`.bashrc` for bash) before the
generation of standard documentation to browse this documentation:

```
export UNLAB_GPU_DOC_PATH=$HOME/unlab-gpu-doc
```

## Interpreter

An interpreter is a program that interprets entered lines or a script code in the Unlab scripting
language. The interpreter can work in an interactive mode or a non-interactive mode.

### Interactive mode

The interactive mode allows you to enter and edit lines which are interpreted. Also, the interactive
mode allows you to access to the command history by press the up key or the down key. The interpreter
can be runned by invoke the following command in the interactive mode:

```
unlab-gpu
```

The sample interaction in the interactive mode:

```
unlab-gpu:1> println("Hello world!!!")
Hello world!!!
unlab-gpu:2> function f(x)
> x + 1
> end
unlab-gpu:5> println(f(2))
3
unlab-gpu:6> quit
```

You can leave from the interpreter by invoke the `quit` command.

### Non-interactive mode

The non-interactive mode allows you execute scripts in the Unlab scritping language. The interpreter
can be runned by invoke the following command in the non-interactive mode for the `script.un` file:

```
unlab-gpu script.un
```
