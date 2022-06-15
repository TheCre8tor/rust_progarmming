fn main() {
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
    */
}
