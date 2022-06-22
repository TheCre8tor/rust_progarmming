fn main() {
    // TODO: Come back for this...
    /* There are ways to introduce dangling pointers
       in rust. Here is an easy case:
    */

    /* Throughout its lifetime, a shared reference makes its
       referent read-only: you may not assign to the referent
       or move its value elsewhere. In this code, r ’s lifetime
       contains the attempt to move the vector, so Rust rejects
       the program.

       The assignment to aside moves the vector, leaving
       v uninitialized, turning r into a dangling pointer
    */
    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v; // `r` become a dangling pointer
    let aside = v; // move vector to aside
    r[0];

    // If you change the program as shown here, there’s no problem:
    let vec = vec![4, 8, 19, 27, 34, 10];

    {
        let r = &vec;
        let _pick = r[0]; // ok: vector is still there

        /* In this version, r goes out of scope earlier, the
           reference’s lifetime ends before v is moved aside,
           and all is well.
        */
    }

    let _asides = vec;
}
