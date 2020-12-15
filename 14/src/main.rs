use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("could not read the input file");

    let mut current_bitmask: Vec<(usize, usize)> = Vec::new(); // position and value 
    let mut memory: [usize; 1000000] = [0; 1000000]; // i guess its fine but should not have happened
    let instructions: Vec<&str> = input_file.split('\n').filter(|i| *i != "").collect();
    //part 1
    for instruction in &instructions {
        let command: &str = &instruction[..instruction.find(char::is_whitespace).unwrap()];
        match command {
            "mask" => {
                let mask: &str = &instruction[instruction.find('=').unwrap() + 2..]; // slice string to = sign and offset it to remove space
                current_bitmask = Vec::new();
                for (i, c) in mask.chars().enumerate() {
                    if c != 'X' {
                        current_bitmask.push((i, c.to_digit(10).unwrap() as usize));
                    }
                }
            },
            mem => {
                let address: &str = &mem[4..mem.len() -1];
                let value: &str = &instruction[instruction.find('=').unwrap() + 2..];
                memory[address.parse::<usize>().unwrap()] = apply_bitmask_p1(&current_bitmask, value.parse().unwrap());
            }
        };
    }
    
    let mut sum: usize = 0;
    memory.iter().for_each(|v| sum+=v);
    println!("part 1 answer: {}", sum);
    
    //part 2
    let mut current_bitmask: &str = "";
    let mut memory_2: Vec<(usize, usize)> = Vec::new();
    for instruction in &instructions {
        let command: &str = &instruction[..instruction.find(char::is_whitespace).unwrap()];
        match command {
            "mask" => {
                current_bitmask = &instruction[instruction.find('=').unwrap() + 2..];
            },
            mem => {
                let address = (&mem[4..mem.len() -1]).parse::<usize>().unwrap();
                let value = (&instruction[instruction.find('=').unwrap() + 2..]).parse::<usize>().unwrap();
                for address in apply_bitmask_p2(current_bitmask, address) {
                    match memory_2.iter().position(|&m| m.0 == address) { //inefficient af but cant index vector differently and normal array just kept overflowing
                        Some(n) => {
                            memory_2[n] = (address, value);
                        },
                        None => {
                            memory_2.push((address, value));
                        }
                    }
                }
            }
            
        }
    }
    let mut sum = 0;
    memory_2.iter().for_each(|v| sum+=v.1);
    println!("part 2 answer: {}", sum);
    
}


fn get_combinations(bitmask: &str) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    match bitmask.find('X') {
        Some(_) => {
            for i in 0..2 { // 0 and 1
                let next: Vec<usize> = get_combinations(&bitmask.replacen('X', &i.to_string(), 1));
                result = [&result[..], &next[..]].concat();
            }
        },
        None => {
            result.push(usize::from_str_radix(bitmask, 2).unwrap());
        }
    } 
    result
}

fn apply_bitmask_p2(bitmask: &str, address: usize) -> Vec<usize> {
    
    let mut mask: String = String::new();
    for (b,a) in bitmask.chars().zip(format_radix(address as u32, 2).chars()) {
        if b == '0' {
            mask.push(a);
        }
        else {
            mask.push(b);
        }
    }

    return get_combinations(&mask);
}



fn apply_bitmask_p1(bitmask: &Vec<(usize, usize)>, value: usize) -> usize {
    let mut bin_value: Vec<char> = format_radix(value as u32, 2).chars().collect();
    for (position, value) in bitmask {
        
        bin_value[*position] = if *value == 1 { '1' } else { '0' };
    }
    let output = bin_value.into_iter().collect::<String>();
    usize::from_str_radix(&output, 2).unwrap()
}

fn add_padding(string: String) -> String {
    return format!("{:0>36}", string);
}


fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result: Vec<char> = Vec::new();

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    add_padding(result.into_iter().rev().collect())
}