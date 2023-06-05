use std::{
    fs,
    io::{BufReader, BufWriter},
};

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let fd = BufReader::new(fs::File::open(&args[1])?);

    let mut cb = BufWriter::new(fs::File::create("out.cbor")?);

    let parser = tacview::Parser::new(fd)?;
    for record in parser {
        let record = record?;
        ciborium::into_writer(&record, &mut cb)?;
    }
    Ok(())
}
