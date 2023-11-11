#![allow(dead_code)]

fn main() {
    let vec = vec!(7,1,5,3,6,4);

    let profit = max_profit(vec);

    println!("{}", profit);
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = -1;
    let mut min = i32::MAX;

    for p in prices {
        if p < min {
            min = p;
        }

        if p - min > profit {
            profit = p - min;
        }
    }

    profit
}