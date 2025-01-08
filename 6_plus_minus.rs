use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let len = arr.len();
    let mut pos:i32 = 0;
    let mut neg:i32 = 0;
    let mut zero:i32 = 0;
    for item in arr{
        if item>&0{
            pos+=1;
        }
        if item<&0{
            neg+=1;
        }
        if item==&0{
            zero+=1;
        }
    }
    println!("{:.6}", (pos as f32)/(len as f32));
    println!("{:.6}", (neg as f32)/(len as f32));
    println!("{:.6}", (zero as f32)/(len as f32));
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
