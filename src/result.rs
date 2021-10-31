//! Extra utilities for handling failures.
//!
//! This module extends capabilities offered by [`std::result`].

/// Extension with a set of extra combinators for `Result<A, E>`.
pub trait ResultExt<A, E> {
    /// Applies `f` yielding yet another result if `Ok(x)` otherwise propagates `Err`.
    ///
    /// ```
    /// use lifterr::result::ResultExt;
    ///
    /// fn ok() -> Result<i32, &'static str> { Ok(1) }
    /// fn err() -> Result<i32, &'static str> { Err("e") }
    ///
    /// assert_eq!(ok().then(|| Ok("42")), Ok("42"));
    /// assert_eq!(err().then(|| Ok("42")), Err("e"));
    /// ```
    fn then<F, B>(self, f: F) -> Result<B, E>
    where
        F: Fn() -> Result<B, E>;

    /// Applies `f` yielding a value which is then wrapped into another result if `Ok(x)` otherwise propagates `Err`.
    ///
    /// ```
    /// use lifterr::result::ResultExt;
    ///
    /// fn ok() -> Result<i32, &'static str> { Ok(1) }
    /// fn err() -> Result<i32, &'static str> { Err("e") }
    ///
    /// assert_eq!(ok().remap(|| "42"), Ok("42"));
    /// assert_eq!(err().remap(|| "42"), Err("e"));
    /// ```
    fn remap<F, B>(self, f: F) -> Result<B, E>
    where
        Self: Sized,
        F: Fn() -> B,
    {
        self.then(|| Ok(f()))
    }

    /// Replaces whatever value of type `A` in `Result<A, E>` with an unit.
    fn void(self) -> Result<(), E>
    where
        Self: Sized,
    {
        self.remap(|| ())
    }
}

impl<A, E> ResultExt<A, E> for Result<A, E> {
    fn then<F, B>(self, f: F) -> Result<B, E>
    where
        F: Fn() -> Result<B, E>,
    {
        self.and_then(|_| f())
    }
}
