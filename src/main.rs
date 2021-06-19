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
        .arg(
            Arg::with_name("prefix")
                .short("p")
                .long("prefix")
                .value_name("PREFIX")
                .default_value("")
                .help("Sets prefix of generated words"),
        )
        .get_matches();

    let words = value_t!(matches, "words", usize).context("'--words' must be a number")?;
    let mut prefix =
        value_t!(matches, "prefix", String).context("Fatal error: failed to parse '--prefix'")?;
    if !prefix.bytes().all(|c| c.is_ascii_alphabetic()) {
        bail!("'--prefix' must be alphabetic");
    }
    prefix.make_ascii_lowercase();
    Ok(Config { words, prefix })
}
