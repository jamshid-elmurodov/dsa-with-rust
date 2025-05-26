impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            if nums[l] == val && nums[r] != val {
                nums[l] = nums[r];
                nums[r] = val;
                l += 1;
            } else if nums[l] != val {
                l += 1;
            }

            r += 1;
        }

        l as i32
    }
}