use std::io::{Result, Write};

pub fn run() -> Result<()> {
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"Hello World")?;

    Ok(())
}
