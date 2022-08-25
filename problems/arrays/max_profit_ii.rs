// Best Time to Buy and Sell Stock II - https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/

/* Problem Statement:
 * You are given an integer array prices where prices[i] is the price of a
 * given stock on the ith day. On each day, you may decide to buy and/or sell the stock.
 * You can only hold at most one share of the stock at any time.
 * However, you can buy it then immediately sell it on the same day.
 * Find and return the maximum profit you can achieve.
 */

/* Example:
 *
 * Input: prices = [7,1,5,3,6,4]
 * Output: 7
 * Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
   Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
   Total profit is 4 + 3 = 7.
 *
 * Input: prices = [1,2,3,4,5]
 * Output: 4
 * Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
   Total profit is 4.
 *
 * Input: prices = [7,6,4,3,1]
 * Output: 0
 * Explanation: There is no way to make a positive profit, so we never buy the stock to achieve the maximum profit of 0.
*/

use std::collections::HashMap;

fn main() {
    let tests: HashMap<Vec<i32>, i32> = HashMap::from([
        (vec![7, 1, 5, 3, 6, 4], 7),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![7, 6, 4, 3, 1], 0),
    ]);

    for (prices, expected) in tests {
        let res = max_profit_ii(prices);
        if res == expected {
            println!("Valid max profit: {}", res);
        } else {
            println!("Invalid max profit. Expected: {} | Got: {}", expected, res);
        }
    }
}

fn max_profit_ii(prices: Vec<i32>) -> i32 {
    let mut max: i32 = 0;

    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            max += prices[i] - prices[i - 1];
        }
    }

    max
}
