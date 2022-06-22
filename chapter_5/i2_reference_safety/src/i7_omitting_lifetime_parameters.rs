pub fn run() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    /* In the simplest case, if your function doesn’t return
       any references (or other types that require lifetime
       parameters), then you never need to write out lifetimes
       for your parameters.
    */
    fn sum_r_xy(r: &i32, s: S) -> i32 {
        r + s.x + s.y
    }

    // This function’s signature is shorthand for:
    fn sum_s_xy<'a, 'b, 'c>(r: &'a i32, s: S<'b, 'c>) -> i32 {
        r + s.x + s.y
    }

    /* If there’s only a single lifetime that appears among your
       function’s parameters, then Rust assumes any lifetimes in
       your return value must be that one:
    */
    fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
        (&point[0], &point[2])
    }

    // With all the lifetimes written out, the equivalent would be:
    fn first_third_expand<'a>(point: &'a [i32; 3]) -> (&'a i32, &'a i32) {
        (&point[0], &point[2])
    }
}
