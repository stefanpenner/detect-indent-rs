# detect-indent-rs [![Test](https://github.com/stefanpenner/detect-indent-rs/actions/workflows/CI.yml/badge.svg)](https://github.com/stefanpenner/detect-indent-rs/actions/workflows/CI.yml)
rust port of
[sindresorhus/detect-indent](https://github.com/sindresorhus/detect-indent) All
attribution goes to that project.

> Detect the indentation of code

Pass in a string of any kind of text and get the indentation.

## Use cases

- Persisting the indentation when modifying a file.
- Have new content match the existing indentation.
- Setting the right indentation in your editor.


## Usage

Add this to your `Cargo.toml`

```toml
[dependencies]
detect-indent = "0.1"
```

and this to your crate root

```rust
extern crate detect_indent;
use detect_indent::detect_indent;

fn main() {
  println!("{:?}", detect_indent(""));
}
```

## Algorithm

The current algorithm looks for the most common difference between two
consecutive non-empty lines. [More Details](https://github.com/sindresorhus/detect-indent)

