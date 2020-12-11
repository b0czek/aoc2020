use std::fs;
fn main() {
    let read_data: String =
        fs::read_to_string("input.txt").expect("Something went wrong when reading the file!");
    let lines: Vec<&str> = read_data.split('\n').collect();
    let mut grid: Vec<Vec<char>> = Vec::new();
    
    // create grid
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
                /* 
                //part 1
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
                }  */
                //part 2    
                adjacent_seats = grid.find_seat(icol ,irow);
                
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
                        // part 1
                        // if occupied_seats >= 4 {

                        if occupied_seats >= 5 {
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






trait GetAndFindValue {
    fn get_value(&self, x: isize, y: isize) -> char;
    fn find_seat(&self, start_x: isize, start_y: isize) -> Vec<char>;
}

impl GetAndFindValue for Vec<Vec<char>> {
    //replacing all invalid indexes with !floor! empty chair actually, doesnt affect part 1 
    fn get_value(&self, x: isize, y: isize) -> char {
        if x < 0 || y < 0 {
            return 'L';
        }
        match self.get(y as usize) {
            Some(r) => match r.get(x as usize) {
                Some(rr) => return *rr,
                None => return 'L',
            },
            None => return 'L',
        }
    }
    fn find_seat(&self, start_x: isize, start_y: isize) -> Vec<char> {
        let mut result: Vec<char> = Vec::new();

        for win_row in (start_y - 1)..(start_y + 2) {
            for win_col in (start_x - 1)..(start_x + 2) {

                if win_row == start_y && win_col == start_x {
                    continue;
                    // idk how to invert this condition 
                }
                else if self.get_value(win_col, win_row) == '.' {
                    // did not bother to create recursive function
                    let mut offset_x = win_col - start_x; 
                    let mut offset_y = win_row - start_y;
                    'find_seat: loop {
                        let current_char = self.get_value(start_x + offset_x, start_y + offset_y);
                        if current_char != '.' {
                            break 'find_seat;
                        }
                        offset_x += win_col - start_x;
                        offset_y += win_row - start_y;

                    }
                    result.push(self.get_value(start_x + offset_x, start_y + offset_y));
                }
                else {
                    result.push(self.get_value(win_col, win_row));
                }
            }
        }  
        return result
    }
}
