#[path = "./array.rs"]
mod array;

#[path = "./vector.rs"]
mod vector;

#[path = "./slices.rs"]
mod slices;

#[path = "./string_type.rs"]
mod string_type;

fn main() {
    array::run();
    vector::run();
    slices::run();
    string_type::run();

    /*
        Rust has three types for representing a
        sequence of values in memory:

        1. Array Type
        2. Vector Type
        3. Slices Type

        1. Array Type: [T; N] represent an array of N values,
           each of the type T.
           It is not growable and you can't append new elements
           or shrink it.

        2. Vector Type: Vec<T> represent a vector of Ts.
           It is a dynamically allocated, growable sequence of
           values of type T.

        3. Slice Type: &[T] and &mut [T]

           &[T] -> Shared slice of Ts.
           &mut [T] -> Mutable slice of Ts.

           slices are references to a series of elements that are
           part of some other value, like an array or vector.

           ? A mutable slice &mut [T] allows read and modification of
           ? elements, but can't be shared.

           ? A shared slice &[T] lets you share access among several
           ? readers, but doesn't let you modify elements.
    */
}
