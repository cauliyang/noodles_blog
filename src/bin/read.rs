use noodles::bam;
use noodles::sam;
use std::fs::File;
use std::thread::sleep;

fn read_bam(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = File::open(path).map(bam::Reader::new)?;
    let header: sam::Header = reader.read_header()?.parse()?;
    reader.read_reference_sequences()?;

    reader
        .records(&header)
        .map(|r| r.unwrap())
        .for_each(|record| {
            println!("read name: {}", record.read_name().unwrap());
            sleep(std::time::Duration::from_millis(1000));
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

    read_bam(&path).unwrap();
}
