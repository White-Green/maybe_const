# partial_const

This library provides a way to handle constant and non-constant values in a unified way.
This library takes both flexibilities to take dynamic values and (partial) parameter checking by the compiler.

## Example
```rust
fn maybe_one<T: partial_const::MayBeConst<i32>>(i: T) -> bool
    where T: partial_const::Equals<partial_const::ConstI32<1>> {
//  where T: partial_const::Equals<partial_const::i32::Const<1>> { <- Alternative
    i.value() == 1i32
}

assert!(maybe_one(partial_const::ConstI32::<1>::new()));
assert!(maybe_one::<partial_const::ConstI32<1>>(partial_const::ConstI32::new()));
assert!(maybe_one::<i32>(1));
assert!(!maybe_one::<i32>(2));
// assert!(maybe_one(partial_const::ConstI32::<2>::new())); <- Compile Error
```

License: MIT
