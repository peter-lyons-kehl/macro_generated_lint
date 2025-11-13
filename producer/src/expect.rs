pub macro only_unsafe( $e:expr ) {{
    #[deny(unused_unsafe)]
    const _: bool = unsafe { $e };
}}
