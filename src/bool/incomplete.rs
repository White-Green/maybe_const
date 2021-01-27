const fn lt(a: bool, b: bool) -> usize {
    assert!(!a && b);
    0
}

impl<const VALUE1: bool, const VALUE2: bool> crate::Lt<Const<VALUE2>> for Const<VALUE1> where [(); lt(VALUE1, VALUE2)]: Sized {}

impl<const VALUE: bool> crate::Lt<bool> for Const<VALUE> {}

impl<const VALUE: bool> crate::Lt<Const<VALUE>> for bool {}

impl crate::Lt<bool> for bool {}

const fn gt(a: bool, b: bool) -> usize {
    assert!(a && !b);
    0
}

impl<const VALUE1: bool, const VALUE2: bool> crate::Gt<Const<VALUE2>> for Const<VALUE1> where [(); gt(VALUE1, VALUE2)]: Sized {}

impl<const VALUE: bool> crate::Gt<bool> for Const<VALUE> {}

impl<const VALUE: bool> crate::Gt<Const<VALUE>> for bool {}

impl crate::Gt<bool> for bool {}

const fn leq(a: bool, b: bool) -> usize {
    assert!(a <= b);
    0
}

impl<const VALUE1: bool, const VALUE2: bool> crate::Leq<Const<VALUE2>> for Const<VALUE1> where [(); leq(VALUE1, VALUE2)]: Sized {}

impl<const VALUE: bool> crate::Leq<bool> for Const<VALUE> {}

impl<const VALUE: bool> crate::Leq<Const<VALUE>> for bool {}

impl crate::Leq<bool> for bool {}

const fn geq(a: bool, b: bool) -> usize {
    assert!(a >= b);
    0
}

impl<const VALUE1: bool, const VALUE2: bool> crate::Geq<Const<VALUE2>> for Const<VALUE1> where [(); geq(VALUE1, VALUE2)]: Sized {}

impl<const VALUE: bool> crate::Geq<bool> for Const<VALUE> {}

impl<const VALUE: bool> crate::Geq<Const<VALUE>> for bool {}

impl crate::Geq<bool> for bool {}

#[cfg(test)]
mod test_incomplete {
    use super::*;

    #[test]
    fn test_lt() {
        fn test<A: crate::MayBeConst<bool>, B: crate::MayBeConst<bool>>() where A: crate::Lt<B> {}
        test::<Const<false>, Const<true>>();
        test::<Const<false>, bool>();
        test::<bool, Const<false>>();
        test::<bool, bool>();
        // test::<Const<false>, Const<false>>(); // <-Compile Error!
        // test::<Const<true>, Const<false>>(); // <-Compile Error!
    }

    #[test]
    fn test_gt() {
        fn test<A: crate::MayBeConst<bool>, B: crate::MayBeConst<bool>>() where A: crate::Gt<B> {}
        test::<Const<true>, Const<false>>();
        test::<Const<false>, bool>();
        test::<bool, Const<false>>();
        test::<bool, bool>();
        // test::<Const<false>, Const<false>>(); // <-Compile Error!
        // test::<Const<false>, Const<true>>(); // <-Compile Error!
    }

    #[test]
    fn test_leq() {
        fn test<A: crate::MayBeConst<bool>, B: crate::MayBeConst<bool>>() where A: crate::Leq<B> {}
        test::<Const<false>, Const<false>>();
        test::<Const<false>, Const<true>>();
        test::<Const<false>, bool>();
        test::<bool, Const<false>>();
        test::<bool, bool>();
        // test::<Const<true>, Const<false>>(); // <-Compile Error!
    }

    #[test]
    fn test_geq() {
        fn test<A: crate::MayBeConst<bool>, B: crate::MayBeConst<bool>>() where A: crate::Geq<B> {}
        test::<Const<false>, Const<false>>();
        test::<Const<true>, Const<false>>();
        test::<Const<false>, bool>();
        test::<bool, Const<false>>();
        test::<bool, bool>();
        // test::<Const<false>, Const<true>>(); // <-Compile Error!
    }
}
