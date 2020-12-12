use std::fs;
use std::io;
struct ShipPosition {
    x: isize,
    y: isize,
    rotation: isize
}

fn main() {
    let read_input = fs::read_to_string("input.txt").expect("could not read the text file");
    let instructions: Vec<&str> = read_input.split('\n').collect(); 

    let ship_position = &mut ShipPosition {
        x: 0,
        y: 0,
        rotation: 270 // treat N as 180 deg and S as 0
    };

    for instruction in instructions {
        execute_instruction(ship_position, instruction);
        /*let _tmp: &mut String = &mut String::new();
        io::stdin().read_line(_tmp);
        if let Ok(n) = get_ships_direction(ship_position.rotation) {
            
            println!("x {}  y {}  r {}, dir {}", ship_position.x, ship_position.y, ship_position.rotation, n);
        }
        */
    }

    let abs_x = ship_position.x.abs();
    let abs_y = ship_position.y.abs();
    println!("manhattan distance equals {}", abs_x + abs_y);
}

fn get_ships_direction(rotation: isize) -> Result<char, &'static str>{
    match (rotation % 360).abs() {
        90 => return Ok('W'),
        180 => return Ok('N'),
        270 => return Ok('E'),
        0 => return Ok('S'),
        _ => return Err("deficient rotation")
    };
}


fn execute_instruction(ship_position: &mut ShipPosition, instruction: &str) -> () {
    if instruction.is_empty() {
        return;
    } 

    let action: char = instruction.chars().nth(0).unwrap();
    let value: isize = instruction[1..].parse::<isize>().unwrap();
    match action {
        'N' => ship_position.x += value,
        'S' => ship_position.x -= value,
        'E' => ship_position.y += value,
        'W' => ship_position.y -= value,
        'L' => ship_position.rotation -= value,
        'R' => ship_position.rotation += value,
        'F' => {
            if let Ok(d) = get_ships_direction(ship_position.rotation) {
                let new_instruction =  format!("{}{}", d, value);
                
                execute_instruction(ship_position, &new_instruction[..]);

            }
        },
        _ => {}
    }
}