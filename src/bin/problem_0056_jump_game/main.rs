#![allow(dead_code)]

fn main() {
    let vec = vec![2,3,0,1,4];

    println!("{}", jump_greedy(vec))
}

// my original DP method
fn jump(nums: Vec<i32>) -> i32 {
    let mut dist = vec![i32::MAX; nums.len()];

    let last = dist.len() - 1;
    dist[last] = 0;

    for ptr in (0..nums.len()-1).rev() {
        let max_ptr = (ptr + 1 + nums[ptr] as usize).min(nums.len());

        for n_ptr in ptr+1..max_ptr {
            let d = dist[n_ptr];
            if d != i32::MAX {
                dist[ptr] = dist[ptr].min(d+1);
            }
        }
    }

    dist[0]
}

// DP method
fn jump_2(nums: Vec<i32>) -> i32 {
    let mut dist = vec![i32::MAX; nums.len()];
    let end = dist.len() - 1;
    dist[end] = 0;

    for ptr in (0..nums.len()-1).rev() {
        let max_ptr = (ptr + 1 + nums[ptr] as usize).min(nums.len());

        let m = dist[ptr+1..max_ptr].iter().min();

        if m.is_none() || *m.unwrap() == i32::MAX {
            continue
        }

        dist[ptr] = 1 + *m.unwrap();
    }

    dist[0]
}

fn jump_greedy(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return (nums.len() - 1) as i32;
    }

    let (mut min_dist, mut current_window_end, mut next_window_end): (usize, usize, usize) = (0, 0, 0);

    for ptr in 0..nums.len()-1 {
        let n = nums[ptr] as usize;

        next_window_end = next_window_end.max(ptr + n);

        if ptr == current_window_end {
            min_dist += 1;
            current_window_end = next_window_end;
        }

        if current_window_end >= nums.len() - 1 {
            break;
        }
    }

    min_dist as i32
}