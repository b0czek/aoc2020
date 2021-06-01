use std::collections::HashMap;

fn main() {
    let input = vec![11, 18, 0, 20, 1, 7, 16];

    println!("2020 word: {}", calculate_spoken_number(&input, 2020));
    println!(
        "30 millionth word: {}",
        calculate_spoken_number(&input, 30000000)
    );
}

fn calculate_spoken_number(input: &Vec<usize>, nth_spoken_number: usize) -> usize {
    let mut last_occurences: HashMap<usize, usize> = HashMap::new();

    for i in 0..input.len() - 1 {
        last_occurences.insert(input[i], i);
    }
    let mut last_spoken_number = *input.last().unwrap();

    for i in input.len() - 1..nth_spoken_number - 1 {
        let new_number: usize;
        match last_occurences.get_key_value(&last_spoken_number) {
            Some(x) => new_number = i - *(x.1),
            None => new_number = 0,
        }
        last_occurences.insert(last_spoken_number, i);
        last_spoken_number = new_number;
    }
    last_spoken_number
}
