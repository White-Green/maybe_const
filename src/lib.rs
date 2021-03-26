//! This library provides a way to handle constant and non-constant values in a unified way.
//! This library takes both flexibilities to take dynamic values and (partial) parameter checking by the compiler.
//!
//! # Example
//! ```
//! # #[cfg(feature = "i32")] #[rustversion::since(1.51)] fn test() {
//! fn maybe_one<T: partial_const::MayBeConst<i32>>(i: T) -> bool
//!     where T: partial_const::Equals<partial_const::ConstI32<1>> {
//! //  where T: partial_const::Equals<partial_const::i32::Const<1>> { <- Alternative
//!     i.value() == 1i32
//! }
//!
//! assert!(maybe_one(partial_const::ConstI32::<1>::new()));
//! assert!(maybe_one::<partial_const::ConstI32<1>>(partial_const::ConstI32::new()));
//! assert!(maybe_one::<i32>(1));
//! assert!(!maybe_one::<i32>(2));
//! // assert!(maybe_one(partial_const::ConstI32::<2>::new())); <- Compile Error
//! # }
//! # #[cfg(not(feature = "i32"))] fn test(){}
//! # #[cfg(feature = "i32")] #[rustversion::not(since(1.51))] fn test(){}
//! # test();
//! ```
#![cfg_attr(feature = "incomplete", allow(incomplete_features))]
#![cfg_attr(feature = "incomplete", feature(const_generics))]
#![cfg_attr(feature = "incomplete", feature(const_evaluatable_checked))]
#![cfg_attr(feature = "incomplete", feature(const_panic))]

#[cfg(feature = "bool")]
/// module for [prim@bool]
pub mod bool;

#[cfg(feature = "bool")]
#[rustversion::since(1.51)]
/// See [struct@bool::Const]
pub type ConstBool<const VALUE: bool> = crate::bool::Const<VALUE>;

include!("stable.rs");
#[rustversion::since(1.51)]
include!("min.rs");
#[rustversion::all(since(1.51), nightly)]
#[cfg(feature = "incomplete")]
include!("incomplete.rs");

macro_rules! expand_impl {
    ($c:tt, $t:tt, $($doc1:expr),*;$($doc2:expr),*;$($doc3:expr),*) => {
        $(#[doc=$doc1])*
        pub mod $t {
            impl_stable!($t);
            #[rustversion::since(1.51)]
            impl_min!($t, $($doc3),*);
            #[rustversion::all(since(1.51), nightly)]
            #[cfg(feature = "incomplete")]
            impl_incomplete!($t);
        }
        #[rustversion::since(1.51)]
        impl_min_out!($c, $t, $($doc2),*);
    };
}
#[cfg(feature = "usize")]
expand_impl!(ConstUsize, usize, "module for [prim@usize]"; "See [struct@usize::Const]";
    "Constant [prim@usize] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::usize::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::usize::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "isize")]
expand_impl!(ConstIsize, isize, "module for [prim@isize]"; "See [struct@isize::Const]";
    "Constant [prim@isize] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::isize::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::isize::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "i8")]
expand_impl!(ConstI8, i8, "module for [prim@i8]"; "See [struct@i8::Const]";
    "Constant [prim@i8] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::i8::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::i8::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "i16")]
expand_impl!(ConstI16, i16, "module for [prim@i16]"; "See [struct@i16::Const]";
    "Constant [prim@i16] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::i16::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::i16::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "i32")]
expand_impl!(ConstI32, i32, "module for [prim@i32]"; "See [struct@i32::Const]";
    "Constant [prim@i32] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::i32::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::i32::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "i64")]
expand_impl!(ConstI64, i64, "module for [prim@i64]"; "See [struct@i64::Const]";
    "Constant [prim@i64] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::i64::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::i64::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "i128")]
expand_impl!(ConstI128, i128, "module for [prim@i128]"; "See [struct@i128::Const]";
    "Constant [prim@i128] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::i128::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::i128::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "u8")]
expand_impl!(ConstU8, u8, "module for [prim@u8]"; "See [struct@u8::Const]";
    "Constant [prim@u8] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::u8::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::u8::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "u16")]
expand_impl!(ConstU16, u16, "module for [prim@u16]"; "See [struct@u16::Const]";
    "Constant [prim@u16] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::u16::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::u16::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "u32")]
expand_impl!(ConstU32, u32, "module for [prim@u32]"; "See [struct@u32::Const]";
    "Constant [prim@u32] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::u32::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::u32::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "u64")]
expand_impl!(ConstU64, u64, "module for [prim@u64]"; "See [struct@u64::Const]";
    "Constant [prim@u64] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::u64::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::u64::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "u128")]
expand_impl!(ConstU128, u128, "module for [prim@u128]"; "See [struct@u128::Const]";
    "Constant [prim@u128] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::u128::Const::<42>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::u128::Const<42>>(), 0);",
    "assert_eq!(const_value.value(), 42);",
    "```");
#[cfg(feature = "char")]
expand_impl!(ConstChar, char, "module for [prim@char]"; "See [struct@char::Const]";
    "Constant [prim@char] value",
    "",
    "# Example",
    "```",
    "# use partial_const::*;",
    "let const_value = partial_const::char::Const::<'A'>::new();",
    "assert_eq!(core::mem::size_of::<partial_const::char::Const<'A'>>(), 0);",
    "assert_eq!(const_value.value(), 'A');",
    "```");
