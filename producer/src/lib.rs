#![feature(decl_macro)]

pub macro generate_unused_unsafe( $e:expr ) {{
    #[deny(unused_unsafe)]
    const _: bool = unsafe { $e };
}}

pub fn f() {
    generate_unused_unsafe!(true)
}
