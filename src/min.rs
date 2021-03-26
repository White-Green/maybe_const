macro_rules! impl_min {
    ($t:tt, $($doc:expr),*) => {
        $(#[doc = $doc])*
        #[derive(Clone, Copy, Default, Debug)]
        pub struct Const<const VALUE: $t>(core::marker::PhantomData<Const<VALUE>>);

        impl<const VALUE: $t> Const<VALUE> {
            #[inline(always)]
            pub fn new() -> Self {
                Default::default()
            }
        }

        impl<const VALUE: $t> core::fmt::Display for Const<VALUE> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                VALUE.fmt(f)
            }
        }

        impl<const VALUE: $t> crate::MayBeConstAT for Const<VALUE> {
            type Type = $t;
            const IS_CONST: bool = true;
            #[inline(always)]
            fn value(&self) -> $t {
                VALUE
            }
        }

        impl<const VALUE: $t> crate::Equals<Const<VALUE>> for Const<VALUE> {}
        impl<const VALUE: $t> crate::Equals<$t> for Const<VALUE> {}
        impl<const VALUE: $t> crate::Equals<Const<VALUE>> for $t {}

        impl<const VALUE1: $t, const VALUE2: $t> core::cmp::PartialEq<Const<VALUE2>> for Const<VALUE1> {
            fn eq(&self, _: &Const<VALUE2>) -> bool { VALUE1 == VALUE2 }
        }
        impl<const VALUE: $t> core::cmp::PartialEq<$t> for Const<VALUE> {
            fn eq(&self, rhs: &$t) -> bool { VALUE == *rhs }
        }
        impl<const VALUE: $t> core::cmp::PartialEq<Const<VALUE>> for $t {
            fn eq(&self, _: &Const<VALUE>) -> bool { *self == VALUE }
        }

        impl<const VALUE1: $t, const VALUE2: $t> core::cmp::PartialOrd<Const<VALUE2>> for Const<VALUE1> {
            fn partial_cmp(&self, _: &Const<VALUE2>) -> Option<core::cmp::Ordering> { VALUE1.partial_cmp(&VALUE2) }
        }
        impl<const VALUE: $t> core::cmp::PartialOrd<$t> for Const<VALUE> {
            fn partial_cmp(&self, rhs: &$t) -> Option<core::cmp::Ordering> { VALUE.partial_cmp(rhs) }
        }
        impl<const VALUE: $t> core::cmp::PartialOrd<Const<VALUE>> for $t {
            fn partial_cmp(&self, _: &Const<VALUE>) -> Option<core::cmp::Ordering> { self.partial_cmp(&VALUE) }
        }

        #[cfg(test)]
        mod test_min {
            use super::*;
            #[test]
            fn test_equals() {
                fn test<A: crate::MayBeConst<$t>, B: crate::MayBeConst<$t>>() where A: crate::Equals<B> {}
                test::<Const<{0 as $t}>, Const<{0 as $t}>>();
                test::<Const<{0 as $t}>, $t>();
                test::<$t, Const<{0 as $t}>>();
                // test::<Const<{1 as $t}>, Const<{0 as $t}>>(); // <-Compile Error!
            }

            #[test]
            fn test_is_const() {
                assert!(<Const<{0 as $t}> as crate::MayBeConstAT>::IS_CONST);
            }

            #[test]
            fn test_partial_eq() {
                assert!(Const::<{0 as $t}>::new().eq(&Const::<{0 as $t}>::new()));
                assert!(Const::<{0 as $t}>::new().eq(&{0 as $t}));
                assert!({0 as $t}.eq(&Const::<{0 as $t}>::new()));
                assert!({0 as $t}.eq(&{0 as $t}));

                assert!(!Const::<{1 as $t}>::new().eq(&Const::<{0 as $t}>::new()));
                assert!(!Const::<{1 as $t}>::new().eq(&{0 as $t}));
                assert!(!{1 as $t}.eq(&Const::<{0 as $t}>::new()));
                assert!(!{1 as $t}.eq(&{0 as $t}));
            }

            #[test]
            fn test_partial_ord() {
                assert_eq!(Const::<{1 as $t}>::new().partial_cmp(&Const::<{1 as $t}>::new()), Some(core::cmp::Ordering::Equal));
                assert_eq!(Const::<{1 as $t}>::new().partial_cmp(&{1 as $t}), Some(core::cmp::Ordering::Equal));
                assert_eq!({1 as $t}.partial_cmp(&Const::<{1 as $t}>::new()), Some(core::cmp::Ordering::Equal));
                assert_eq!({1 as $t}.partial_cmp(&{1 as $t}), Some(core::cmp::Ordering::Equal));

                assert_eq!(Const::<{1 as $t}>::new().partial_cmp(&Const::<{0 as $t}>::new()), Some(core::cmp::Ordering::Greater));
                assert_eq!(Const::<{1 as $t}>::new().partial_cmp(&{0 as $t}), Some(core::cmp::Ordering::Greater));
                assert_eq!({1 as $t}.partial_cmp(&Const::<{0 as $t}>::new()), Some(core::cmp::Ordering::Greater));
                assert_eq!({1 as $t}.partial_cmp(&{0 as $t}), Some(core::cmp::Ordering::Greater));

                assert_eq!(Const::<{0 as $t}>::new().partial_cmp(&Const::<{1 as $t}>::new()), Some(core::cmp::Ordering::Less));
                assert_eq!(Const::<{0 as $t}>::new().partial_cmp(&{1 as $t}), Some(core::cmp::Ordering::Less));
                assert_eq!({0 as $t}.partial_cmp(&Const::<{1 as $t}>::new()), Some(core::cmp::Ordering::Less));
                assert_eq!({0 as $t}.partial_cmp(&{1 as $t}), Some(core::cmp::Ordering::Less));
            }
        }
    }
}

macro_rules! impl_min_out {
    ($c:tt, $t:tt, $($doc:expr),*) => {
        $(#[doc = $doc])*
        pub type $c<const VALUE:$t> = crate::$t::Const<VALUE>;
    }
}
