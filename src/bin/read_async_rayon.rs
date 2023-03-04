use anyhow::{Context, Result};
use noodles::bam;
use noodles::sam;
use rayon::prelude::*;
use sam::record::cigar::Cigar;
use sam::record::data::Data;
use std::fs::File;

use std::thread::sleep;

fn read_bam_async_rayon(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = File::open(path).map(bam::Reader::new)?;
    let header: sam::Header = reader.read_header()?.parse()?;
    reader.read_reference_sequences()?;

    reader
        // .records(&header)
        .lazy_records()
        .par_bridge()
        .map(|r| r.unwrap())
        .for_each(|record| {
            let read_name = record.read_name().unwrap().unwrap();

            let data = Data::try_from(record.data())
                .with_context(|| format!("failed to get data {}", read_name))
                .unwrap();

            let cigar = Cigar::try_from(record.cigar())
                .with_context(|| format!("failed to get cigar {}", read_name))
                .unwrap();

            let sequence = sam::record::Sequence::try_from(record.sequence())
                .with_context(|| format!("failed to get sequence {}", read_name))
                .unwrap();

            sleep(std::time::Duration::from_millis(1000));
            println!("read name: {}, cigar: {}", read_name, cigar,);
        });

    Ok(())
}

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let threds = std::env::args()
        .nth(2)
        .unwrap_or("4".to_string())
        .parse::<usize>()
        .unwrap();

    rayon::ThreadPoolBuilder::new()
        .num_threads(threds)
        .build_global()
        .unwrap();

    read_bam_async_rayon(&path).unwrap();
}
