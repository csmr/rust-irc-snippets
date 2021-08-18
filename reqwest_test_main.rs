#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let body = reqwest::get("https://httpbin.org/ip")
        .await?
        .text()
        .await?;

    println!("{}", body);
    Ok(())
}
