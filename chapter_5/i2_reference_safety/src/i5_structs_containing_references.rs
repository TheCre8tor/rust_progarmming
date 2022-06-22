pub fn run() {
    /* How does Rust handle references stored
       in data structures?
    */

    // This does not compile.
    /* The safety constraints Rust places on references
       can’t magically disappear just because we hid the
       reference inside a struct.
    */
    // struct S {
    //     r: &i32, // missing lifet specifier
    // }

    /* NOTE:
       Whenever a reference type appears inside another
       type’s definition, you must write out its lifetime.
    */

    // This says that r can only refer to i32 values that
    // will last for the lifetime of the program, which is
    // rather limiting.

    // struct S {
    //     r: &'static i32,
    // }

    // The alternative is to give the type a lifetime parameter
    // 'a , and use that for r:
    struct S<'a> {
        r: &'a i32,
    }

    /* By taking a lifetime parameter 'a and using it in s’s type,
       we’ve allowed Rust to relate a T value’s lifetime to that of
       the reference its S holds.
    */
    struct T<'a> {
        s: S<'a>,
    }

    let s;

    {
        let x = 10;
        /* the expression S { r: &x } creates a fresh S value with
           some lifetime 'a . When you store &x in the r field, you
           constrain 'a to lie entirely within x’s lifetime.
        */
        s = S { r: &x }; // `x` does not live long enough.
    }

    // assert_eq!(*s.r, 10); // uncommenting this will cause an issue
}
