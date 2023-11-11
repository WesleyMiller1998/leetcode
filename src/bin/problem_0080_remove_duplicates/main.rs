#![allow(dead_code)]

fn main() {
    let mut vec = vec!(0,0,1,1,1,2,3,3);

    let n = remove_duplicates(&mut vec);

    println!("{:?}", vec);
    println!("{}", n);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32
    }

    let mut ptr: usize = 2;

    while ptr < nums.len() {
        if nums[ptr] == nums[ptr-1] && nums[ptr] == nums[ptr-2] {
            nums.remove(ptr);
        } else {
            ptr += 1;
        }
    }

    ptr as i32
}

fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32
    }

    let mut ptr: usize = 2;
    let mut total: usize = nums.len();

    while ptr < total {
        if nums[ptr] == nums[ptr-1] && nums[ptr] == nums[ptr-2] {
            shift_vec(nums, ptr);
            total -= 1;
        } else {
            ptr += 1;
        }
    }

    total as i32
}

fn shift_vec(nums: &mut Vec<i32>, idx: usize) {
    let v = nums[idx];

    for n in idx+1..nums.len() {
        nums[n-1] = nums[n];
    }

    let last = nums.len();
    nums[last - 1] = v;
}