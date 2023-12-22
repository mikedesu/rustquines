# rustquines

Quines, written in Rust

- [darkmage](https://www.evildojo.com)
- [Twitter](https://www.twitter.com/evildojo666)
- [Youtube](https://www.youtube.com/@evildojo666)
- [Twitch](https://www.twitch.tv/@evildojo666)

# building

This will be improved soon, but for now:

```
# write your primary code in template2.rs
# copy template2.rs to template.rs
# delete all of the newlines and tabs
# run makequine.py and write out to test.rs
# compile test.rs
# run test and write out to test2.rs
# compile test2.rs and run

cp template2.rs template.rs
python3 makequine.py > test.rs
rustc test.rs
./test > test2.rs
rustc test2.rs
```

# preview

[![asciicast](https://asciinema.org/a/88XhtsMueWBUxjNEihZHtV8OR.svg)](https://asciinema.org/a/88XhtsMueWBUxjNEihZHtV8OR)


