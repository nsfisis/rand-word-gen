use anyhow::{bail, Context, Result};
use rand::distributions::{Distribution, Uniform};
use rand_word_gen::Model;
use std::io::{BufWriter, Write};

struct Config {
    words: usize,
    prefix: String,
}

fn main() -> Result<()> {
    let config = parse_args()?;

    let model = Model::new();
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(3..=6);
    let first_char = config.prefix.bytes().last().map(|c| c as char);
    for _ in 0..config.words {
        let len = dist.sample(&mut rng);
        writeln!(
            out,
            "{}{}",
            config.prefix,
            model.generate(&mut rng, len, first_char)
        )?;
    }

    Ok(())
}

fn parse_args() -> Result<Config> {
    use clap::{arg, command, value_parser};

    let matches = command!()
        .arg(
            arg!(-n --words <NUMBER_OF_WORDS> "Sets number of generated words")
                .default_value("20")
                .value_parser(value_parser!(usize)),
        )
        .arg(arg!(-p --prefix <PREFIX> "Sets prefix of generated words").default_value(""))
        .get_matches();

    let words = *matches
        .get_one::<usize>("words")
        .context("'--words' must be a number")?;
    let mut prefix = matches
        .get_one::<String>("prefix")
        .context("Fatal error: failed to parse '--prefix'")?
        .clone();
    if !prefix.bytes().all(|c| c.is_ascii_alphabetic()) {
        bail!("'--prefix' must be alphabetic");
    }
    prefix.make_ascii_lowercase();
    Ok(Config { words, prefix })
}
