impl crate::MayBeConstAT for bool {
    type Type = bool;
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
}
