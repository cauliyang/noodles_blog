use anyhow::{Context, Result};
use noodles::bam;
use noodles::sam;
use rayon::prelude::*;
use std::path::Path;

use noodles::bgzf;

use std::io::{self, Read, Seek};

pub trait NoodleBamIndexReaderExt {
    fn seek_to_first_record(&mut self) -> io::Result<bgzf::VirtualPosition>;
}

impl<R> NoodleBamIndexReaderExt for bam::indexed_reader::IndexedReader<bgzf::Reader<R>>
where
    R: Read + Seek,
{
    // add code here
    fn seek_to_first_record(&mut self) -> io::Result<bgzf::VirtualPosition> {
        // seek to first record
        let areader = self.get_mut();
        // areader.seek(bgzf::VirtualPosition::default()).unwrap();
        areader.seek(bgzf::VirtualPosition::default())?;

        self.read_header()?;
        self.read_reference_sequences()?;
        Ok(self.get_ref().virtual_position())
    }
}

fn count<T>(path: T) -> Result<()>
where
    T: AsRef<Path>,
{
    let mut reader = bam::indexed_reader::Builder::default()
        .build_from_path(path.as_ref())
        .with_context(|| {
            format!(
                "failed to read bam file and index not existed {:?} ",
                path.as_ref()
            )
        })?;

    let header: sam::Header = reader
        .read_header()
        .context("failed to read bam reader")?
        .parse()
        .context("failed to parse bam rader")?;

    let reference = reader
        .read_reference_sequences()
        .context("failed to read reference sequences")?;

    let count1 = reader.lazy_records().count();
    println!("first count: {}", count1);

    reader.seek_to_first_record().unwrap();
    let count2 = reader.lazy_records().count();
    println!("second count: {}", count2);

    Ok(())
}

fn query<T>(path: T) -> Result<()>
where
    T: AsRef<Path>,
{
    let mut reader = bam::indexed_reader::Builder::default()
        .build_from_path(path.as_ref())
        .with_context(|| {
            format!(
                "failed to read bam file and index not existed {:?} ",
                path.as_ref()
            )
        })?;

    let header: sam::Header = reader
        .read_header()
        .context("failed to read bam reader")?
        .parse()
        .context("failed to parse bam rader")?;

    let reference = reader
        .read_reference_sequences()
        .context("failed to read reference sequences")?;

    let region = "chr17:79778148-79778149"
        .parse()
        .expect("failed to parse region");

    let count = reader.query(&header, &region).unwrap().par_bridge().count();
    println!("{} records found", count);

    Ok(())
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    count(&path).unwrap();
}
