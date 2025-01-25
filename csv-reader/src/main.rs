use ::std::io;
use std::env::args;
fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    println!("****CSV reader using rust****");

    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage {} <filename> ", args[0]);
        return 1;
    }
    let file = std::fs::File::open(&*args[1]).unwrap();
    println!("########{:?}", &*args[1]);
    println!("********{:?}", args[1]);
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result.unwrap();
        println!("{:?}", record);
    }
    0
}
