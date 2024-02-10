// noinspection SpellCheckingInspection
extern crate reqwest;

use std::collections::HashMap;

#[allow(dead_code)]
async fn call_body(url: &str) -> Result<(), reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&map)
        .send()
        .await
        .map_err(|e| {
            eprintln!("HTTP request failed: {}", e);
            e
        })?
        .error_for_status()
        .map_err(|e| {
            eprintln!("HTTP response error status: {}", e);
            e
        })?;

    println!("Status: {}", res.status());

    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    #[ignore]
    async fn test_call_body() {
        let url = "https://httpbin.org/post".to_string();
        let res = call_body(&url).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_call_body_with_mock_success() {
        let mock_server = MockServer::start().await;

        let response =
            ResponseTemplate::new(200).set_body_string(r#"{"message": "This is a mock response"}"#);

        Mock::given(method("POST"))
            .and(path("/post"))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let url = format!("{}/post", mock_server.uri());
        let result = call_body(&url).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_call_body_with_mock_error() {
        let mock_server = MockServer::start().await;

        let response =
            ResponseTemplate::new(500).set_body_string(r#"{"error": "Internal Server Error"}"#);

        Mock::given(method("POST"))
            .and(path("/post"))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let url = format!("{}/post", mock_server.uri());
        let result = call_body(&url).await;

        assert!(result.is_err());
    }
}
