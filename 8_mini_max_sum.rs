use std::io::{self, BufRead};
/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */


fn miniMaxSum(arr: &[i32]) {
    let mut min:i64 = i64::MAX;
    let mut max:i64 = 0;
    let mut combinations:Vec<Vec<i32>> = vec![];
    for i in 0..=4{
        let mut newArr:[i32;4] = [0;4];
        for j in 0..=3{
            newArr[j] = if j<i{arr[j]}else{arr[j+1]}
        }
        combinations.push(newArr.to_vec());
    }
    for combination in combinations{
        let mut sum:i64 = 0;
        for item in combination{
            sum+=item as i64;
        }
        if sum<min{
            min = sum;
        }
        if sum>max{
            max = sum;
        }
    }
    println!("{} {}", min, max);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
