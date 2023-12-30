# rustquines

Quines, written in Rust

- [darkmage](https://www.evildojo.com)
- [Twitter](https://www.twitter.com/evildojo666)
- [Youtube](https://www.youtube.com/@evildojo666)
- [Twitch](https://www.twitch.tv/@evildojo666)

# building

write your primary code in `template.rs`
we have to use cargo build in order to start pulling in other libraries
so we wrap the process in `generate_quine.sh`
the current version randomly sizes the towers generated

```
# run generate_quine.sh
./generate_quine.sh
```

To invoke the loop to watch the code get compiled then printed out (for visualization), run `./do_loop.sh`.


# preview

[![asciicast](https://asciinema.org/a/fR8nQrRnSTTrx0bWeMldArJ1n.svg)](https://asciinema.org/a/fR8nQrRnSTTrx0bWeMldArJ1n)

