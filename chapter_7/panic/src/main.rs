#[path = "./unwinding.rs"]
mod unwinding;

fn main() {
    /* There are two different kinds of error-handling in Rust:
       ? panic and Results.

       * Ordinary errors are handled using Results.
       -> These are typically caused by things outside the program,
          like erroneous input, a network outage, or a permissions
          problem.

       * Panic is for the other kind of error,
       -> the kind that should never happen.

       A program panics when it encounters something so messed up that
       there must be a bug in the program itself. Something like:
       ? • Out-of-bounds array access
       ? • Integer division by zero
       ? • Calling .unwrap() on an Option that happens to be None
       ? • Assertion failure

       Rust can either unwind the stack when a panic happens, or abort
       the process. Unwinding is the default.
    */

    unwinding::run();
}
