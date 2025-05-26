impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 1;

        while r < nums.len() {
            if nums[l] != nums[r] {
                l += 1;
                nums[l] = nums[r];
            }

            r += 1;
        }

        (l + 1) as i32
    }
}