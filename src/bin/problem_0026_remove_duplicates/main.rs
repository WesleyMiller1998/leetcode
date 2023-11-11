#![allow(dead_code)]

fn main() {
    let mut vec = vec!(1,2,2,3,4,4,4,5,6,6);
    
    let removed = remove_duplicates(&mut vec);

    println!("{:?}", vec);
    println!("{}", removed);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut new_vec: Vec<i32> = Vec::with_capacity(nums.len());

    if nums.len() <= 1 {
        return nums.len() as i32
    }

    new_vec.push(nums[0]);

    for n in 1..nums.len() {
        if nums[n-1] != nums[n] {
            new_vec.push(nums[n]);
        }
    }

    nums.clone_from(&new_vec);
    nums.len() as i32
}

fn remove_duplicates_idiomatic(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();

    nums.len() as i32
}