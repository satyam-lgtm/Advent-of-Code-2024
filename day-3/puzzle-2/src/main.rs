use std::fs;
use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    
    let mut hay: String = "".to_string();

    let mut keep_doing = true;
    for i in 0..contents.len() {
        if i >= 6 && &contents[i - 6..i + 1] == "don't()" {
            keep_doing = false;
        } else if i >= 3 && &contents[i - 3..i + 1] == "do()" {
            keep_doing = true;
        } else {
            if keep_doing {
                hay.push(contents.chars().nth(i).unwrap());
            }
        }
    }

    let mul: Vec<&str> = re.find_iter(hay.as_str()).map(|m| m.as_str()).collect();
    
    let mut result: i64 = 0;

    for i in 0..mul.len() {
        let num = Regex::new(r"\d+").unwrap();
        let nums: Vec<&str> = num.find_iter(mul[i]).map(|m| m.as_str()).collect();
        result += nums[0].parse::<i64>().unwrap() * nums[1].parse::<i64>().unwrap();
    }

    println!("{result}");
}