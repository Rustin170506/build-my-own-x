use std::collections::{HashSet, VecDeque};

pub fn walls_and_gates(rooms: Vec<Vec<i32>>) {
    let mut rooms = rooms;
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    for r in 0..rooms.len() {
        for c in 0..rooms[0].len() {
            if rooms[r][c] == 0 {
                q.push_back((r as i32, c as i32));
                visited.insert((r as i32, c as i32));
            }
        }
    }

    let mut dist = 0;

    while !q.is_empty() {
        let q_len = q.len();
        for _ in 0..q_len {
            let (r, c) = q.pop_front().unwrap();
            rooms[r as usize][c as usize] = dist;
            add_room(&mut q, &mut visited, &rooms, r + 1, c);
            add_room(&mut q, &mut visited, &rooms, r - 1, c);
            add_room(&mut q, &mut visited, &rooms, r, c + 1);
            add_room(&mut q, &mut visited, &rooms, r, c - 1);
        }
        dist += 1
    }
}

fn add_room(
    q: &mut VecDeque<(i32, i32)>,
    visited: &mut HashSet<(i32, i32)>,
    rooms: &[Vec<i32>],
    r: i32,
    c: i32,
) {
    if (r >= 0 && r < rooms.len() as i32)
        && (c >= 0 && c < rooms[0].len() as i32)
        && rooms[r as usize][c as usize] != -1
        && !visited.contains(&(r, c))
    {
        q.push_back((r, c));
        visited.insert((r, c));
    }
}
