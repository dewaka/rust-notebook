# Twoway

## Fast substring search by two way algorithm

[twoway](https://github.com/bluss/twoway) implements [Two way algorithm](http://www-igm.univ-mlv.fr/~lecroq/string/node26.html) for substring search which is highly optimised using SMID acceleration (depending on availability).

Example usage,

```rust
let loc = twoway::find_str("hello there how are you?", "there");
assert_eq!(Some(6), loc);
```
