#![allow(dead_code)]

fn main() {
    let vec = vec![7,1,5,3,6,4];

    let profit = max_profit(vec);

    println!("{}", profit);
}

fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0
    }
    
    let mut profit = 0;

    for i in 0..prices.len() - 1 {
        if prices[i + 1] - prices[i] > 0{
            profit += prices[i + 1] - prices[i];
        }
    }

    profit
}

fn max_profit_2(prices: Vec<i32>) -> i32 {
    prices.windows(2).fold(0, |profit, window| profit + (window[1] - window[0]).max(0))
}