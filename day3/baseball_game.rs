impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let mut sum: i32 = 0;

        for o in operations.iter() {
            match o.as_str() {
                "+" => {
                    let last2 = stack[stack.len() - 2];
                    let last1 = stack.last().unwrap();

                    let val = last2 + last1;
                    stack.push(val);
                    sum += val;
                }
                "D" => {
                    let val = stack.last().unwrap() * 2;
                    stack.push(val);
                    sum += val;
                }
                "C" => {
                    let num = stack.pop().unwrap();
                    sum -= num;
                }
                _ => {
                    let num: i32 = o.parse().unwrap();
                    stack.push(num);
                    sum += num;
                }
            }
        }

        sum
    }
}