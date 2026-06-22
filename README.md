# Unlab-gpu

Micro neural scripting language for GPU is simple scripting language that operates on matrices. This
scripting language is created to create and train neural networks by operate on the matrices. The
Unlab-gpu crate contains an interpreter of this scripting language and a package manager for this
scripting language. This crate also provides a library of this scripting language.

## Programs

This crate contains the following programs:

- `unlab-gpu` - the interpreter of this scripting language
- `unlab-pkg` - the package manager for this scripting language

## Installation

You can install the programs of this crate by invoke the following command:

```
cargo install unlab-gpu
```

You also can install the programs of this crate for example with support for CUDA v11.5 by invoke the
following command:

```
cargo install --features cuda-11050 unlab-gpu
```

Then you can generate the documentation of standard library by invoke the following command:

```
unlab-pkg std-doc
```

If your browser hasn't access to the hidden files as Firefox from Ubuntu, you should add the following
line to shell configuration (`.bashrc` for bash) before the generation of standard documentation to
browse this documentation:

```
export UNLAB_GPU_DOC_PATH=$HOME/unlab-gpu-doc
```

## Usage

You can use the library of this crate by add the following lines in the `Cargo.toml` file:

```toml
[dependencies]
unlab-gpu = "0.1.0"
```

## Features

The following features of this crate can be used by you:

- `opencl` - use OpenCL (default)
- `cuda` - use CUDA
- `cuda-*` - choose CUDA version (for example `cuda-11050`)
- `default_cublas` - use the cuBLAS library to multiplication of matrices as default for CUDA
- `default_mma` - use the mma instruction to multiplication of matrices as default for CUDA
- `plot` - use plotter for the interpreter (default)

## Library installation

You can install the `github.com/luckboy/unn` library by invoke the following command:

```
unlab-pkg install github.com/luckboy/unn
```

## Examples

The following example presents multiplication of matrices:

```
A = [1, 2; 3, 4; 5, 6]
B = [1, 2, 3; 4, 5, 6]
println("A * B = ", A * B)
```

The following example presents chart drawing:

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

The following example presents training of neural network:

```
uselib("pl.luckboy/unn")
usemods("pl_luckboy_unn")
usevars("pl_luckboy_unn")
net = mlp(.[ 10, 100, 15 .], .[ tanh .], se, xavier_init)
X = [rand() fill 100; fill 10]
Y = [rand() fill 100; fill 15]
net2 = etrain(100, net, X, Y, { eta: 0.1 }, alg::gd, none, true, true)?
```

The above example requires the `github.com/luckboy/unn" library.

## Documentation

If you want to learn this scripting language, you should read the
[Unlab tutorial](https://github.com/luckboy/unlab-gpu/blob/master/doc/tutorial.md). The documentation
files in the `doc` directory are a documentation for this scipting language and the package manager:

- [`tutorial.md`](https://github.com/luckboy/unlab-gpu/blob/master/doc/tutorial.md) - the Unlab
  tutorial
- [`reference.md`](https://github.com/luckboy/unlab-gpu/blob/master/doc/reference.md) - the Unlab
  reference
- [`pkg-reference.md`](https://github.com/luckboy/unlab-gpu/blob/master/doc/pkg-reference.md) - the
  Unlab-pkg reference
- [`backend.md`](https://github.com/luckboy/unlab-gpu/blob/master/doc/backend.md) - the backend
  configuration
- [`environment.md`](https://github.com/luckboy/unlab-gpu/blob/master/doc/environment.md) - the
  environment

If you want to browse the documentation of standard library, you can open this documentation by enter
the following command to the interpreter:

```
doc()
```

## License

This software is licensed under the Mozilla Public License v2.0. See the LICENSE file for the full
licensing terms.
