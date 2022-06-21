pub fn run() {
    /* In Rust, references are created explicitly with the
       & operator, and dereferenced explicitly with the *
       operator:
    */

    let x = 10;
    let r = &x;
    assert!(*r == 10);

    /* To create a mutable reference, use the &mut operator: */

    let mut y = 32;
    let m = &mut y;
    *m += 32;

    assert!(*m == 64);

    /* . operator implicitly dereferences its left operand, if needed: */

    struct Anime {
        name: &'static str,
        bechedel_pass: bool,
    }

    let aria = Anime {
        name: "Aria: The Animation",
        bechedel_pass: true,
    };

    let anime_ref = &aria;

    assert_eq!(anime_ref.name, "Aria: The Animation");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");
}
