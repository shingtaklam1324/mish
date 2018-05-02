# Mish

A `libm`-ish program. It provides most of the functions of `libm` in pure `#![no_std]` Rust.
Note that it is completely unoptimised, using `+-*/` and loops only.

Most of the Trig functions are from a CORDIC implementation, apart from `sin`, `cos` and `tan`,
which are done using a Taylor series to the term of `x^7`

Hyperbolic trig functions are still missing at the moment.

Documentation is missing, but most of the signatures are the same as `std`, so use those docs instead.

Most functions have been checked to 4 d.p. with a Casio Scientific Calculator, consider increasing
the amount of iterations and validating before using for applications where accuracy is critical.

## Modules

* `funcs` is where all of the functions are located
    * `m` contains a majority of the mathematical functions
    * `n` deals with the floating point numbers themselves
    * `o` has some basic operations
    * `t` contains the trig functions (at root: (`angle_shift` and `cbrt`))
        * `consts` contains the loop up tables for `inv`
        * `inv` contains the inverse trig functions
        * `t` contains the trig functions
* `prelude` is where most users should import from
    * `f32` contains all functions for `f32`
    * `f64` contains all functions for `f64`

**Note**: All functions with iterative methods have a counterpart, not included in `prelude`, with end with `_`.
These methods have an extra integer (`i32` or `usize`) argument, which specifies the amount of iterations to run for.
This can be used to specify precision vs speed of execution. The methods included by default are wrappers with a default
value for the loop iterations.
