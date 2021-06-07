use std::fmt;
use std::ops::RangeInclusive;
pub enum Cube {
    Dead,
    Active,
}

impl std::clone::Clone for Cube {
    fn clone(&self) -> Cube {
        match self {
            Cube::Dead => Cube::Dead,
            Cube::Active => Cube::Active,
        }
    }
}

//                    z   y   x
type CubesElements = Vec<Vec<Vec<Cube>>>;
type Coordinates = (usize, usize, usize);
pub struct Cubes {
    elements: CubesElements,
}

impl Cubes {
    pub fn new(input_data: &str) -> Cubes {
        // let elements: CubesElements = vec![vec![vec![]]];

        let starting_region = input_data
            .lines()
            .map(|line| {
                line.chars()
                    .map(move |c| if c == '.' { Cube::Dead } else { Cube::Active })
                    .collect::<Vec<Cube>>()
            })
            .collect::<Vec<Vec<Cube>>>();
        Cubes {
            elements: vec![starting_region],
        }
    }

    pub fn get(&self, coordinates: Coordinates) -> &Cube {
        &self.elements[coordinates.0][coordinates.1][coordinates.2]
    }

    pub fn count_active_cubes(&self) -> usize {
        self.elements
            .iter()
            .flatten()
            .flatten()
            .fold(0, |acc, cube| match cube {
                Cube::Active => acc + 1,
                _ => acc,
            })
    }

    pub fn get_active_adjacent_cubes(&self, coordinates: Coordinates) -> Vec<Coordinates> {
        self.get_adjacent_cubes(coordinates)
            .into_iter()
            .filter(|adjacent_cube| match self.get(*adjacent_cube) {
                Cube::Active => true,
                _ => false,
            })
            .collect()
    }

    pub fn get_adjacent_cubes(&self, coordinates: Coordinates) -> Vec<Coordinates> {
        let mut adjacent_cubes: Vec<Coordinates> = vec![];

        for z_index in get_adjacent_range(coordinates.0) {
            match self.elements.get(z_index) {
                Some(z) => {
                    for y_index in get_adjacent_range(coordinates.1) {
                        match z.get(y_index) {
                            Some(y) => {
                                for x_index in get_adjacent_range(coordinates.2) {
                                    match y.get(x_index) {
                                        Some(_) => {
                                            let cube_coordinates = (z_index, y_index, x_index);
                                            if cube_coordinates != coordinates {
                                                adjacent_cubes.push(cube_coordinates);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        adjacent_cubes
    }

    fn expand_layers(&mut self) {
        let layers_width = self.elements[0][0].len() + 2;

        self.elements.iter_mut().for_each(|layer| {
            layer.insert(0, populate_y(layers_width));
            layer.iter_mut().skip(1).for_each(|row| {
                row.insert(0, Cube::Dead);
                row.push(Cube::Dead);
            });
            layer.push(populate_y(layers_width));
        });
    }

    pub fn expand_universe(&mut self) {
        self.expand_layers();

        let universe_width = self.elements[0][0].len();
        let universe_height = self.elements[0].len();
        self.elements
            .insert(0, populate_z(universe_height, universe_width));
        self.elements
            .push(populate_z(universe_height, universe_width));
    }

    pub fn cycle_cubes(&mut self) {
        self.expand_universe();
        let new_universe = (&self.elements).to_vec();

        // bad solution, almost like slapping a goto
        let get_new_cube_state = |coordinates: Coordinates, cube: Cube| -> Cube {
            let active_adjacent_cubes_count = self.get_active_adjacent_cubes(coordinates).len();
            match cube {
                Cube::Active => {
                    if active_adjacent_cubes_count == 2 || active_adjacent_cubes_count == 3 {
                        Cube::Active
                    } else {
                        Cube::Dead
                    }
                }
                _ => {
                    if active_adjacent_cubes_count == 3 {
                        Cube::Active
                    } else {
                        Cube::Dead
                    }
                }
            }
        };

        self.elements = new_universe
            .into_iter()
            .enumerate()
            .map(|(idz, layer)| {
                layer
                    .into_iter()
                    .enumerate()
                    .map(move |(idy, row)| {
                        row.into_iter()
                            .enumerate()
                            .map(move |(idx, cube)| get_new_cube_state((idz, idy, idx), cube))
                            .collect::<Vec<Cube>>()
                    })
                    .collect::<Vec<Vec<Cube>>>()
            })
            .collect::<CubesElements>();
    }
}

fn populate_z(height: usize, width: usize) -> Vec<Vec<Cube>> {
    (0..height).map(|_| populate_y(width)).collect()
}
fn populate_y(width: usize) -> Vec<Cube> {
    (0..width).map(|_| Cube::Dead).collect()
}

fn get_adjacent_range(coordinate: usize) -> RangeInclusive<usize> {
    let start = if coordinate == 0 { 0 } else { coordinate - 1 };
    start..=coordinate + 1
}

impl fmt::Debug for Cubes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (*self.elements)
            .into_iter()
            .enumerate()
            .for_each(|(idx, y_layer)| {
                writeln!(f, "z layer: {}", idx);
                writeln!(
                    f,
                    "{}",
                    y_layer
                        .into_iter()
                        .map(|x_layer| {
                            x_layer
                                .into_iter()
                                .map(|cube| match cube {
                                    Cube::Active => '#',
                                    Cube::Dead => '.',
                                })
                                .collect::<String>()
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                );
            });
        write!(f, "")
    }
}
