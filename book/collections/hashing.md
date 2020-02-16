# Hashing

## Writing a custom hasher in Rust

Implementing [FNV Hash](http://www.isthe.com/chongo/tech/comp/fnv/index.html),

```rust
struct FnvHasher(u64);

impl Default for FnvHasher {
    fn default() -> Self {
        FnvHasher(0x811c9dc5)
    }
}

impl Hasher for FnvHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;

        for c in bytes.iter() {
            hash = hash.wrapping_mul(0x01000193);
            hash ^= *c as u64;
        }
    }
}
```

