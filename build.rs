use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::path::Path;

const NUM_LETTERS: usize = 26;

fn main() -> Result<()> {
    let chars = {
        let file = File::open("/usr/share/dict/words")?;
        let reader = BufReader::new(file);
        parse(reader)?
    };

    let out_path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("prebuilt_model.rs");
    let file = File::create(out_path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "[")?;
    for i in 0..(NUM_LETTERS + 1) {
        write!(writer, "[")?;
        for j in 0..(NUM_LETTERS + 1) {
            write!(writer, "{},", chars[i][j])?;
        }
        writeln!(writer, "],")?;
    }
    writeln!(writer, "]")?;

    Ok(())
}

fn parse(r: BufReader<File>) -> Result<[[usize; NUM_LETTERS + 1]; NUM_LETTERS + 1]> {
    let mut chars = [[0; NUM_LETTERS + 1]; NUM_LETTERS + 1];
    for line in r.lines() {
        let word = line?;
        let mut prefix = NUM_LETTERS;
        for c in word.bytes() {
            let c = c.to_ascii_lowercase();
            if !c.is_ascii_lowercase() {
                continue;
            }
            let i = (c - b'a') as usize;
            chars[prefix][i] += 1;
            prefix = i;
        }
    }
    Ok(chars)
}
