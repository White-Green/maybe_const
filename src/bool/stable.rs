impl crate::MayBeConstAT for bool {
    type Type = bool;
    const IS_CONST: bool = false;
    #[inline(always)]
    fn value(&self) -> bool {
        *self
    }
}

impl crate::Equals<bool> for bool {}

#[cfg(test)]
mod test_stable {
    #[test]
    fn test_equals() {
        fn test<A: crate::MayBeConst<bool>, B: crate::MayBeConst<bool>>() where A: crate::Equals<B> {}
        test::<bool, bool>();
    }

    #[test]
    fn test_is_const() {
        assert!(!<bool as crate::MayBeConstAT>::IS_CONST);
    }
}
