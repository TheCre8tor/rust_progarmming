pub mod plant_structures;
use plant_structures::leaves::Likes;

/* The use declaration causes the name mem to
   be a local alias for ::std::mem throughout
   the enclosing block or module.

   Paths in use declarations are automatically
   absolute paths, so there is no need for a
   leading ::
*/
use std::{collections::HashMap, mem, primitive};

fn main() {
    println!("Hello, world!");

    plant_structures::leaves::get();

    let _ = Likes {
        love: false,
        hate: false,
    };

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

    // Type Aliases

    type _Table = HashMap<String, Vec<String>>;
}
