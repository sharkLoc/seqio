//! Simple wrapper of file io
//! * **Utils**: File reading and writing, support gzip format compression, see [`utils`] module for an introduction
//! * **Fastq**: Fastq file reader, see[`fastq`] module for an introduction
//!
//! # install
//! To use seqio in your Rust project, run command blow
//! ```shell
//! cargo add seqio
//! ```
//! Or add the following to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! seqio = "0.1.0"
//! ```
//! Now seqio modules can be used directly in your source code, for example:
//! ```rust
//! use seqio::utils::*;
//! use seqio::fastq::fqreader;
//! ```


pub mod fastq;
pub mod utils;
