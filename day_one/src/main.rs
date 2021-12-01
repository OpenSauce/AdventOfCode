use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let FILEINDEX = "./resources/input.txt";
    let mut values: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(FILEINDEX) {
        for line in lines {
            if let Ok(ip) = line {
                let val = ip.parse().unwrap();
                values.push(val)
            }
        }
    }

    println!("{}", stage_one(&mut values));
    println!("{}", stage_two(&mut values));
}

fn stage_one(values: &mut Vec<i32>) -> i32 {
    let mut count = 0;
    let mut previousValue = 0;

    for val in values.iter_mut() {
        if *val > previousValue && previousValue != 0 {
            count += 1;
        }
        previousValue = *val;
    }

    return count;
}

fn stage_two(values: &mut Vec<i32>) -> i32 {
    let mut count = 0;
    let mut previousValue = 0;
    let mut three = 0;
    let mut two = 0;
    let mut one = 0;
    let mut fullCom = 0;

    for val in values.iter_mut() {
        one = two;
        two = three;
        three = *val;
        fullCom = one + two + three;
        if one != 0 && fullCom > previousValue {
            count += 1;
        }
        previousValue = fullCom;
    }

    return count - 1;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
