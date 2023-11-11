#![allow(dead_code)]

fn main() {
    let mut vec = vec!(3,3);
    let mut v = remove_element(&mut vec, 3);

    println!("Remove element original");
    println!("{:?}", vec);
    println!("{}", v);

    vec = vec!(1,2,3,4,5,6,3,4,3,3);
    v = remove_element2(&mut vec, 3);

    println!("Remove element idiomatic");
    println!("{:?}", vec);
    println!("{}", v);
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut total = nums.len();

    if total == 0 {
        return 0;
    }

    let mut ptr:usize = 0;

    while ptr < total {
        if nums.get(ptr).unwrap().eq(&val) {
            let val = *nums.get(ptr).unwrap();
            nums[ptr] = nums[total - 1];
            nums[ptr] = val;
            total -= 1;
        } else {
            ptr += 1;
        }
    }

    total as i32
}

fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|x| *x != val);
    nums.len() as i32
}