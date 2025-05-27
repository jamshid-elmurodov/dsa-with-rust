impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut res = nums.clone();
        nums.iter().for_each(|n| res.push(*n));
        res
    }
}