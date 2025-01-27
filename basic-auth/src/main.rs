use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let username = String::from("kaustubh");
    let password: Option<String> = None;

    let response = client
        .get("https://httpbin.org/get")
        .basic_auth(username, password)
        .send()
        .await?;

    println!("\n\nThe response is : {:?}", response.status());

    Ok(())
}
