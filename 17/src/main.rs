use std::fs;

mod cubes;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("could not read given file")
}

fn main() {
    let input_data = read_input("input.txt");
    let mut data = cubes::Cubes::new(&input_data);
    for _ in 0..6 {
        data.cycle_cubes();
    }

    println!("{:?}", data);
    println!("{}", data.count_active_cubes());
}
