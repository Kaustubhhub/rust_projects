use reqwest::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("ASYNC GET request using RUST");
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
