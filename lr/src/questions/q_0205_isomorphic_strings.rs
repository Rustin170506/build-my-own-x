use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    assert_eq!(s.len(), t.len());

    let mut st_map = BTreeMap::new();
    let mut ts_map = BTreeMap::new();

    for i in 0..s.len() {
        let sc = s.chars().nth(i).unwrap();
        let tc = t.chars().nth(i).unwrap();

        if let Entry::Vacant(e) = st_map.entry(sc) {
            e.insert(tc);
        } else if st_map.get(&sc).unwrap() != &tc {
            return false;
        }

        if let Entry::Vacant(e) = ts_map.entry(tc) {
            e.insert(sc);
        } else if ts_map.get(&tc).unwrap() != &sc {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert!(is_isomorphic("egg".to_string(), "add".to_string()));
        assert!(!is_isomorphic("foo".to_string(), "bar".to_string()));
        assert!(is_isomorphic("paper".to_string(), "title".to_string()));
    }
}
