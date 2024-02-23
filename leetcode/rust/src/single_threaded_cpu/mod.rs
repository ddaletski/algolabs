pub struct Solution;

//////////////////////////////////////////////////////////

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug, Clone)]
struct TaskItem {
    duration: usize,
    id: i32,
    start_time: usize,
}

impl PartialOrd for TaskItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.duration.partial_cmp(&self.duration) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        other.id.partial_cmp(&self.id)
    }
}

impl Ord for TaskItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.duration.cmp(&self.duration) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        other.id.cmp(&self.id)
    }
}

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks: Vec<TaskItem> = tasks
            .into_iter()
            .enumerate()
            .map(|(id, item)| TaskItem {
                id: id as i32,
                start_time: item[0] as usize,
                duration: item[1] as usize,
            })
            .collect();
        tasks.sort_unstable_by_key(|task| task.start_time);

        let mut order = vec![];
        let tasks_count = tasks.len();

        let mut heap = BinaryHeap::with_capacity(tasks_count);
        let mut end = 0;

        let mut src_iter = tasks.into_iter().peekable();
        while order.len() < tasks_count {
            while let Some(task) = src_iter.peek() {
                if task.start_time > end {
                    break;
                }

                heap.push(task.clone());
                src_iter.next();
            }

            if let Some(task) = heap.pop() {
                end += task.duration;
                order.push(task.id);
            } else if let Some(task) = src_iter.peek() {
                end = task.start_time;
            }
        }

        order
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;
    use rstest::rstest;

    #[rstest]
    #[case(vec![[1,2],[2,4],[3,2],[4,1]], vec![0,2,3,1])]
    #[case(vec![[7,10],[7,12],[7,5],[7,4],[7,2]], vec![4,3,2,0,1])]
    #[case(vec![[19,13],[16,9],[21,10],[32,25],[37,4],[49,24],[2,15],[38,41],[37,34],[33,6],[45,4],[18,18],[46,39],[12,24]], vec![6,1,2,9,4,10,0,11,5,13,3,8,12,7])]
    fn it_works(#[case] tasks: Vec<[i32; 2]>, #[case] expected_order: Vec<i32>) {
        let tasks = tasks.into_iter().map(|x| x.into()).collect();

        assert_returns!(expected_order, Solution::get_order, tasks);
    }
}
