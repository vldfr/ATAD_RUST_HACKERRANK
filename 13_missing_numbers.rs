use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'missingNumbers' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY arr
 *  2. INTEGER_ARRAY brr
 */

fn missingNumbers(arr: &[i32], brr: &[i32]) -> Vec<i32> {
    let mut missing:Vec<i32> = Vec::new();
    let mut arr_mut:Vec<i32> = arr.to_vec();
    let mut brr_mut:Vec<i32> = brr.to_vec();
    arr_mut.sort();
    brr_mut.sort();
    // println!("{:?}", arr_mut);
    // println!("{:?}", brr_mut);
    let mut a_i:usize=0;
    for b_i in 0..brr_mut.len() as usize {
        // print!("a:{} , b:{} - ", arr[a_i], brr[b_i]);
        if a_i >= arr_mut.len() || arr_mut[a_i] != brr_mut[b_i] {
            // print!("missing");
            if !missing.contains(&brr_mut[b_i]){
                // print!(" added");
                missing.push(brr_mut[b_i]);
            }
            // println!("");
        }
        else{
            // println!("same");
            a_i+=1;
        }
    }
    return missing;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = missingNumbers(&arr, &brr);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
