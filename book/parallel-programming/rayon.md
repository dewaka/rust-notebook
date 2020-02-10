# Rayon

## Parllalel collection transformations using rayon

Mutating a vectors to double its elements can be done by,

```rust
let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
// Transform sequentially
nums.iter_mut().for_each(|n| *n *= 2);
dbg!(nums);
```

With [rayon](https://docs.rs/rayon/1.3.0/rayon/) it is easy to perform the same operation in parallel by using `par` iterators instead,


```rust
use rayon::prelude::*;

let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
// Transform in parallel
nums.iter_mut().for_each(|n| *n *= 2);
dbg!(nums);
```
