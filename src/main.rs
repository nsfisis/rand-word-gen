use anyhow::{Context, Result};
use rand::distributions::{Distribution, Uniform};
use rand_word_gen::Model;
use std::io::{BufWriter, Write};

fn main() -> Result<()> {
    let words = parse_args()?;

    let model = Model::new();
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(3..=6);
    for _ in 0..words {
        let len = dist.sample(&mut rng);
        writeln!(out, "{}", model.generate(&mut rng, len))?;
    }

    Ok(())
}

fn parse_args() -> Result<usize> {
    use clap::{crate_description, crate_version, value_t, App, Arg};

    let matches = App::new("rand-word-gen")
        .version(crate_version!())
        .version_short("v")
        .about(crate_description!())
        .arg(
            Arg::with_name("words")
                .short("n")
                .long("words")
                .value_name("NUMBER_OF_WORDS")
                .default_value("20")
                .help("Sets number of generated words"),
        )
        .get_matches();

    let words = value_t!(matches, "words", usize).context("'--words' must be a number")?;
    Ok(words)
}
