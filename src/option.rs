//! Extra utilities for handling optionality.
//!
//! This module extends capabilities offered by [`std::option`].

/// Extension with a set of extra combinators for `Option<A>`.
pub trait OptionExt<A> {
    /// Applies `f` yielding yet another option if `Some(x)` otherwise propagates `None`.
    ///
    /// ```
    /// use lifterr::option::OptionExt;
    ///
    /// fn some() -> Option<i32> { Some(1) }
    /// fn none() -> Option<i32> { None }
    ///
    /// assert_eq!(some().then(|| Some("42")), Some("42"));
    /// assert_eq!(none().then(|| Some("42")), None);
    /// ```
    fn then<F, B>(self, f: F) -> Option<B>
    where
        F: FnOnce() -> Option<B>;

    /// Applies `f` yielding a value which is then wrapped into another option if `Some(x)` otherwise propagates `None`.
    ///
    /// ```
    /// use lifterr::option::OptionExt;
    ///
    /// fn some() -> Option<i32> { Some(1) }
    /// fn none() -> Option<i32> { None }
    ///
    /// assert_eq!(some().remap(|| "42"), Some("42"));
    /// assert_eq!(none().remap(|| "42"), None);
    /// ```
    fn remap<F, B>(self, f: F) -> Option<B>
    where
        Self: Sized,
        F: FnOnce() -> B,
    {
        self.then(|| f().into())
    }

    /// Replaces whatever value of type `A` in `Option<A>` with an unit.
    fn void(self) -> Option<()>
    where
        Self: Sized,
    {
        self.remap(|| ())
    }

    /// Runs `f` with a reference to `A` when `Some(a)`.
    ///
    /// ```
    /// use lifterr::option::OptionExt;
    /// assert_eq!(Some(10).inspect(|a| println!("a = {a}")), Some(10));
    /// ```
    fn inspect<F>(self, f: F) -> Option<A>
    where
        F: FnOnce(&A);

    /// Recovers from an absent value with a total function.
    fn recover<F>(self, f: F) -> Option<A>
    where
        F: FnOnce() -> A,
        Self: Sized,
    {
        self.recover_with(|| f().into())
    }

    /// Recovers from an absent value with a partial function.
    ///
    /// ```
    /// use lifterr::option::OptionExt;
    ///
    /// fn not_found() -> Option<i32> { None }
    /// fn fallback() -> Option<i32> { Some(42) }
    ///
    /// assert_eq!(Some(10).recover_with(fallback), Some(10));
    /// assert_eq!(not_found().recover_with(fallback), Some(42));
    /// ```
    fn recover_with<F>(self, f: F) -> Option<A>
    where
        F: FnOnce() -> Option<A>;
}

impl<A> OptionExt<A> for Option<A> {
    fn then<F, B>(self, f: F) -> Option<B>
    where
        F: FnOnce() -> Option<B>,
    {
        self.and_then(|_| f())
    }

    fn inspect<F>(self, f: F) -> Option<A>
    where
        F: FnOnce(&A),
    {
        self.map(|a| {
            f(&a);
            a
        })
    }

    fn recover_with<F>(self, f: F) -> Option<A>
    where
        F: FnOnce() -> Option<A>,
    {
        self.map_or_else(f, A::into)
    }
}
