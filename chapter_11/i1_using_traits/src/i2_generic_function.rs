use std::{
    collections::btree_map::Values,
    fmt::Debug,
    fs::File,
    hash::Hash,
    io::{Result, Write},
};

pub fn run() -> Result<()> {
    let mut data = File::create("empty.txt")?;
    let mut bytes: Vec<u8> = vec![];

    /* Rust will infer the type from the arguments ->
       say_hello::<File>(&mut data);
       say_hello::<Vec<u8>(&mut data);
    */
    say_hello(&mut data)?;
    say_hello(&mut bytes)?;

    /* If the generic function we are calling doesn't have any arguments
       that provide useful clues, you may have to spell it out.
       fn collect<B: FromIterator<Self::Item>>(self) -> B;
    */
    let _v1 = (0..100).collect::<Vec<u8>>();

    let numbers: Vec<u32> = vec![2, 6, 8, 9, 6, 0];
    top_ten(&numbers);

    Ok(())
}

fn say_hello<W: Write>(out: &mut W) -> Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {
    println!("Supported Values: {:?}", values);
}

// Generic function can have multiple type parameters ->

fn _run_query<M, R>(_data: &Vec<u8>, _map: M, _reduce: R) -> Result<()>
where
    M: Hash + Eq,
    R: Debug + Hash + Eq,
{
    Ok(())
}
