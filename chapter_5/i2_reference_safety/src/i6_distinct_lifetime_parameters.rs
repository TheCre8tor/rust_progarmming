pub fn run() {
    /* Suppose you’ve defined a structure containing
       two references like this:
    */
    struct S<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    /* Both references use the same lifetime 'a.
       This could be a problem if your code if you
       wants to do something like this:
    */

    let x = 10;
    let r: &i32;

    {
        let y = 20;

        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }

    assert_eq!(*r, 10);

    /* The problem arises because both references
       in S have the same lifetime 'a. Changing the
       definition of S to let each reference have a
       distinct lifetime fixes everything:
    */

    struct SX<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    let u = 10;
    let k: &i32;

    {
        let i = 20;

        {
            let f = SX { x: &u, y: &i };
            k = f.x;
        }
    }

    assert_eq!(*k, u);
}

/* Function signatures can have similar effects.
   Suppose we have a function like this:
*/
fn f<'a>(r: &'a i32, _s: &'a i32) -> &'a i32 {
    r
} // perhaps too tight ->

fn x<'a, 'b>(r: &'a i32, _s: &'b i32) -> &'a i32 {
    r
} // better ->
