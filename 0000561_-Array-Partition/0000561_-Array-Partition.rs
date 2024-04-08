impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut num = 0;

        for (index, value) in nums.iter().enumerate() {
            if (index % 2 == 0) {
                num += value;
            }
        }

        num
    }
}
