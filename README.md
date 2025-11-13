Workaround [rust-lang/rust#110613](https://github.com/rust-lang/rust/issues/110613). This intentionally fails to build.

Requires `nightly`:
- `cd producer`
- `cargo +nightly check`
- `cd ../consumer`
- `cargo +nightly check`

No doctest example yet.
