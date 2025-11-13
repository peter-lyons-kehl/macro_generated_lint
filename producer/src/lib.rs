#![cfg_attr(feature = "verify-on-nightly", feature(decl_macro))]

// Using rustdoc triple slash comments here, even though it's not really documentation, because otherwise
//
/// The macro defined with declarative macros 2.0 syntax has to be in a separate module that is
/// behind `#[cfg(...)]`. Otherwise, if it were defined here - **even** behind `#[cfg(feature =
/// "verify-on-nightly")]`, we get a warning even on **stable** like this:
/// ```ignore
/// ❯ cargo +stable check
/// warning: `macro` is experimental
/// --> src/lib.rs:4:1
///   |
/// 4 | / pub macro expect_only_unsafe( $e:expr ) {{
/// 5 | |     #[deny(unused_unsafe)]
/// 6 | |     const _: bool = unsafe { $e };
/// 7 | | }}
///   | |__^
///   |
///   = note: see issue #39412 <https://github.com/rust-lang/rust/issues/39412> for more information
///   = warning: unstable syntax can change at any point in the future, causing a hard error!
///   = note: for more information, see issue #65860 <https://github.com/rust-lang/rust/issues/65860>
/// ``````
/// See https://github.com/rust-lang/rust/issues/65860#issuecomment-547825815
#[cfg(feature = "verify-on-nightly")]
pub mod expect;

pub fn f() {
    //#[cfg(feature = "verify-on-nightly")]
    //expect::only_unsafe!(true)
}
