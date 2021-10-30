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
        F: Fn() -> Option<B>;

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
        F: Fn() -> B,
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
}

impl<A> OptionExt<A> for Option<A> {
    fn then<F, B>(self, f: F) -> Option<B>
    where
        F: Fn() -> Option<B>,
    {
        self.and_then(|_| f())
    }
}
