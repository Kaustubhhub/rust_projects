extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    println!("****Project compression using rust****");
    if args().len() != 3 {
        eprint!("usage : `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    match copy(&mut input, &mut encoder) {
        Ok(_) => println!("Compression completed in {:?}", start.elapsed()),
        Err(err) => eprintln!("Compression failed: {}", err),
    }

    let output = encoder.finish().unwrap();
    println!(
        "target len : {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("target len : {:?}", output.metadata().unwrap().len());
    println!("elapsed time : {:?}", start.elapsed());
}
