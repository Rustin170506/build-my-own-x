use std::collections::BTreeMap;

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for ticket in tickets {
        graph
            .entry(ticket[0].clone())
            .or_default()
            .push(ticket[1].clone());
    }

    for destinations in graph.values_mut() {
        destinations.sort_by(|a, b| b.cmp(a)); // Sort in reverse order
    }

    let mut route = Vec::new();

    fn dfs(graph: &mut BTreeMap<String, Vec<String>>, airport: &str, route: &mut Vec<String>) {
        while let Some(destination) = graph.get_mut(airport).and_then(|dests| dests.pop()) {
            dfs(graph, &destination, route);
        }
        route.push(airport.to_string());
    }

    dfs(&mut graph, "JFK", &mut route);
    route.reverse();
    route
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
