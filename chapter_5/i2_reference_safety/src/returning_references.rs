pub fn run() {
    let v = vec![6, 4, 2, 1, 10];

    let result = smallest(&v);

    println!("{}", result);

    /* When a function takes a single reference as an argument,
       and returns a single reference, Rust assumes that the two
       must have the same lifetime.
    */
    let _s: &i32;

    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        _s = smallest_with_lifetime(&parabola);
    } // parabola was dropped here.

    // assert_eq!(*s, 0); // bad: points to element of dropped array

    /* FIX:
       Moving s so that its lifetime is clearly contained within
       parabola ’s fixes the problem:
    */
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let x = smallest_with_lifetime(&parabola);
        assert_eq!(*x, 0);
    }
}

// v should have at least one element.
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];

    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }

    s
}

fn smallest_with_lifetime<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];

    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }

    s
}
