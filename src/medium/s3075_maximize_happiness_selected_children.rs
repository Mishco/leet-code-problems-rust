use std::collections::BinaryHeap;

pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
    let mut local_happiness = happiness;
    let mut heap: BinaryHeap<i32> = local_happiness.into_iter().collect();
    let mut total_happiness: i64 = 0;
    let mut desc: i64 = 0;

    for _ in 0..k {
        let mut num = (heap.pop().unwrap() as i64) - desc;
        if num < 0 {
            num = 0;
        }
        total_happiness += num;
        desc += 1;
    }

    return total_happiness;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(maximum_happiness_sum(Vec::from([1, 2, 3]), 2), 4);
        assert_eq!(maximum_happiness_sum(Vec::from([1, 1, 1, 1]), 2), 1);
    }
}