pub fn run() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Anthropoda", "Insecta"];

    //* Long duplicate filled with values ->
    /* This is an example of an duplicate [V; L],
       where V is the value each element should
       have, and N is the length.
    */
    let populate_default = [0; 10];

    // Example:
    let mut sieve = [true; 10000];

    thruthy_sieve(&mut sieve);

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);
    assert_eq!(populate_default, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    println!("{}", sieve[211]);
    println!("{}", sieve[9876]);

    //? This is an example of a slice ->
    println!("Slice: {:?}", &populate_default[..3]);

    //? Remove duplicates in Arrays ->
    let mut duplicate = vec![1, 1, 1, 2, 3, 3, 2, 4, 4, 5, 6];
    duplicate.sort();
    duplicate.dedup();
    println!("Duplicate: {:?}", duplicate);
}

pub fn thruthy_sieve(data: &mut [bool]) {
    for i in 2..100 {
        if data[i] {
            let mut j = i * i;

            while j < 10000 {
                data[j] = false;
                j += i;
            }
        }
    }
}
