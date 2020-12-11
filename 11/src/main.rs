use std::fs;
fn main() {
    let read_data: String =
        fs::read_to_string("input.txt").expect("Something went wrong when reading the file!");
    let lines: Vec<&str> = read_data.split('\n').collect();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for i in 0..lines.len() {
        if lines[i].is_empty() {
            continue;
        }
        grid.push(lines[i].chars().collect());
    }
    let mut iteration = 1;
    loop {
        let mut changes = 0;

        let mut pending_changes: Vec<(usize, usize)> = Vec::new();

        for i in 0..grid.len() {
            let row = &grid[i];
            for j in 0..row.len() {
                let col = &row[j];
                let mut adjacent_seats: Vec<char> = Vec::new();
            
                let (irow, icol) = (i as isize, j as isize);

                //create 3x3 window of surrounding seats
                for win_row in (irow - 1)..(irow + 2) {
                    for win_col in (icol - 1)..(icol + 2) {

                        if win_row == irow && win_col == icol {
                            continue;
                            // idk how to invert this condition 
                        }
                        else {
                            adjacent_seats.push(grid.get_value(win_col, win_row));
                            
                        }
                    }
                }  
                // check if iterated chair should change
                let occupied_seats = adjacent_seats.iter().filter(|&c| *c == '#').count();
                match grid[i][j] {
                    'L' => {
                        if occupied_seats == 0 {
                            pending_changes.push((i,j));
                            changes += 1;
                        }
                    },
                    '#' => {
                        if occupied_seats >= 4 {
                            pending_changes.push((i,j));
                            changes += 1;
                        }
                    }
                    _ => {}
                }

            }
        }

        // when the iteration finishes, apply changes to vector
        for changes in pending_changes {
            match grid[changes.0][changes.1] {
                'L' => {
                    grid[changes.0][changes.1] = '#';
                },
                '#' => {
                    grid[changes.0][changes.1] = 'L';
                }
                _ => {}
            }

        }
        
        println!("zakonczono iteracje {} z {} zmianami", iteration, changes);
        iteration += 1;

        if changes == 0 {
            break;
        }
    }
    // get count of occupied chairs
    let all_occupied_seats = grid.iter().flatten().filter(|&c| *c == '#').count();
    println!("occupied seats in the end: {}", all_occupied_seats);
}

trait GetValue {
    fn get_value(&self, x: isize, y: isize) -> char;
}

impl GetValue for Vec<Vec<char>> {
    //replacing all invalid indexes with floor 
    fn get_value(&self, x: isize, y: isize) -> char {
        if x < 0 || y < 0 {
            return '.';
        }
        match self.get(y as usize) {
            Some(r) => match r.get(x as usize) {
                Some(rr) => return *rr,
                None => return '.',
            },
            None => return '.',
        }
    }
}
