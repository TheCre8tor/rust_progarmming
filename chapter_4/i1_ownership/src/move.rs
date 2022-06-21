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
    /* *****************************************
       Assigning to a variable is slightly different, in that
       if you move a value into a variable that was already
       initialized, Rust drops the variable’s prior value.
    */

    /* In this code, when the program assigns the string
       "Siddhartha" to s , its prior value "Govinda" gets
       dropped first.
    */

    let mut _s = "Govinda".to_string();
    _s = "Siddhartha".to_string(); // value "Govinda" dropped here

    /* But consider the following:
       This time, `o` has taken ownership of the original string
       from t , so that by the time we assign to t , it is
       uninitialized. In this scenario, no string is dropped.

       NOTE: Moves always apply to the value proper, not the heap
             storage they own.
    */
    let mut t = "Govinda".to_string();
    let o = t;
    t = "Siddhartha".to_string(); // nothing is dropped here.

    println!("t: {}", t);
    println!("o: {}", o);
}
