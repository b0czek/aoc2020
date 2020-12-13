use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read the input file");
    let data: Vec<&str> = input.split('\n').collect();
    
    let bus_ids: Vec<usize> = data[1].split(',').filter(|id| id != &"x").map(|id| id.parse::<usize>().unwrap()).collect();
    
    let timestamp: usize = data[0].parse().unwrap();
    let mut closest_bus_time: usize = usize::MAX;
    let mut closest_bus: usize = 0;
    
    for bus_id in &bus_ids {
        let mut local_timestamp = 0;
        while local_timestamp < timestamp {
            local_timestamp += bus_id;
        }
        if local_timestamp < closest_bus_time {
            closest_bus_time = local_timestamp;
            closest_bus = *bus_id;
        }
        
    }
    let minutes_needed_to_wait = closest_bus_time - timestamp;
    println!("closes bus has an id of {} and will arrive in {} minutes. the puzzle answer is {}", closest_bus, minutes_needed_to_wait, closest_bus * minutes_needed_to_wait);
    
    //part 2
    let mut remainders: Vec<i64> = Vec::new(); 
    let mut modulis: Vec<i64> = Vec::new();  
    let mut t_offset: i64 = 0;
    for raw_data in data[1].split(',').collect::<Vec<&str>>() {
        if raw_data == "x" {
            t_offset += 1;
            continue;
        }
        let bus_id:i64 = raw_data.parse().unwrap();
        modulis.push(bus_id);
        
        remainders.push(if t_offset == 0 { 0 } else { bus_id - t_offset });
        t_offset += 1;
    }

    let result = chinese_remainder(&remainders, &modulis).unwrap();
    println!("result for part is {}", result);
}


// im too stupid and tired to write that code unfortunately

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &Vec<i64>, modulii: &Vec<i64>) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}
 