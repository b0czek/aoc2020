

use std::fs;

fn main() {
    let read_data: String = fs::read_to_string("input.txt")
        .expect("Something went wrong when reading the file!");
    
    let lines: Vec<&str> = read_data.split('\n').collect();
    let mut adapter_joltages: Vec<usize> = Vec::new();
    
    for line in lines {
        match line.parse::<usize>() {
            Ok(j) => {
                adapter_joltages.push(j);
            },
            Err(_) => continue
        }
    }
    

    adapter_joltages.sort();
    let differences: [usize; 3];
    match get_joltage_diff(0, adapter_joltages.clone()) {
        Ok(d) => differences = d,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
    
    println!("there were {} differences of 1 and {} of 3", differences[0], differences[2]);
    println!("multiplied giving {}", differences[0] * differences[2]);
    
    adapter_joltages.push(0); //starting at 0
    adapter_joltages.sort(); // resort
    let chunks = detect_chunks(adapter_joltages.clone());

    let mut combinations = 1;
    for chunk in chunks {
        let combs: u32 = chunk.len() as u32 - 2;
        let base: usize = 2;
        let mut eglible_combinations = base.pow(combs) as u64;

        for i in 0..eglible_combinations {
            
            if eglible_combinations == 8 && i == 0 { // max size of mutable chunk is 3 where only 000 is bad combination so why not 
                eglible_combinations -= 1;
            }
        }
        combinations *= eglible_combinations;

    }
    println!("combs {}", combinations);
}

fn get_joltage_diff(last_joltage: usize, mut joltages_list: Vec<usize>) -> Result<[usize; 3], &'static str> {
    // keep track of differences in joltage in vector 
    let mut differences: [usize; 3] = [0,0,0];
    if joltages_list.len() == 0 {
        // built in diff 3 converter into the device
        return Ok([0,0,1]);
    }
    let diff = joltages_list[0] - last_joltage;
    
    if diff > 3 || diff < 1 {
        return Err("invalid difference");
    } 

    differences[diff - 1] += 1;

    let next_numbers: [usize; 3];
    match get_joltage_diff(joltages_list[0], joltages_list.drain(1..).collect()) {
        Ok(n) => next_numbers = n,
        Err(e) => {
            return Err(e);
        }
    }
    
    for i in 0..next_numbers.len() {
        differences[i] += next_numbers[i];
    }

    return Ok(differences);

    
}
fn are_joltages_eglible(joltages_list: Vec<usize>) -> bool {
    match get_joltage_diff(0, joltages_list) {
        Ok(_) => return true,
        Err(_) => return false
    }
}


fn detect_chunks(joltages_list: Vec<usize>) -> Vec<Vec<usize>> {



    let mut chunks: Vec<Vec<usize>> = Vec::new();

    let mut i = 1;
    while i < joltages_list.len() - 1 { // 2 last lines in my dataset went missing but they are mandatory anyway so i guess i will just save myself from avoiding indexing out of bounds
        // println!("{}", joltages_list[i]);
        let mut last_joltage: usize = joltages_list[i -1];
        let mut chunk: Vec<usize> = Vec::new();
        let mut first_of_chunk: bool = true;
        while joltages_list[i] - last_joltage == 1 { // there are no differences of 2 in my dataset so thatll do
            if first_of_chunk {
                // println!("{}", joltages_list[i - 1]);
                chunk.push(i - 1);
                first_of_chunk = false;
            }
            // println!("{}", joltages_list[i]);
            chunk.push(i);
            last_joltage = joltages_list[i];
            i+=1;
        }
        if chunk.len() == 0 {
            i+=1;
        }
        else {
            // println!(" ----------- one chunk ------------");
            chunks.push(chunk);
        }
    }

    return chunks;
}