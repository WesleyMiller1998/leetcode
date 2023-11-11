#![allow(dead_code)]

fn main() {
    let mut vec = vec!(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27);

    rotate_3(&mut vec, 11);

    println!("{:?}", vec);
}  

fn rotate(nums: &mut Vec<i32>, k:i32) {
    if nums.len() <= 1 {
        return
    }

    let rotations = (k as usize) % nums.len();

    let split_idx = nums.len() - rotations;
    let mut new_vec: Vec<i32> = Vec::with_capacity(nums.len());

    new_vec.extend_from_slice(&nums[split_idx..nums.len()]);
    new_vec.extend_from_slice(&nums[0..split_idx]);

    nums.clone_from(&new_vec);
}

fn rotate_2(nums: &mut Vec<i32>, k:i32) {
    let k = k as usize;
    let n = nums.len();
    nums.rotate_right(k % n);
}

fn rotate_3(nums: &mut Vec<i32>, k:i32) {
    let k = k as usize % nums.len();
    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
}