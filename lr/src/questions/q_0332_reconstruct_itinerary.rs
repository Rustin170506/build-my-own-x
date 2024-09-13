use std::collections::BTreeMap;

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();

    // Build the graph
    for ticket in tickets {
        graph
            .entry(ticket[0].clone())
            .or_default()
            .push(ticket[1].clone());
    }

    // Sort destinations in reverse order
    for destinations in graph.values_mut() {
        destinations.sort_by(|a, b| b.cmp(a));
    }

    let mut route = Vec::new();
    let mut stack = vec!["JFK".to_string()];

    // Hierholzer's algorithm
    while let Some(airport) = stack.last() {
        if let Some(destinations) = graph.get_mut(airport) {
            if destinations.is_empty() {
                route.push(stack.pop().unwrap());
            } else {
                let next = destinations.pop().unwrap();
                stack.push(next);
            }
        } else {
            route.push(stack.pop().unwrap());
        }
    }

    route.into_iter().rev().collect()
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
