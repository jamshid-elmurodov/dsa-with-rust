impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let un = n as usize;

        for i in 0..un {
            res.push(nums[i]);
            res.push(nums[i + un]);
        }

        res
    }
}