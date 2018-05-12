# Mish

A `libm`-ish program. It provides most of the functions of `libm` in pure `#![no_std]` Rust.
Note that it is completely unoptimised, using `+-*/` and loops only.

Documentation is missing, but most of the signatures are the same as `std`, so use those docs instead.

Most functions have been checked, consider increasing
the amount of iterations and validating before using for applications where accuracy is critical.

This is a proof of concept, and for most of the functions, they only work within a range. This is not
intended to be a replacement for libm. For that, look to something like `m`.

## Using

Add this to your `Cargo.toml`

```toml
mish = "0.2.1"
```

add this to your crate root

```rust
extern crate mish;
```

and import all functions

```rust
use mish::*;
```

## Modules

* `funcs` is where all of the functions are located, all inner functions have been re-exported here
    * `m` contains a majority of the mathematical functions
    * `n` deals with the floating point numbers themselves
    * `o` has some basic operations
    * `t` contains the trig functions
        * `inv` contains the inverse trig functions
        * `t` contains the trig functions
        * `h` contains hyperbolic functions
        * `hinv` contains inverse hyperbolic functions

**Note**: All functions with iterative methods have a counterpart, not included in `prelude`, that end with `_`.
These methods have an extra integer (`i32` or `usize`) argument, which specifies the amount of iterations to run for.
This can be used to specify precision vs speed of execution. The methods included by default are wrappers with a default
value for the loop iterations.
