// Definition of Interval:
#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
    let mut starts = vec![];
    let mut ends = vec![];

    for interval in intervals {
        starts.push(interval.start);
        ends.push(interval.end);
    }

    starts.sort();
    ends.sort();

    let mut rooms = 0;
    let mut end_index = 0;

    for start in starts {
        if start < ends[end_index] {
            rooms += 1;
        } else {
            end_index += 1;
        }
    }

    rooms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_meeting_rooms() {
        assert_eq!(
            min_meeting_rooms(vec![
                Interval::new(0, 30),
                Interval::new(5, 10),
                Interval::new(15, 20)
            ]),
            2
        );
        assert_eq!(
            min_meeting_rooms(vec![
                Interval::new(7, 10),
                Interval::new(2, 4),
                Interval::new(15, 20)
            ]),
            1
        );
        assert_eq!(
            min_meeting_rooms(vec![
                Interval::new(7, 10),
                Interval::new(2, 4),
                Interval::new(15, 20),
                Interval::new(0, 30)
            ]),
            2
        );
    }
}
