//! # fastq file io utils
//! * [`fqreader`] open uncompressed/gzipped file from `std::io::stdin` or file, then return a fastq file reader
//! * [`record`] fastq read struct
//! 
//!




use crate::utils;
use std::io::{BufRead, Read};


/// fastq file reader
#[allow(non_camel_case_types)]
pub struct fqreader {
    pub reader: Box<dyn BufRead>,
}


impl fqreader {
    /// return a fastq file reader
    /// ```rust
    /// use seqio::fastq::fqreader;
    /// 
    /// let file = &Some("test/demo.fq");
    /// let records = fqreader::new(file).unwrap();
    /// 
    /// for rec in records.flatten() {
    ///     println!("{}\t{}",rec.name(), rec.len())
    /// }
    /// ```
    /// read content from fastq file/file.gz or stdin
    pub fn new(file: &Option<&str>) -> std::io::Result<Self> {
        Ok(Self {
            reader: utils::file_reader(file)?,
        })
    }
}
/// fastq reads struct
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct record {
    id: String,
    seq: String,
    symbol: String,
    qual: String,
}

impl record {
    /// init a empty fastq record
    pub fn new() -> Self {
        Self {
            id: String::new(),
            seq: String::new(),
            symbol: String::new(),
            qual: String::new(),
        }
    }
    /// report read length
    pub fn len(&self) -> usize {
        self.seq.len()
    }
    /// report read id description information
    pub fn desc(&self) -> Option<&str> {
        if let Some(ret) = self.id.split_once(" ") {
            Some(ret.1)
        } else {
            None
        }
    }
    /// just report read name
    pub fn name(&self) -> &str {
        if let Some(ret) = self.id.split_once(" ") {
            ret.0
        } else {
            self.id.as_str()
        }
    }
}

impl Iterator for fqreader {
    type Item = Result<record, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut idx = 0usize;
        let mut record = record::new();
        for line in self.reader.by_ref().lines().flatten() {
            idx += 1;
            match idx {
                1 => record.id = line,
                2 => record.seq = line,
                3 => record.symbol = line,
                4 => record.qual = line,
                _ => unreachable!(),
            }
            if idx == 4 {
                return Some(Ok(record));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::fqreader;

    #[test]
    fn test1() -> std::io::Result<()> {
        let file = &Some("test/demo.fq");
        let recs = fqreader::new(file)?;
        for multi in recs.flatten() {
            println!("{:?}", multi);
            println!("{}\t{}\t{:?}", multi.name(), multi.len(), multi.desc());
        }

        Ok(())
    }

}
