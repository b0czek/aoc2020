
use std::fs;
const PREAMBLE_SIZE: usize = 25;
fn main() {
    let read_data =
    fs::read_to_string("input.txt").expect("Something went wrong when reading the file");
    // split by new lines after reading file
    let lines: Vec<&str> = read_data.split('\n').collect();
    let mut numbers: Vec<u64> = Vec::new();
    let mut preamble: Vec<u64> = Vec::new();
    // push them into u64 vector 
    for line in lines {
        match line.parse::<u64>() {
            Ok(n) => {
                if preamble.len() < PREAMBLE_SIZE {
                    preamble.push(n);
                }
                else {
                    numbers.push(n);
                }
            },
            Err(_) => continue
        }

    }
    let numbers_len = numbers.len();
    for n in 0..numbers_len - 1 {
        let number = numbers[n];
        println!("{} {}",n, number);
        match find_summing_number(number, preamble.clone()) {
            Ok(_) =>{
                preamble = preamble.drain(1..).collect();
                preamble.push(number);
            },
            Err(_) => {
                println!("{} {} {} ", number, numbers[633], n);
                for i in 2..n {
                    for j in 0..numbers_len - i {
                        let mut sum: u64 = 0;
                        let mut sumComponents: Vec<u64> = Vec::new();
                        for k in 0..i + 1 {
                            // println!("i {} j {} k {}, j+k {}", i,j,k, j+k);
                            sumComponents.push(numbers[j+k]);
                            sum += numbers[j+k];
                        }
                        if sum == number {
                            sumComponents.sort();
                            println!("znalazlem ino rigcz {} i {}", sumComponents[0], sumComponents[sumComponents.len() - 1]);
                            return;

                        }
                    }
                }
                println!("niestety nie udalo sie znalezc rigczu");
                return;
            }
        }
    }
}

fn find_summing_number(desired_number:u64, search_pool:Vec<u64>) -> Result<(u64, u64), &'static str> {
    for i in 0..search_pool.len() {
        let selector: u64 = search_pool[i];
        for j in 0.. search_pool.len() {
            let cursor = search_pool[j];
            if cursor + selector == desired_number {
                return Ok((selector, cursor));
            }
        }
    }
    return Err("aaaa");
}