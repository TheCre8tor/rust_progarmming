use std::{
    fs::File,
    io::{Result, Write},
};

pub fn run() -> Result<()> {
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"Hello World")?;

    let mut buff: Vec<u8> = vec![];
    // let writer: dyn Write = buf; // This fails because the size of the object is not know at compile time
    let writer: &mut dyn Write = &mut buff;

    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file);

    /* Likewise, Rust will happily convert a Box<File> to
       a Box<Write> , a value that owns a writer in the heap
    */

    let _w: Box<dyn Write> = Box::new(local_file);

    Ok(())
}

fn say_hello(_data: &mut dyn Write) {}
