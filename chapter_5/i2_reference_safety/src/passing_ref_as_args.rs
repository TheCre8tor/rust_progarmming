pub fn run() {
    let x = 10;

    /* From g’s signature alone, Rust knows it will not save `p`
       anywhere that might outlive the call: any lifetime that
       encloses the call must work for 'a.
       So Rust chooses the smallest possible lifetime for &x: that
       of the call to g. This meets all constraints: it doesn’t
       outlive x, and encloses the entire call to g.

       Although g takes a lifetime parameter 'a, we didn’t need
       to mention it when calling g. You only need to worry about
       lifetime parameters when defining functions and types; when
       using them, Rust infers the lifetimes for you.
    */

    g(&x);

    /* This fails to compile: the reference &x must not outlive x,
       but by passing it to f, we constrain it to live at least as
       long as 'static.
    */
    // f(&x);
}

fn g<'a>(_p: &'a i32) {}

fn _f(_p: &'static i32) {}
