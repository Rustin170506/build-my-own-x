use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let euclidean_distance1 = (((self.x * self.x) + (self.y * self.y)) as f64).sqrt();
        let euclidean_distance2 = (((other.x * other.x) + (other.y * other.y)) as f64).sqrt();
        euclidean_distance2.partial_cmp(&euclidean_distance1)
    }
}

impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let euclidean_distance1 = (((self.x * self.x) + (self.y * self.y)) as f64).sqrt();
        let euclidean_distance2 = (((other.x * other.x) + (other.y * other.y)) as f64).sqrt();
        euclidean_distance2
            .partial_cmp(&euclidean_distance1)
            .unwrap()
    }
}

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let points: Vec<Point> = points.iter().map(|p| Point::new(p[0], p[1])).collect();

    let mut heap = BinaryHeap::from(points);

    let mut result = Vec::new();
    for _ in 0..k {
        result.push(heap.pop().unwrap())
    }

    result.iter().map(|p| vec![p.x, p.y]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_closest() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let expected = vec![vec![-2, 2]];
        assert_eq!(k_closest(points, k), expected);

        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let expected = vec![vec![3, 3], vec![-2, 4]];
        assert_eq!(k_closest(points, k), expected);
    }
}
