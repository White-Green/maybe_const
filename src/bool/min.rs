#[derive(Clone, Copy, Default, Debug)]
pub struct Const<const VALUE: bool>(core::marker::PhantomData<Const<VALUE>>);

impl<const VALUE: bool> Const<VALUE> {
    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }
}

impl<const VALUE: bool> core::fmt::Display for Const<VALUE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        VALUE.fmt(f)
    }
}

impl<const VALUE: bool> crate::MayBeConstAT for Const<VALUE> {
    type Type = bool;
    const IS_CONST: bool = true;
    #[inline(always)]
    fn value(&self) -> bool {
        VALUE
    }
}

impl<const VALUE: bool> crate::Equals<Const<VALUE>> for Const<VALUE> {
    type ConstSide = Const<VALUE>;
    #[inline(always)]
    fn get_const_side(&self, _: &Const<VALUE>) -> Option<Self::ConstSide> {
        Some(*self)
    }
}

impl<const VALUE: bool> crate::Equals<bool> for Const<VALUE> {
    type ConstSide = Const<VALUE>;
    #[inline(always)]
    fn get_const_side(&self, rhs: &bool) -> Option<Self::ConstSide> {
        if *self == *rhs {
            Some(*self)
        } else {
            None
        }
    }
}

impl<const VALUE: bool> crate::Equals<Const<VALUE>> for bool {
    type ConstSide = Const<VALUE>;
    #[inline(always)]
    fn get_const_side(&self, rhs: &Const<VALUE>) -> Option<Self::ConstSide> {
        if *self == *rhs {
            Some(*rhs)
        } else {
            None
        }
    }
}

impl<const VALUE1: bool, const VALUE2: bool> core::cmp::PartialEq<Const<VALUE2>> for Const<VALUE1> {
    fn eq(&self, _: &Const<VALUE2>) -> bool { VALUE1 == VALUE2 }
}

impl<const VALUE: bool> core::cmp::PartialEq<bool> for Const<VALUE> {
    fn eq(&self, rhs: &bool) -> bool { VALUE == *rhs }
}

impl<const VALUE: bool> core::cmp::PartialEq<Const<VALUE>> for bool {
    fn eq(&self, _: &Const<VALUE>) -> bool { *self == VALUE }
}

impl<const VALUE1: bool, const VALUE2: bool> core::cmp::PartialOrd<Const<VALUE2>> for Const<VALUE1> {
    fn partial_cmp(&self, _: &Const<VALUE2>) -> Option<core::cmp::Ordering> { VALUE1.partial_cmp(&VALUE2) }
}

impl<const VALUE: bool> core::cmp::PartialOrd<bool> for Const<VALUE> {
    fn partial_cmp(&self, rhs: &bool) -> Option<core::cmp::Ordering> { VALUE.partial_cmp(rhs) }
}

impl<const VALUE: bool> core::cmp::PartialOrd<Const<VALUE>> for bool {
    fn partial_cmp(&self, _: &Const<VALUE>) -> Option<core::cmp::Ordering> { self.partial_cmp(&VALUE) }
}

#[cfg(test)]
mod test_nightly {
    use super::*;

    #[test]
    fn test_equals() {
        fn test<A: crate::MayBeConst<bool>, B: crate::MayBeConst<bool>>() where A: crate::Equals<B> {}
        test::<Const<false>, Const<false>>();
        test::<Const<false>, bool>();
        test::<bool, Const<false>>();
        // test::<Const<{1 as bool}>, Const<{0 as bool}>>(); // <-Compile Error!
    }

    #[test]
    fn test_equals_result_const() {
        fn const_side_is_const<A: crate::MayBeConst<bool>, B: crate::MayBeConst<bool>>() -> bool where A: crate::Equals<B> {
            <<A as crate::Equals<B>>::ConstSide as crate::MayBeConstAT>::IS_CONST
        }
        assert!(const_side_is_const::<Const<false>, Const<false>>());
        assert!(const_side_is_const::<Const<false>, bool>());
        assert!(const_side_is_const::<bool, Const<false>>());

        use crate::Equals;
        assert_eq!(Const::<false>::new().get_const_side(&Const::<false>::new()), Some(Const::<false>::new()));
        assert_eq!(Const::<false>::new().get_const_side(&false), Some(Const::<false>::new()));
        assert_eq!(false.get_const_side(&Const::<false>::new()), Some(Const::<false>::new()));
        assert_eq!(false.get_const_side(&false), Some(false));

        // assert_eq!(Const::<true>::new().get_const_side(&Const::<false>::new()), None); // <-Compile Error!
        assert_eq!(Const::<true>::new().get_const_side(&false), None);
        assert_eq!(true.get_const_side(&Const::<false>::new()), None);
        assert_eq!(true.get_const_side(&false), None);
    }

    #[test]
    fn test_is_const() {
        assert!(<Const<false> as crate::MayBeConstAT>::IS_CONST);
    }

    #[test]
    fn test_partial_eq() {
        assert!(Const::<false>::new().eq(&Const::<false>::new()));
        assert!(Const::<false>::new().eq(&false));
        assert!(false.eq(&Const::<false>::new()));
        assert!(false.eq(&false));

        assert!(!Const::<true>::new().eq(&Const::<false>::new()));
        assert!(!Const::<true>::new().eq(&false));
        assert!(!true.eq(&Const::<false>::new()));
        assert!(!true.eq(&false));
    }

    #[test]
    fn test_partial_ord() {
        assert_eq!(Const::<true>::new().partial_cmp(&Const::<true>::new()), Some(core::cmp::Ordering::Equal));
        assert_eq!(Const::<true>::new().partial_cmp(&true), Some(core::cmp::Ordering::Equal));
        assert_eq!(true.partial_cmp(&Const::<true>::new()), Some(core::cmp::Ordering::Equal));
        assert_eq!(true.partial_cmp(&true), Some(core::cmp::Ordering::Equal));

        assert_eq!(Const::<true>::new().partial_cmp(&Const::<false>::new()), Some(core::cmp::Ordering::Greater));
        assert_eq!(Const::<true>::new().partial_cmp(&false), Some(core::cmp::Ordering::Greater));
        assert_eq!(true.partial_cmp(&Const::<false>::new()), Some(core::cmp::Ordering::Greater));
        assert_eq!(true.partial_cmp(&false), Some(core::cmp::Ordering::Greater));

        assert_eq!(Const::<false>::new().partial_cmp(&Const::<true>::new()), Some(core::cmp::Ordering::Less));
        assert_eq!(Const::<false>::new().partial_cmp(&true), Some(core::cmp::Ordering::Less));
        assert_eq!(false.partial_cmp(&Const::<true>::new()), Some(core::cmp::Ordering::Less));
        assert_eq!(false.partial_cmp(&true), Some(core::cmp::Ordering::Less));
    }
}
