/// A trait for handling constant and non-constant values in a common way
///
/// # Example
/// ```
/// # #[cfg(feature = "usize")] #[rustversion::since(1.51)] fn test() {
/// fn twice<T: partial_const::MayBeConst<usize>>(i: T) -> usize {
///     i.value() * 2
/// }
///
/// assert_eq!(twice(1usize), 2usize);
/// assert_eq!(twice(partial_const::ConstUsize::<1>::new()), 2usize);
/// # }
/// # #[cfg(not(feature = "usize"))] fn test(){}
/// # #[cfg(feature = "usize")] #[rustversion::not(since(1.51))] fn test(){}
/// # test();
/// ```
pub trait MayBeConst<T>: MayBeConstAT<Type=T> {}

/// A trait [MayBeConst] by associated type for internal trait bounds.
pub trait MayBeConstAT: Sized + Clone + Copy + Default + core::fmt::Debug + core::fmt::Display {
    type Type: MayBeConstAT<Type=Self::Type>;
    const IS_CONST: bool;
    fn value(&self) -> Self::Type;
}

impl<T: MayBeConstAT> MayBeConst<T::Type> for T {}

/// A trait for putting equality constraints on constants.
/// It will be implemented if the constants are equal.
///
/// # Example
/// ```
/// # #[cfg(feature = "u8")] #[rustversion::since(1.51)] fn test() {
/// fn equal<A: partial_const::MayBeConst<u8>, B: partial_const::MayBeConst<u8>>(a: A, b: B) -> bool
///     where A: partial_const::Equals<B> {
///     a.value() == b.value()
/// }
///
/// assert!(equal(partial_const::ConstU8::<1>::new(), partial_const::ConstU8::<1>::new()));
/// assert!(equal(partial_const::ConstU8::<1>::new(), 1));
/// assert!(!equal(partial_const::ConstU8::<1>::new(), 2));
/// // assert!(equal(partial_const::ConstU8::<1>::new(), partial_const::ConstU8::<2>::new())); <- Compile Error
/// # }
/// # #[cfg(not(feature = "u8"))] fn test(){}
/// # #[cfg(feature = "u8")] #[rustversion::not(since(1.51))] fn test(){}
/// # test();
/// ```
pub trait Equals<T: MayBeConstAT>: MayBeConstAT<Type=T::Type> {}

macro_rules! impl_stable {
    ($t:tt) => {
        impl crate::MayBeConstAT for $t {
            type Type = $t;
            const IS_CONST: bool = false;
            #[inline(always)]
            fn value(&self) -> $t {
                *self
            }
        }

        impl crate::Equals<$t> for $t {}

        #[cfg(test)]
        mod test_stable {
            #[test]
            fn test_equals() {
                fn test<A: crate::MayBeConst<$t>, B: crate::MayBeConst<$t>>() where A: crate::Equals<B> {}
                test::<$t, $t>();
            }

            #[test]
            fn test_is_const() {
                assert!(!<$t as crate::MayBeConstAT>::IS_CONST);
            }
        }
    }
}
