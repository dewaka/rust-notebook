# Conversions between Strings and Vec<char>

## How to convert a String to a vector of chars?

```rust
let s = "Hello there";

// Convert a string to a Vec<char>
let cvec: Vec<char> = s.chars().collect();
```

## How to convert vector of chars back to a String?

Carrying from the above example,
```rust
// Convert a string to a Vec<char>
let cvec: Vec<char> = s.chars().collect();

// Convert a Vec<char> into a String
let back: String = cvec.into_iter().collect();
```
