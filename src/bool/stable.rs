impl crate::MayBeConstAT for bool {
    type Type = bool;
    const IS_CONST: bool = false;
    #[inline(always)]
    fn value(&self) -> bool {
        *self
    }
}

impl crate::Equals<bool> for bool {
    type ConstSide = bool;
    fn get_const_side(&self, rhs: &bool) -> Option<Self::ConstSide> {
        if *self == *rhs {
            Some(*self)
        } else {
            None
        }
    }
}

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
