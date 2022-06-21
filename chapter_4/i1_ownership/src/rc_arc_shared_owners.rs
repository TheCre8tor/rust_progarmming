use std::rc::Rc;

pub fn run() {
    /* -> Rc and Arc: Shared Ownership
       If you want the lifetime of all your values to
       simply live until everyone’s done using it. For
       these cases, Rust provides the reference-counted
       pointer types Rc and Arc.

       ? The Rc and Arc types are very similar;
       the only difference between them is that:
       * 1. An Arc is safe to share between threads directly.
       * -> The name Arc is short for [atomic reference count]

       * 2. Rc uses faster non-thread-safe code to update its
       * reference count.
       * -> If you don’t need to share the pointers between
       * threads, there’s no reason to pay the performance
       * penalty of an Arc , so you should use Rc.
    */

    /* Python uses reference counts to manage its values’ lifetimes.
       You can use Rc to get a similar effect in Rust. Consider the
       following code:
    */

    // Rust can infer all these types; written out for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    /* For any type T, an Rc<T> value is a pointer to a heap-allocated T
       that has had a reference count affixed to it. Cloning an Rc<T> value
       does not copy the T; instead, it simply creates another pointer to
       it, and increments the reference count.

       Each of the three Rc<String> pointers is referring to the same block
       of memory, which holds a reference count and space for the String.
       The usual ownership rules apply to the Rc pointers themselves, and
       when the last extant Rc is dropped, Rust drops the String as well.
    */

    // You can use any of String’s usual methods directly on an Rc<String>:

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavour", u);
}
