pub mod plant_structures;

/* The use declaration causes the name mem to
   be a local alias for ::std::mem throughout
   the enclosing block or module.

   Paths in use declarations are automatically
   absolute paths, so there is no need for a
   leading ::
*/
use std::mem;

fn main() {
    println!("Hello, world!");

    plant_structures::leaves::get();

    if 1 > 2 {
        /* This function name, ::std::collections::HashMap, is an
           absolute path, because it starts with a double colon.

           ::std -> refers to the top-level module of the standard libary.
           ::std::collections -> ::collections is a submodule within the standard libary.
           ::std::collections::HashMap -> ::HashMap is a public struct in that module.
        */
        let _ = ::std::collections::HashMap::<String, String>::new();
    }

    if 2 < 1 {
        mem::swap(&mut 1, &mut 2);
    }
}
