#[path = "./passing_ref_as_args.rs"]
mod passing_ref_as_args;

fn main() {
    /* Rust’s equivalent of a global variable is called
       a static: it’s a value that’s created when the
       program starts and lasts until it terminates.

       RULES:
       1. Every static must be initialized.
       2. Mutable statics are inherently not thread-safe.
          -> For these reasons, you may access a mutable
          static only within an unsafe block.
    */
    static mut STASH: &i32 = &128;

    static WORTH_POINTING_AT: i32 = 3;

    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }

    f(&WORTH_POINTING_AT);

    // LECTURE: 2
    passing_ref_as_args::run();
}
