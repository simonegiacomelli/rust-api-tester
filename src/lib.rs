pub fn add(left: usize, right: usize) -> usize {
    left + right
}


fn get(url: &str) -> String {
    reqwest::blocking::get(url).unwrap().text().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = get("http://httpbin.org/json");
        assert!(actual.contains("Sample Slide Show"))
    }
}
