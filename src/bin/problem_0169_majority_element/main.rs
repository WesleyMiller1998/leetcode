#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    let vec = vec!(2,2,1,1,1,2,2);

    let elem = majority_element(vec);

    println!("{}", elem);
}

// O(n) time complexity and O(n) space complexity
fn majority_element(nums: Vec<i32>) -> i32 {    
    let mut map: HashMap<i32, i32> = HashMap::new();

    for n in nums {
        let val = map.get(&n);

        let v = match val {
            None => 1,
            Some(&v) => v+1,
        };

        map.insert(n, v);
    }

    let mut max: Option<i32> = None;
    let mut max_n: Option<i32> = None;

    for (&k, &v) in map.iter() {
        if max_n.is_none() || max_n.is_some_and(|x| v > x) {
            max = Some(k);
            max_n = Some(v);
        }
    }

    max.unwrap_or(0)
}

// O(n) time complexity and O(1) space complexity
fn majority_element_2(nums: Vec<i32>) -> i32 {
    let mut majority = i32::MIN;
    let mut count = 0;

    for n in nums {
        if n == majority {
            count += 1;
        } else {
            if count == 0 {
                majority = n;
                count = 1;
            } else {
                count -= 1;
            }
        }
    }

    majority
}