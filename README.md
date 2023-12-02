<style>
  picture {
    display: block;
    margin-left: auto;
    margin-right: auto;
    width: 75%;
  }
</style>

<a href="https://github.com/Pencilcaseman/hasty">
    <picture>
      <source
        srcset="https://raw.githubusercontent.com/Pencilcaseman/hasty/master/img/logo_dark_mode.png" 
        media="(prefers-color-scheme: dark)">
      <img src="https://raw.githubusercontent.com/Pencilcaseman/hasty/master/img/logo_light_mode.png">
    </picture>
</a>

# Hasty

Hasty provides a Rust-native interface to high-performance BLAS libraries, such as
[OpenBLAS](https://github.com/OpenMathLib/OpenBLAS),
[Intel MKL](https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html),
[Apple Accelerate](https://developer.apple.com/documentation/accelerate), and more.

Unlike existing BLAS bindings, Hasty will automatically detect and link to the best available
BLAS library on your system without any configuration required. You can also specify a path to
a specific BLAS library via the `HASTY_BLAS_PATH` environment variable, if you wish.

*Note that you may need to perform a clean build of your project if you change the BLAS library
that Hasty links to.*

For more information, see the [documentation](https://docs.rs/hasty).

## Example

```rust
fn main() {
    let lib = hasty::get_blas_library();
    println!("Using BLAS Library: {lib}");
    
    type Scalar = f32;

    let m: u64 = 2;
    let n: u64 = 1;
    let k: u64 = 3;
    let mut a: Vec<Scalar> = vec![0.0; (m * k) as usize];
    let mut b: Vec<Scalar> = vec![0.0; (k * n) as usize];
    let mut c: Vec<Scalar> = vec![0.0; (m * n) as usize];

    for i in 0..(m * k) {
        a[i as usize] = i as Scalar + 1.0;
    }

    for i in 0..(k * n) {
        b[i as usize] = i as Scalar + 1.0;
    }

    hasty::level3::gemm(
        hasty::StorageOrder::RowMajor,
        hasty::Transpose::NoTrans,
        hasty::Transpose::NoTrans,
        m,
        n,
        k,
        1.0,
        &a,
        k,
        &b,
        n,
        0.0,
        &mut c,
        n,
    );

    println!("Result: {:?}", c);
}
```

## Development Plans

### More BLAS Libraries

Hasty currently supports a range of BLAS libraries, but it's difficult to test them all. We want to support
as many BLAS libraries as possible, so if you find a configuration that doesn't work, please open an issue.

### Fall-back BLAS Library

We aim to have fall-back implementations for all BLAS functions if we don't find a suitable BLAS library on
your system, but they will be much slower than the optimized implementations provided by BLAS libraries.
Ideally, we'd like to optimise these implementations as much as possible, but that's a tricky task and will
require contributions from the community.

### Missing Functions

Hasty is still a work in progress, and there are a ***lot*** of BLAS functions to implement.
If you need a function that isn't implemented yet, please open an issue or submit a pull request,
and I'll get it added as soon as possible!
