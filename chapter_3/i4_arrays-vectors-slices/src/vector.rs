pub fn run() {
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

    /*
        ? The vec! macro is equivalent to calling Vec::new()
        ? to create a new, empty vector
    */

    let mut vet = Vec::new();

    vet.push("step");
    vet.push("on");
    vet.push("no");
    vet.push("pets");

    assert_eq!(vet, vec!["step", "on", "no", "pets"]);
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}
