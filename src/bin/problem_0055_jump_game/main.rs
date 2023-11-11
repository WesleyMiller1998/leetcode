#![allow(dead_code)]

fn main() {
    let vec = vec![2,3,1,1,4];

    println!("{}", can_jump(vec));
}

fn can_jump(nums: Vec<i32>) -> bool {
    let mut max_idx: usize = 0;

    for ptr in 0..nums.len() - 1 {
        max_idx = max_idx.max(ptr + nums[ptr] as usize);

        if max_idx == ptr {
            return false
        }
    }
    
    true
}