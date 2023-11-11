#![allow(dead_code)]

fn main() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let m = 3;
    let mut nums2 = vec![2,5,6];
    let n = 3;

    merge(&mut nums1, m, &mut nums2, n);

    println!("{:?}", &nums1);
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n:i32) {
    let (mut m, mut n) = (m as usize, n as usize);

    while n > 0 {
        if m > 0 && nums1[m - 1] > nums2[n - 1] {
            nums1[m + n - 1] = nums1[m - 1];
            m -= 1;
        } else {
            nums1[m + n - 1] = nums2[n - 1];
            n -= 1;
        }
    }
}