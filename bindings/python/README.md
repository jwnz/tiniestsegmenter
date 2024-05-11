# TiniestSegmenter

A port of [TinySegmenter](http://chasen.org/~taku/software/TinySegmenter/) written in pure, safe rust with no dependencies. You can find bindings for both [Rust](https://github.com/jwnz/tiniestsegmenter/tree/master/tiniestsegmenter) and [Python](https://github.com/jwnz/tiniestsegmenter/tree/master/bindings/python/).

TinySegmenter is an n-gram word tokenizer for Japanese text originally built by [Taku Kudo](http://chasen.org/~taku/) (2008). 


<b> Usage </b>

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
