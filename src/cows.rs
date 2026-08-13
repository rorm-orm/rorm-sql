//! Alternate [`Cow`]s which are covariant over their second argument

use std::borrow::Cow;
use std::ops::Deref;

/// [`Cow<'a, [T]>`](Cow) which is covariant over `T`.
#[derive(Clone, Debug, PartialEq)]
pub enum VecCow<'a, T> {
    /// Borrowed data
    Borrowed(&'a [T]),

    /// Owned data
    Owned(Vec<T>),
}

impl<'a, T> Deref for VecCow<'a, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        match self {
            VecCow::Borrowed(slice) => slice,
            VecCow::Owned(vec) => vec,
        }
    }
}

impl<'a, T> From<Cow<'a, [T]>> for VecCow<'a, T>
where
    T: Clone,
{
    fn from(value: Cow<'a, [T]>) -> Self {
        match value {
            Cow::Borrowed(x) => Self::Borrowed(x),
            Cow::Owned(x) => Self::Owned(x),
        }
    }
}

impl<'a, T> From<&'a [T]> for VecCow<'a, T> {
    fn from(value: &'a [T]) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a, T> From<Vec<T>> for VecCow<'a, T> {
    fn from(value: Vec<T>) -> Self {
        Self::Owned(value)
    }
}

/// `Cow<'a, T>` which is covariant over `T`.
#[derive(Clone, Debug, PartialEq)]
pub enum RefCow<'a, T> {
    /// Borrowed data
    Borrowed(&'a T),

    /// Owned data
    Owned(T),
}

impl<'a, T> Deref for RefCow<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Self::Borrowed(x) => x,
            Self::Owned(x) => x,
        }
    }
}

impl<'a, T> From<Cow<'a, T>> for RefCow<'a, T>
where
    T: Clone,
{
    fn from(value: Cow<'a, T>) -> Self {
        match value {
            Cow::Borrowed(x) => Self::Borrowed(x),
            Cow::Owned(x) => Self::Owned(x),
        }
    }
}

impl<'a, T> From<&'a T> for RefCow<'a, T> {
    fn from(value: &'a T) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a, T> From<T> for RefCow<'a, T> {
    fn from(value: T) -> Self {
        Self::Owned(value)
    }
}
