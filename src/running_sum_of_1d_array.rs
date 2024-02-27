pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    nums.into_iter()
        .map(|n| {
            sum += n;
            sum
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::running_sum_of_1d_array::running_sum;

    #[test]
    fn when_array_size_is_one() {
        assert_eq!(vec![1], running_sum(vec![1]))
    }

    #[test]
    fn when_array_more_than_one() {
        assert_eq!(vec![1, 3], running_sum(vec![1, 2]))
    }

    #[test]
    fn when_array_some_number_is_less_than_zero() {
        assert_eq!(vec![1, 0], running_sum(vec![1, -1]))
    }
}
