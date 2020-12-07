# Random word generator

Generates random pseudo-English words. It is mainly for naming your hobby projects in a moment.

It internally uses "Markov chain" to generate words. The chain model is built in a compiling phase of the tool, referring to `words` file (`/usr/share/dict/words`).

## Usage

```
$ rand-word-gen
eendo
ddips
uphr
blinta
unto
ont
vedaro
dera
ekarrb
goice
sfodo
ainis
rier
fatem
myimd
grasic
honge
ustoge
ear
nal

# Sets a number of generated words.
$ rand-word-gen -n 5
piva
ors
glo
mapt
blclyp
```

See `rand-word-gen --help` for details.

## Build

```
$ cargo build --release
```

## TODO

* Pass a custom words file at runtime.
* Specify length of generated words.

## License

MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
