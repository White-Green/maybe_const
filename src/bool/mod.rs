include!("stable.rs");
#[rustversion::since(1.51)]
include!("min.rs");
#[rustversion::all(since(1.51), nightly)]
#[cfg(feature = "incomplete")]
include!("incomplete.rs");
