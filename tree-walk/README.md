# Work in Progress

This is an intentionally naive, painfully slow, and still-incomplete
implementation of the tree-walk interpreter for Lox from the first
half of the book [Crafting
Interpreters](https://craftinginterpreters.com).  I'm porting it to
Rust, chapter-by-chapter, with no foresight, which means that it's
unidiomatic at times, as the book uses Java in the first half, along
with some awkward OOP patterns.

I plan to be more particular for the second half, focusing on the
bytecode-based implementation.

## Usage

You should be able to run it on any of the scripts in the `examples`
directory and see some debugging output showing what's happening.

```
cargo run -- examples/basic.lox
```

You can also run in interpreter mode without a file with just `cargo
run`.
