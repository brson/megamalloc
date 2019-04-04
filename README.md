The megamalloc crate packages up all the Rust allocator crates,
exposing their metrics through a consistent interface,
providing tools to compare performance and tweak configurations.

It supports the following features:

- global - Define the global allocator (default)
- system_alloc - Use the system allocator (default)
- jemalloc - Use jemalloc
- ralloc - Use ralloc
- tcmalloc - Use tcmalloc

The megamalloc global allocator wraps the global allocator of
whichever implementation is selected, and provides additional methods.

```rust
/// Returns a string uniquely naming the allocator
pub fn name(&self) -> &'static str;

/// Returns allocator-dependent metrics as a `Vec` of `(str, usize)`
pub fn fetch_stats(&self) -> Result<Option<AllocStats>>;
```

Metrics are implemented for glibc malloc (the system allocator on most
linux targets), jemalloc, and tcmalloc.

That's all for now. More in the future.
