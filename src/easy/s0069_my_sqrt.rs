#[warn(dead_code)]
pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }

    let mut left = 1;
    let mut right = x;
    let mut mid: i32;

    while left <= right {
        mid = left + (right - left) / 2;

        if mid == x / mid {
            return mid;
        } else if mid < x / mid {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return right;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(121), 11);
    }

}
