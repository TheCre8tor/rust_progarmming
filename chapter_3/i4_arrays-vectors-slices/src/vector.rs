pub fn run() {
    // A vector Vec<T> is a resizable array of elements of type T,
    // allocated on the heap.

    let mut v = vec![2, 3, 5, 7];

    //? fold works like 'reduce' method in JS
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);

    v.push(11);
    v.push(13);

    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);

    //* Long duplicate filled with values ->
    /* This is an example of an duplicate vec![V; L],
       where V is the value each element should
       have, and N is the length.
    */

    let buffer = new_pixel_buffer(10, 2);
    println!("{:?}", buffer);

    // ? The vec! macro is equivalent to calling Vec::new()
    // ? to create a new, empty vector

    let mut vet = Vec::new();

    vet.push("step");
    vet.push("on");
    vet.push("no");
    vet.push("pets");

    assert_eq!(vet, vec!["step", "on", "no", "pets"]);

    // Another possibility is to build a vector from the values
    // produced by an iterator

    let vac: Vec<i8> = (0..5).collect();
    assert_eq!(vac, [0, 1, 2, 3, 4]);

    /*  A Vec<T> consist of three values:
        1. A pointer to the heap-allocated buffer allocated to
           hold elements;
        2. The number of elements that buffer has the capacity
           to store;
        3. And the number it actually contains now -> 'it length'

    */

    let mut v = vec![10, 20, 30, 40, 50];

    // Make the element at index 3 be 35.
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    // Remove the element at index 1.
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}
