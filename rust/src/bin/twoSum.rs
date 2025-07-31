use std::collections::HashMap;

struct Solution; 

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_index = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&complement_index) = num_to_index.get(&complement) {
                return vec![complement_index as i32, i as i32];
            }
            num_to_index.insert(num, i);
        }

        vec![]
    }
}

fn main() {
    let numbers: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result: Vec<i32> = Solution::two_sum(numbers, target);
    
    println!("Result: {:?}", result);
}
