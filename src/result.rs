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
        self.then(|| f().into_ok())
    }

    /// Replaces whatever value of type `A` in `Result<A, E>` with an unit.
    fn void(self) -> Result<(), E>
    where
        Self: Sized,
    {
        self.remap(|| ())
    }

    /// Applies `f` yielding yet another result if `Err(x)` otherwise propagates `Ok`.
    ///
    /// ```
    /// use lifterr::result::ResultExt;
    ///
    /// fn ok() -> Result<i32, &'static str> { Ok(1) }
    /// fn err() -> Result<i32, &'static str> { Err("e") }
    ///
    /// assert_eq!(ok().then_err(|| Err("42")), Ok(1));
    /// assert_eq!(err().then_err(|| Err("42")), Err("42"));
    /// ```
    fn then_err<F, H>(self, f: F) -> Result<A, H>
    where
        F: Fn() -> Result<A, H>;

    /// Applies `f` yielding a value which is then wrapped into another result if `Err(x)` otherwise propagates `Ok`.
    ///
    /// ```
    /// use lifterr::result::ResultExt;
    ///
    /// fn ok() -> Result<i32, &'static str> { Ok(1) }
    /// fn err() -> Result<i32, &'static str> { Err("e") }
    ///
    /// assert_eq!(ok().remap_err(|| "42"), Ok(1));
    /// assert_eq!(err().remap_err(|| "42"), Err("42"));
    /// ```
    fn remap_err<F, H>(self, f: F) -> Result<A, H>
    where
        Self: Sized,
        F: Fn() -> H,
    {
        self.then_err(|| f().into_err())
    }

    /// Replaces whatever value of type `E` in `Result<A, E>` with an unit.
    #[allow(clippy::result_unit_err)]
    fn void_err(self) -> Result<A, ()>
    where
        Self: Sized,
    {
        self.remap_err(|| ())
    }

    /// Swaps `Ok(o)` into `Err(o)` or `Err(e)` into `Ok(e)`.
    ///
    /// ```
    /// use lifterr::result::ResultExt;
    ///
    /// fn ok() -> Result<i32, &'static str> { Ok(1) }
    /// fn err() -> Result<i32, &'static str> { Err("e") }
    ///
    /// assert_eq!(ok().swap(), Err(1));
    /// assert_eq!(err().swap(), Ok("e"));
    /// ```
    fn swap(self) -> Result<E, A>;

    /// Recovers from an error of type `E` with a non-fallible function.
    fn recover<F>(self, f: F) -> Result<A, E>
    where
        F: FnOnce(E) -> A,
        Self: Sized,
    {
        self.recover_with(|e| f(e).into_ok())
    }

    /// Recovers from an error of type `E` with a fallible function, possibly remapping to a different error of type `H`.
    ///
    /// ```
    /// use lifterr::result::ResultExt;
    ///
    /// fn err_not_fine() -> Result<i32, &'static str> { Err("bad error") }
    /// fn err_fine() -> Result<i32, &'static str> { Err("42") }
    ///
    /// assert_eq!(err_not_fine().recover_with(|e| if e == "42" { Ok(42) } else { Err("not fine") }), Err("not fine"));
    /// assert_eq!(err_fine().recover_with(|e| if e == "42" { Ok(42) } else { Err("not fine") }), Ok(42));
    /// ```
    fn recover_with<F, H>(self, f: F) -> Result<A, H>
    where
        F: FnOnce(E) -> Result<A, H>;
}

impl<A, E> ResultExt<A, E> for Result<A, E> {
    fn then<F, B>(self, f: F) -> Result<B, E>
    where
        F: Fn() -> Result<B, E>,
    {
        self.and_then(|_| f())
    }

    fn then_err<F, H>(self, f: F) -> Result<A, H>
    where
        F: Fn() -> Result<A, H>,
    {
        self.or_else(|_| f())
    }

    fn swap(self) -> Result<E, A> {
        match self {
            Ok(o) => Err(o),
            Err(e) => Ok(e),
        }
    }

    fn recover_with<F, H>(self, f: F) -> Result<A, H>
    where
        F: FnOnce(E) -> Result<A, H>,
    {
        self.map_or_else(f, A::into_ok)
    }
}

/// Lifter of values into successful results.
pub trait IntoOk<O> {
    /// Lifts a value of type `O` into a `Result<O, E>` by wrapping it into an `Ok`.
    ///
    /// ```
    /// use lifterr::result::IntoOk;
    ///
    /// assert_eq!(42.into_ok::<&'static str>(), Ok(42));
    /// ```
    fn into_ok<E>(self) -> Result<O, E>;
}

impl<O> IntoOk<O> for O {
    fn into_ok<E>(self) -> Result<O, E> {
        Ok(self)
    }
}

/// Lifter of values into failed results.
pub trait IntoErr<E> {
    /// Lifts a value of type `E` into a `Result<O, E>` by wrapping it into an `Err`.
    ///
    /// ```
    /// use lifterr::result::IntoErr;
    ///
    /// assert_eq!("e".into_err::<i32>(), Err("e"));
    /// ```
    fn into_err<O>(self) -> Result<O, E>;
}

impl<E> IntoErr<E> for E {
    fn into_err<O>(self) -> Result<O, E> {
        Err(self)
    }
}
