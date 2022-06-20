struct Person {
    name: String,
    birth: i32,
}

fn main() {
    // --- LECTURE: 1
    /*
       ? A variable owns it value.
       * When control leaves block in which the variable
       * is declared, the variable is dropped, so its value
       * is dropped along with it.
    */

    print_padovan();

    // --- LECTURE: 2
    {
        /*
           ? When the program calls Box::new, it allocates space
           ? for a tuple of two f64 values on the heap, moves its
           ? argument (0.625, 0.5) into that space, and returns a
           ? pointer to it.
        */

        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here

        assert_eq!(label, "(0.625, 0.5)");
    } // both dropped here

    // --- LECTURE: 3
    /* ****************************
     * Just as variables own their values,
     * structs own their fields;
     * and tuples, arrays, and vectors own their elements
     */

    let mut composers = Vec::new();

    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });

    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });

    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

fn print_padovan() {
    //! a vector of 32-bit integers
    let mut padovan = vec![1, 1, 1]; // allocated here

    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);

    /* When the variable padovan goes out of scope at the end of
       the function, the program drops the vector. And since the
       vector owns its buffer, the buffer goes with it.
    */
} // dropped here
