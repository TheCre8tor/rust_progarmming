pub fn run() {
    // Assigning a string moves the value
    let str1 = "somnambulance".to_string();
    let _str2 = str1;

    // whereas assigning an i32 copies it
    let num1: i32 = 36;
    let _num2 = num1;
    let _num3 = num1;

    /* An i32 is simply a pattern of bits in memory;
       it doesn’t own any heap resources, or really
       depend on anything other than the bytes it
       comprises. By the time we’ve moved its bits to
       num2, we’ve made a completely independent copy
       of num1.
    */

    /* As a rule of thumb, any type that needs to do
       something special when a value is dropped cannot
       be Copy.

       ? A Vec needs to free its elements;
       ? a File needs to close its file handle;
       ? a MutexGuard needs to unlock its mutex.

       Bit-for-bit duplication of such types would leave
       it unclear which value was now responsible for
       the original’s resources.
    */

    // By default, struct and enum types are not Copy:
    /* If all the fields of your struct are themselves Copy,
       then you can make the type Copy as well by placing the
       attribute #[derive(Copy, Clone)] above the definition
    */

    // 1. Without Clone and Copy Trait ->

    struct Label {
        number: i32,
    }

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let l = Label { number: 3 };

    print(l);

    // println!("My label number is: {}", l.number); // borrow of moved value: `l`

    // 2. With Clone and Copy Trait ->

    #[derive(Clone, Copy)]
    struct StringLabel {
        number: i32,
    }

    fn printfn(x: StringLabel) {
        println!("STRING: {}", x.number);
    }

    let str = StringLabel { number: 67 };

    printfn(str);

    println!("My label number is: {}", str.number);
}
