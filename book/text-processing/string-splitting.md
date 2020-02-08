# String Splitting

## How to split a string based on specific separators?

- By separator - `s.split("separator")`
- By whitespace - `s.split_whitespace()`
- By newlines - `s.lines()`

## How to split a string based on multiple characters?

Standard library `split` can be used with a closure to fulfill this use case.

```rust
fn split_stuff() {
    let message = "Hello there | how are you ; doing?";
    for s in message.split(|c| (c == '|') || (c == ';')) {
        dbg!(s);
    }
}
```

Output would be as follows,

> s = "Hello there "
> s = " how are you "
> s = " doing?"

To get rid of spaces, [`trim()`](https://doc.rust-lang.org/std/string/struct.String.html#method.trim) method can be used.

Although it is possible to write a closure to split on arbitrarily complex conditions, it easier to use `regex` crate for splitting based on regular expressions.

Another approach without using a closure would be to make use of string slice [patterns](https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html). As an example, we can write the above split as,

```rust
for s in message.split(&['|', ';'][..]) {
// ...
}
```

Splits can be collected to a vector and other types of Rust collections with `collect` method.

## How to split a string based on regular expressions?

Splitting a string such as `"hello there; how|    are|you, chathura"` based on multiple characters and taking into account multiple spaces is not simple without using regular expressions.

Following solution demonstrates the use of regex crate to accomplish the task.

```rust
fn split_by_regex() {
    let message = "hello there; how|    are|you, Chathura";

    let re = regex::Regex::new(r"[;,|\s]\s*").unwrap();

    for s in re.split(message) {
        dbg!(s);
    }
}
```
