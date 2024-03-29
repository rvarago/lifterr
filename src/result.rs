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
        F: FnOnce() -> Result<B, E>;

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
        F: FnOnce() -> B,
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
        F: FnOnce() -> Result<A, H>;

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
        F: FnOnce() -> H,
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

    /// Runs `f` with a reference to `A` when `Ok(a)`.
    ///
    /// ```
    /// use lifterr::result::ResultExt;
    /// assert_eq!(Ok::<_, i32>(10).inspect(|a| println!("a = {a}")), Ok(10));
    /// ```
    fn inspect<F>(self, f: F) -> Result<A, E>
    where
        F: FnOnce(&A);

    /// Runs `f` with a reference to `E` when `Err(e)`.
    ///
    /// ```
    /// use lifterr::result::ResultExt;
    /// assert_eq!(Err::<i32, _>(10).inspect(|e| println!("e = {e}")), Err(10));
    /// ```
    fn inspect_err<F>(self, f: F) -> Result<A, E>
    where
        F: FnOnce(&E);

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
        F: FnOnce() -> Result<B, E>,
    {
        self.and_then(|_| f())
    }

    fn then_err<F, H>(self, f: F) -> Result<A, H>
    where
        F: FnOnce() -> Result<A, H>,
    {
        self.or_else(|_| f())
    }

    fn inspect<F>(self, f: F) -> Result<A, E>
    where
        F: FnOnce(&A),
    {
        self.map(|a| {
            f(&a);
            a
        })
    }

    fn inspect_err<F>(self, f: F) -> Result<A, E>
    where
        F: FnOnce(&E),
    {
        self.map_err(|e| {
            f(&e);
            e
        })
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

/// Ability to merge branches of a `Result<A, E>` when `A` and `E` are compatible (e.g. when they unify under an `Into<T>` conversion).
pub trait Merge<T> {
    /// Merges both branches of a result, giving preference to the `Ok` branch when needed.
    ///
    /// ```
    /// use lifterr::result::Merge;
    ///
    /// fn merge_ok() -> i32 {
    ///     Ok::<_, i32>(42).merge()
    /// }
    ///
    /// fn merge_err() -> i32 {
    ///     Err::<i32, _>(42).merge()
    /// }
    ///
    /// assert_eq!(merge_ok(), 42);
    /// assert_eq!(merge_err(), 42);
    /// ```
    fn merge(self) -> T;
}

impl<A, E, T> Merge<T> for Result<A, E>
where
    T: From<A> + From<E>,
{
    fn merge(self) -> T {
        self.map_or_else(T::from, T::from)
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
