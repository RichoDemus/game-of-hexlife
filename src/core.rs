use std::collections::HashSet;
use std::cmp;
use rand::{thread_rng, Rng};

#[derive(Hash, Eq, PartialEq)]
pub struct Coordinate {
    pub y: i64,
    pub x: i64,
}

impl  Coordinate{
 fn new(x: i64, y:i64) -> Self {
     Coordinate{y,x}
 }
}

pub struct Core {
    pub grid: HashSet<Coordinate>,
}

impl Default for Core {
    fn default() -> Self {
        let mut core = Core {
            grid: HashSet::new(),
        };

        for y in 0..10 {
            for x in 0..10 {
                if thread_rng().gen_bool(0.5) {
                    core.grid.insert(Coordinate::new(x,y));
                }
            }
        }

        core
    }
}



impl Core {
    pub fn tick(&mut self) {
        let mut next_generation = HashSet::new();

    let (y_min, y_max, x_min, x_max) = calc_boundary(&self.grid);


    for y in (y_min-1)..=(y_max+1) {
        for x in (x_min-1)..=(x_max+1) {
            let cell = Coordinate::new(x, y);
            let num_neighbours = count_neighbour(&cell, &self.grid);
            let alive = self.grid.contains(&cell);

            let new_state = if alive {
                num_neighbours == 2
            } else {
                num_neighbours == 2
            };

            if new_state {
                next_generation.insert(cell);
            }
        }
    }

        self.grid = next_generation;


    }
}

fn calc_boundary(coordinates: &HashSet<Coordinate>) -> (i64, i64, i64, i64) {
    assert_eq!(coordinates.is_empty(), false, "should be some cells");
    coordinates
        .iter()
        .fold(None, |left_option, right| match left_option {
            None => Some((right.y, right.y, right.x, right.x)),
            Some((y_min, y_max, x_min, x_max)) => Some((
                cmp::min(y_min, right.y),
                cmp::max(y_max, right.y),
                cmp::min(x_min, right.x),
                cmp::max(x_max, right.x),
            )),
        })
        .unwrap()
}

fn count_neighbour(cell: &Coordinate, grid: &HashSet<Coordinate>)  -> u8 {
    let neighbour_offsets = vec![
        // (0,0),
        (1,0),
        (0,1),
        (-1,0),
        (0,-1),
        (1,-1),
        (-1,1),
    ];

    let mut neighbours = 0;
    for (x,y) in neighbour_offsets {
        let coordinate = Coordinate::new(cell.x + x, cell.y + y);
        if grid.contains(&coordinate) {
            neighbours += 1;
        }
    }
    neighbours
}
