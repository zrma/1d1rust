//noinspection SpellCheckingInspection
extern crate reqwest;

use std::collections::HashMap;

#[allow(dead_code)]
async fn call_body() -> Result<(), reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client
        .post("https://httpbin.org/post")
        .json(&map)
        .send()
        .await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}

#[test]
fn test_call_body() {
    let _ = call_body();
}
