# detect-indent-rs

rust port of
(sindresorhus/detect-indent)[https://github.com/sindresorhus/detect-indent] All
attribution goes to that project.

> Detect the indentation of code

Pass in a string of any kind of text and get the indentation.

## Use cases

- Persisting the indentation when modifying a file.
- Have new content match the existing indentation.
- Setting the right indentation in your editor.


## Usage

Add this to your `Cargo.toml`

```
[dependencies]
detect-indent = "0.1"
```

and this to your crate root

```rs
extern crate detect_indent;

fn main() {
  println!(detect_indent(""));
}
```

## Algorithm

The current algorithm looks for the most common difference between two
consecutive non-empty lines. [More Details](https://github.com/sindresorhus/detect-indent)
