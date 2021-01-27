/// A trait for putting less-than constraints on constants.
/// It will be implemented if the constant is less than Rhs.
///
/// # Example
/// ```
/// # #[cfg(feature = "u8")] fn test() {
/// fn lt<A: partial_const::MayBeConst<u8>, B: partial_const::MayBeConst<u8>>(a: A, b: B) -> bool
///     where A: partial_const::Lt<B> {
///     a.value() < b.value()
/// }
///
/// assert!(lt(partial_const::ConstU8::<1>::new(), partial_const::ConstU8::<2>::new()));
/// assert!(lt(partial_const::ConstU8::<1>::new(), 2));
/// assert!(!lt(partial_const::ConstU8::<1>::new(), 1));
/// // assert!(lt(partial_const::ConstU8::<1>::new(), partial_const::ConstU8::<1>::new())); <- Compile Error
/// // assert!(lt(partial_const::ConstU8::<2>::new(), partial_const::ConstU8::<1>::new())); <- Compile Error
/// # }
/// # #[cfg(feature = "u8")] test();
/// ```
pub trait Lt<Rhs> {}

/// A trait for putting greater-than constraints on constants.
/// It will be implemented if the constant is greater than Rhs.
///
/// # Example
/// ```
/// # #[cfg(feature = "u8")] fn test() {
/// fn gt<A: partial_const::MayBeConst<u8>, B: partial_const::MayBeConst<u8>>(a: A, b: B) -> bool
///     where A: partial_const::Gt<B> {
///     a.value() > b.value()
/// }
///
/// assert!(gt(partial_const::ConstU8::<2>::new(), partial_const::ConstU8::<1>::new()));
/// assert!(gt(partial_const::ConstU8::<2>::new(), 1));
/// assert!(!gt(partial_const::ConstU8::<1>::new(), 1));
/// // assert!(gt(partial_const::ConstU8::<1>::new(), partial_const::ConstU8::<1>::new())); <- Compile Error
/// // assert!(gt(partial_const::ConstU8::<1>::new(), partial_const::ConstU8::<2>::new())); <- Compile Error
/// # }
/// # #[cfg(feature = "u8")] test();
/// ```
pub trait Gt<Rhs> {}

/// A trait for putting less-than-or-equal constraints on constants.
/// It will be implemented if the constant is less than or equal Rhs.
///
/// # Example
/// ```
/// # #[cfg(feature = "u8")] fn test() {
/// fn leq<A: partial_const::MayBeConst<u8>, B: partial_const::MayBeConst<u8>>(a: A, b: B) -> bool
///     where A: partial_const::Leq<B> {
///     a.value() <= b.value()
/// }
///
/// assert!(leq(partial_const::ConstU8::<1>::new(), partial_const::ConstU8::<2>::new()));
/// assert!(leq(partial_const::ConstU8::<1>::new(), partial_const::ConstU8::<1>::new()));
/// assert!(leq(partial_const::ConstU8::<1>::new(), 1));
/// assert!(!leq(partial_const::ConstU8::<2>::new(), 1));
/// // assert!(leq(partial_const::ConstU8::<2>::new(), partial_const::ConstU8::<1>::new())); <- Compile Error
/// # }
/// # #[cfg(feature = "u8")] test();
/// ```
pub trait Leq<Rhs> {}

/// A trait for putting greater-than-or-equal constraints on constants.
/// It will be implemented if the constant is greater than or equal Rhs.
///
/// # Example
/// ```
/// # #[cfg(feature = "u8")] fn test() {
/// fn geq<A: partial_const::MayBeConst<u8>, B: partial_const::MayBeConst<u8>>(a: A, b: B) -> bool
///     where A: partial_const::Geq<B> {
///     a.value() >= b.value()
/// }
///
/// assert!(geq(partial_const::ConstU8::<2>::new(), partial_const::ConstU8::<1>::new()));
/// assert!(geq(partial_const::ConstU8::<1>::new(), partial_const::ConstU8::<1>::new()));
/// assert!(geq(partial_const::ConstU8::<1>::new(), 1));
/// assert!(!geq(partial_const::ConstU8::<1>::new(), 2));
/// // assert!(geq(partial_const::ConstU8::<1>::new(), partial_const::ConstU8::<2>::new())); //<- Compile Error
/// # }
/// # #[cfg(feature = "u8")] test();
/// ```
pub trait Geq<Rhs> {}

macro_rules! impl_incomplete {
    ($t:tt) => {
        const fn lt(a: $t, b: $t) -> usize {
            assert!(a < b);
            0
        }
        impl<const VALUE1: $t, const VALUE2: $t> crate::Lt<Const<VALUE2>> for Const<VALUE1> where [(); lt(VALUE1, VALUE2)]: Sized {}
        impl<const VALUE: $t> crate::Lt<$t> for Const<VALUE> {}
        impl<const VALUE: $t> crate::Lt<Const<VALUE>> for $t {}
        impl crate::Lt<$t> for $t {}

        const fn gt(a: $t, b: $t) -> usize {
            assert!(a > b);
            0
        }
        impl<const VALUE1: $t, const VALUE2: $t> crate::Gt<Const<VALUE2>> for Const<VALUE1> where [(); gt(VALUE1, VALUE2)]: Sized {}
        impl<const VALUE: $t> crate::Gt<$t> for Const<VALUE> {}
        impl<const VALUE: $t> crate::Gt<Const<VALUE>> for $t {}
        impl crate::Gt<$t> for $t {}

        const fn leq(a: $t, b: $t) -> usize {
            assert!(a <= b);
            0
        }
        impl<const VALUE1: $t, const VALUE2: $t> crate::Leq<Const<VALUE2>> for Const<VALUE1> where [(); leq(VALUE1, VALUE2)]: Sized {}
        impl<const VALUE: $t> crate::Leq<$t> for Const<VALUE> {}
        impl<const VALUE: $t> crate::Leq<Const<VALUE>> for $t {}
        impl crate::Leq<$t> for $t {}

        const fn geq(a: $t, b: $t) -> usize {
            assert!(a >= b);
            0
        }
        impl<const VALUE1: $t, const VALUE2: $t> crate::Geq<Const<VALUE2>> for Const<VALUE1> where [(); geq(VALUE1, VALUE2)]: Sized {}
        impl<const VALUE: $t> crate::Geq<$t> for Const<VALUE> {}
        impl<const VALUE: $t> crate::Geq<Const<VALUE>> for $t {}
        impl crate::Geq<$t> for $t {}

        #[cfg(test)]
        mod test_incomplete {
            use super::*;
            #[test]
            fn test_lt() {
                fn test<A: crate::MayBeConst<$t>, B: crate::MayBeConst<$t>>() where A: crate::Lt<B> {}
                test::<Const<{1 as $t}>, Const<{2 as $t}>>();
                test::<Const<{1 as $t}>, $t>();
                test::<$t, Const<{1 as $t}>>();
                test::<$t, $t>();
                // test::<Const<{1 as $t}>, Const<{1 as $t}>>(); // <-Compile Error!
                // test::<Const<{2 as $t}>, Const<{1 as $t}>>(); // <-Compile Error!
            }

            #[test]
            fn test_gt() {
                fn test<A: crate::MayBeConst<$t>, B: crate::MayBeConst<$t>>() where A: crate::Gt<B> {}
                test::<Const<{2 as $t}>, Const<{1 as $t}>>();
                test::<Const<{1 as $t}>, $t>();
                test::<$t, Const<{1 as $t}>>();
                test::<$t, $t>();
                // test::<Const<{1 as $t}>, Const<{1 as $t}>>(); // <-Compile Error!
                // test::<Const<{1 as $t}>, Const<{2 as $t}>>(); // <-Compile Error!
            }

            #[test]
            fn test_leq() {
                fn test<A: crate::MayBeConst<$t>, B: crate::MayBeConst<$t>>() where A: crate::Leq<B> {}
                test::<Const<{1 as $t}>, Const<{1 as $t}>>();
                test::<Const<{1 as $t}>, Const<{2 as $t}>>();
                test::<Const<{1 as $t}>, $t>();
                test::<$t, Const<{1 as $t}>>();
                test::<$t, $t>();
                // test::<Const<{2 as $t}>, Const<{1 as $t}>>(); // <-Compile Error!
            }

            #[test]
            fn test_geq() {
                fn test<A: crate::MayBeConst<$t>, B: crate::MayBeConst<$t>>() where A: crate::Geq<B> {}
                test::<Const<{1 as $t}>, Const<{1 as $t}>>();
                test::<Const<{2 as $t}>, Const<{1 as $t}>>();
                test::<Const<{1 as $t}>, $t>();
                test::<$t, Const<{1 as $t}>>();
                test::<$t, $t>();
                // test::<Const<{1 as $t}>, Const<{2 as $t}>>(); // <-Compile Error!
            }
        }
    }
}
