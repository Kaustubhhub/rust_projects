use reqwest::blocking::Response;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\n*** GET request using RUST ***");

    // Perform a blocking GET request
    let mut response: Response = reqwest::blocking::get("https://httpbin.org/get")?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;

    // Print the response details
    println!("Status : {}", response.status());
    println!("\nHeaders : {:#?}", response.headers());
    println!("\nBody : {}", body);

    Ok(())
}
