use std::collections::HashMap;

struct Codec {
    id: usize,
    map: HashMap<usize, String>,
}

impl Codec {
    fn new() -> Self {
        Self {
            id: 0,
            map: HashMap::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    #[allow(non_snake_case)]
    fn encode(&mut self, longURL: String) -> String {
        self.map.insert(self.id, longURL);
        let result = format!("http://tinyurl.com/{}", self.id);
        self.id += 1;

        result
    }

    // Decodes a shortened URL to its original URL.
    #[allow(non_snake_case)]
    fn decode(&self, shortURL: String) -> String {
        let id: usize = shortURL.split("/").last().unwrap().parse().unwrap();
        self.map.get(&id).unwrap().clone()
    }
}

#[test]
fn test_codec() {
    let mut codec = Codec::new();
    let url = "https://leetcode.com/problems/design-tinyurl".to_string();
    let short_url = codec.encode(url.clone());
    let long_url = codec.decode(short_url);
    assert_eq!(url, long_url);
}
