use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let mer:&str = &s[s.len()-2..];
    let time_part = &s[..s.len()-2];
    
    let mut components:Vec<&str> = time_part.split(':').collect();
    
    let mut hours:i32=components[0].parse().expect("Failed hours");
    let minutes:i32=components[1].parse().expect("Failed minutes");
    let seconds:i32=components[2].parse().expect("Failed seconds");
    if mer == "AM"{
        if hours==12 {
            hours = 0;
        }
    }
    else{
        if hours != 12 {
            hours += 12;
        }
    }
    
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
