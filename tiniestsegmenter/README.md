# TiniestSegmenter

A port of [TinySegmenter](http://chasen.org/~taku/software/TinySegmenter/) written in pure, safe rust with no dependencies. You can find bindings for both [Rust](https://github.com/jwnz/tiniestsegmenter/tree/master/tiniestsegmenter) and [Python](https://github.com/jwnz/tiniestsegmenter/tree/master/bindings/python/).

TinySegmenter is an n-gram word tokenizer for Japanese text originally built by [Taku Kudo](http://chasen.org/~taku/) (2008). 


<b> Usage </b>

Add the crate to your project: `cargo add tiniestsegmenter`.

```Rust
use tiniestsegmenter as ts;

fn main() {
    let tokens: Result<Vec<&str>, ts::TokenizeError> = ts::tokenize("ジャガイモが好きです。");
}
```

