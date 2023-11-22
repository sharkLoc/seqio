# seqio

A file reading and writing, support gzip format compression in rust

### install
To use seqio in your Rust project, run command blow
```shell
cargo add seqio
```
Or add the following to your `Cargo.toml`
```text
[dependencies]
seqio = "0.1.0"
```
Now seqio modules can be used directly in your source code, for example:
```rust
use seqio::utils::*;
```