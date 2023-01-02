pub fn add(left: usize, right: usize) -> usize {
    left + right
}

async fn get(url: &str) -> String {
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let actual = get("http://httpbin.org/json").await;
        assert!(actual.contains("Sample Slide Show"))
    }
}
