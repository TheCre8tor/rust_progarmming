pub fn run() {
    /*
        Three types of pointer types in Rust:
           1. References
           2. Boxes
           3. Unsafe Pointers

        1. References: References are Rust basic pointer type.
           A References can point to any value anywhere, stack
           or heap.

           &x -> borrows a reference to x
           *x -> is the value x points to.

           Rust track the ownership and lifetime of values, so
           mistakes like dangling pointers, double frees, and
           pointer invalidation are ruled out at compile time.

        2. Boxes: Box::new() is the simplest way to allocate a
           value on the heap.

        3. Raw Pointers: Rust also has the raw pointer types
           *mut T and *const T. Raw pointers really are just
           like pointers in C++
    */

    easy_allocate_values();
}

fn easy_allocate_values() {
    let stuffs = (12, "eggs");

    // I allocate memory on the heap and then
    // places stuffs of type turple into it.
    let alloc = Box::new(stuffs);

    println!("{:?}", alloc);
}
