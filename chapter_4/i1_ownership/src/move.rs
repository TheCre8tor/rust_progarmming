pub fn run() {
    /* In Rust, for most types, operations like assigning
       a value to a variable, passing it to a function, or
       returning it from a function don’t copy the value:
       they move it.
    */

    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let _t = s;
    // let u = s; // use of moved value: `s` value used here after move

    /* If you want to end up in the same state as the C++
       program, with each variable holding an independent
       copy of the structure, you must call the vector’s
       clone method, which performs a deep copy of the
       vector and its elements
    */

    let p = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let _f = p.clone();
    let _z = p.clone();

    // LECTURE 2: More Operations That Move
}
