use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::BTreeSet;

/*
 * Complete the 'minimumLoss' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts LONG_INTEGER_ARRAY price as parameter.
 */

// fn minimumLoss(price: &[i64]) -> i32 {
//     let mut minLoss:i64 = i64::MAX;
//     for i in 0..price.len(){
//         for j in (i+1)..price.len(){
//             if price[i]>price[j] && price[i]-price[j]< minLoss{
//                 minLoss = price[i]-price[j];
//             }
//         }
//     }
//     minLoss as i32
// }
// failed time constraints

fn minimumLoss(price: &[i64]) -> i32 {
    let mut seen_prices = BTreeSet::new();
    let mut min_loss = i64::MAX;
    for &current_price in price {
        // Find the smallest price in seen_prices that is greater than current_price
        if let Some(&higher_price) = seen_prices.range(current_price + 1..).next() {
            let loss = higher_price - current_price;
            if loss < min_loss {
                min_loss = loss;
            }
        }

        // Insert the current price into the set
        seen_prices.insert(current_price);
    }
    min_loss as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let price: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    let result = minimumLoss(&price);

    writeln!(&mut fptr, "{}", result).ok();
}
