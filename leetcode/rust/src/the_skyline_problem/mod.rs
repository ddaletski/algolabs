struct Solution;

use std::collections::BTreeMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum EventType {
    Start,
    End,
}

#[derive(Clone, Copy)]
struct Event {
    kind: EventType,
    height: i32,
}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut events = buildings
            .into_iter()
            .flat_map(|v| {
                let left = v[0];
                let right = v[1];
                let height = v[2];

                [
                    (
                        left,
                        Event {
                            kind: EventType::Start,
                            height,
                        },
                    ),
                    (
                        right,
                        Event {
                            kind: EventType::End,
                            height,
                        },
                    ),
                ]
            })
            .collect::<Vec<_>>();

        events.sort_by_key(|&(x, event)| (x, event.kind));

        let mut result: Vec<Vec<i32>> = vec![];
        let mut segments = BTreeMap::new();

        for (x, Event { kind, height }) in events {
            match kind {
                EventType::Start => {
                    segments
                        .entry(height)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                EventType::End => {
                    if let Some(freq) = segments.get_mut(&height) {
                        *freq -= 1;
                        if *freq == 0 {
                            // drop(freq);
                            segments.remove(&height);
                        }
                    }
                }
            };

            let current_max_height = segments
                .last_key_value()
                .map(|(k, _)| k)
                .cloned()
                .unwrap_or(0);
            if let Some(last_entry) = result.last() {
                let last_x = last_entry[0];
                let last_height = last_entry[1];

                if last_height == current_max_height {
                    continue;
                }
                if last_x == x && last_height != current_max_height {
                    result.pop();
                }
            }
            result.push(vec![x, current_max_height]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_returns, vec2d};

    #[test]
    fn case1() {
        let buildings = vec2d![
            [2, 9, 10],
            [3, 7, 15],
            [5, 12, 12],
            [15, 20, 10],
            [19, 24, 8]
        ];
        let expected = vec2d![
            [2, 10],
            [3, 15],
            [7, 12],
            [12, 0],
            [15, 10],
            [20, 8],
            [24, 0]
        ];

        assert_returns!(expected, Solution::get_skyline, buildings);
    }

    #[test]
    fn case2() {
        let buildings = vec2d![[0, 2, 3], [2, 5, 3]];
        let expected = vec2d![[0, 3], [5, 0]];

        assert_returns!(expected, Solution::get_skyline, buildings);
    }
}
