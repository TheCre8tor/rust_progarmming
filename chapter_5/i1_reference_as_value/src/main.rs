use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

#[path = "./references_as_values.rs"]
mod references_as_values;

fn main() {
    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );

    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );

    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    /* References are non-owning pointers, so the table variable
       remains the owner of the entire structure; `show` has just
       borrowed it for a bit.
    */

    show(&table);
    sort_works(&mut table);

    // assert_eq!(table["Gesualdo"][0], "many madrigals");

    // LECTURE: 2 ->
    references_as_values::run();
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}: ", artist);

        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}
