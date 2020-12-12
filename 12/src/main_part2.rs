use std::fs;
use std::io;
struct ShipPosition {
    ship_x: isize,
    ship_y: isize,
    waypoint_x: isize,
    waypoint_y: isize
}

fn main() {
    let read_input = fs::read_to_string("input.txt").expect("could not read the text file");
    let instructions: Vec<&str> = read_input.split('\n').collect(); 

    let ship_position = &mut ShipPosition {
        ship_x: 0,
        ship_y: 0,
        waypoint_x: 10,
        waypoint_y: 1
    };

    for instruction in instructions {
        execute_instruction(ship_position, instruction);
        /*let _tmp: &mut String = &mut String::new();
        io::stdin().read_line(_tmp);
        println!("xw {}  yw {}  xs {}, ys {}", ship_position.waypoint_x, ship_position.waypoint_y, ship_position.ship_x, ship_position.ship_y);
*/
        
    }

    let abs_x = ship_position.ship_x.abs();
    let abs_y = ship_position.ship_y.abs();
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
        'N' => ship_position.waypoint_y += value,
        'S' => ship_position.waypoint_y -= value,
        'E' => ship_position.waypoint_x += value,
        'W' => ship_position.waypoint_x -= value,
        'R' => {
            // inefficient af
            let runs = value / 90;
            for _ in 0..runs {
                let new_x = ship_position.waypoint_y;
                let new_y = -ship_position.waypoint_x;
                ship_position.waypoint_x = new_x;
                ship_position.waypoint_y = new_y;
            } 
        },
        'L' => {
            let runs = value / 90;
            for _ in 0..runs {
                let new_x = -ship_position.waypoint_y;
                let new_y = ship_position.waypoint_x;
                ship_position.waypoint_x = new_x;
                ship_position.waypoint_y = new_y;
            } 
        },
        'F' => {
            ship_position.ship_x += value * ship_position.waypoint_x;
            ship_position.ship_y += value * ship_position.waypoint_y;
        },
        _ => {}
    }
}