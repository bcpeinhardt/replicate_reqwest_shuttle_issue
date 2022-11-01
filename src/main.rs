use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Works
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    // Fails
    let resp = reqwest::get("https://hello_world.shuttleapp.rs/hello")
        .await?
        .json::<String>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}
