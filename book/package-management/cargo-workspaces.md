# Cargo workspaces

## How to create a workspace for managing multiple cargo projects?

A [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) can be defined to group multiple cargo projects. Following is the main workspace definition for the code for this project's examples.

```toml
[workspace]

members = [
  "benchmarking",
  "testing",
  "text-processing",
]
```
