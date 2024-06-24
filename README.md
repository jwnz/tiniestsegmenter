# TiniestSegmenter

A port of [TinySegmenter](http://chasen.org/~taku/software/TinySegmenter/) written in pure, safe rust with no dependencies. You can find bindings for both [Rust](https://github.com/jwnz/tiniestsegmenter/tree/master/tiniestsegmenter) and [Python](https://github.com/jwnz/tiniestsegmenter/tree/master/bindings/python/).

TinySegmenter is an n-gram word tokenizer for Japanese text originally built by [Taku Kudo](http://chasen.org/~taku/) (2008). 


## Usage

<b> Python </b>

`tiniestsegmenter` can be installed from PyPI: `pip install tiniestsegmenter`

```Python
import tiniestsegmenter

tokens = tiniestsegmenter.tokenize("ジャガイモが好きです。")
```

With the GIL released on the rust side, multi-threading is also possible.

```Python
import functools
import tiniestsegmenter

tokenizer = functools.partial(tiniestsegmenter.tokenize)

documents = ["ジャガイモが好きです。"] * 10_000
with ThreadPoolExecutor(4) as e:
    list(e.map(encoder, documents))
```

<b> Rust </b>

Add the crate to your project: `cargo add tiniestsegmenter`

Usage:
```Rust
use tiniestsegmenter as ts;

fn main() {
    let tokens: Result<Vec<&str>, ts::TokenizeError> = ts::tokenize("ジャガイモが好きです。");
}
```

## Performance
`tiniestsegmenter` can process 2GB of text in less than 90 seconds on a Macbook Pro at speeds of around `±20 MB/s` on a single thread.

<b>Comparison with similar codebases </b>

Each codebase was benchmarked using the [timemachineu8j](https://www.genpaku.org/timemachine/timemachineu8j.txt) dataset, a Japanese transation of The Time Machine by Herbert George Wells.


| Repo    | Lang | time (ms) |
| -------- | ------- | ------- |
| jwnz/tiniestsegmenter  | Rust    | 11.996 |
| jwnz/tiniestsegmenter  | Python    | 14.803 |
| nyarla/go-japanese-segmenter  | Go    | 36.869 |
| woxtu/rust-tinysegmenter  | Rust    | 44.535 |
| JuliaStrings/TinySegmenter.jl  | Julia    | 45.691 |
| ikawaha/tinysegmenter.go  |  Go  | 58.694 |
| SamuraiT/tinysegmenter  | Python    | 219.604 |

System:<br>
Chip: Apple M2 Pro (Macbook Pro 14-inch, 2023)<br>
Cores: 10<br>
Memory: 16 GB
