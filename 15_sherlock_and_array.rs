use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'balancedSums' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn balancedSums(arr: &[i32]) -> String {
    for i in 0..arr.len(){
        let left = &arr[0..i];
        let right = &arr[i+1..];
        if left.iter().sum::<i32>() == right.iter().sum::<i32>(){
            return "YES".to_string();
        }
    }
    "NO".to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let T = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..T {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = balancedSums(&arr);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
