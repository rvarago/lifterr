# lifterr

> A small set of adapters extending Rust's error-handling capabilities.

[<img alt="github" src="https://img.shields.io/badge/github-rvarago/lifterr?style=for-the-badge&logo=github" height="20">](https://github.com/rvarago/lifterr)
[<img alt="crates.io" src="https://img.shields.io/crates/v/lifterr.svg?style=for-the-badge&logo=rust" height="20">](https://crates.io/crates/lifterr)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-lifterr?style=for-the-badge" height="20">](https://docs.rs/lifterr)

## Option

Rust's [Option](https://doc.rust-lang.org/std/option/enum.Option.html) ships with a suite
of combinators meant to ease common tasks requiring handling optionality safely.

This library extends it with a little extra set of capabilities as defined in the `OptionExt<A>` trait.

## Result

Rust's [Result](https://doc.rust-lang.org/std/result/enum.Result.html) ships with a suite
of combinators meant to ease common tasks requiring handling failure safely.

This library extends it with a little extra set of capabilities as defined in the `ResultExt<A, E>` trait.

Moreover, it's offered convenient functions to lift values into successful or failed results with the traits
`IntoOk<O>` and `IntoErr<E>`, respectively.
