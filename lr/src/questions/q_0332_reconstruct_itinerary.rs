use std::collections::HashMap;

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut tickets = tickets;
    tickets.sort_by(|a, b| {
        if a[0] == b[0] {
            a[1].cmp(&b[1])
        } else {
            a[0].cmp(&b[0])
        }
    });

    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for ticket in &tickets {
        let src = ticket[0].clone();
        let dst = ticket[1].clone();
        adj.entry(src)
            .and_modify(|v| v.push(dst.clone()))
            .or_insert(vec![dst.clone()]);
    }
    let mut res = vec!["JFK".to_owned()];
    dfs("JFK".to_owned(), &tickets, &mut adj, &mut res);

    res
}

fn dfs(
    src: String,
    tickets: &Vec<Vec<String>>,
    adj: &mut HashMap<String, Vec<String>>,
    res: &mut Vec<String>,
) -> bool {
    if res.len() == tickets.len() + 1 {
        return true;
    }

    if !adj.contains_key(&src) {
        return false;
    }

    let mut i = 0;
    while let Some(v) = adj
        .get_mut(&src)
        .and_then(|neighbors| neighbors.get(i).cloned())
    {
        adj.get_mut(&src).unwrap().remove(i);
        res.push(v.clone());
        if dfs(v.clone(), tickets, adj, res) {
            return true;
        }
        adj.get_mut(&src).unwrap().insert(i, v);
        res.pop();
        i += 1;
    }

    false
}

#[test]
fn test_find_itinerary() {
    let tickets = vec![
        vec!["MUC".to_owned(), "LHR".to_owned()],
        vec!["JFK".to_owned(), "MUC".to_owned()],
        vec!["SFO".to_owned(), "SJC".to_owned()],
        vec!["LHR".to_owned(), "SFO".to_owned()],
    ];
    let res = find_itinerary(tickets);
    assert_eq!(
        res,
        vec![
            "JFK".to_owned(),
            "MUC".to_owned(),
            "LHR".to_owned(),
            "SFO".to_owned(),
            "SJC".to_owned()
        ]
    );
}
