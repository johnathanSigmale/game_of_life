use std::collections::{HashMap, HashSet};

use egui::Color32;
use rand::Rng;
 
pub fn step_infinite(
    black_cells: &HashSet<(isize, isize)>,
    neighbors_survival: &(isize, isize),
    neighbors_spawn: &(isize, isize),
) -> HashSet<(isize, isize)> {
    let mut neighbour_map : HashMap<(isize, isize), isize> = HashMap::new();
    let mut black_cells_new : HashSet<(isize, isize)> = HashSet::new();

    // Count neighbors for each black cell
    for &(x, y) in black_cells {
        increment_neighbours(&mut neighbour_map, x, y);
    }

    // Determine the next generation of black cells based on neighbor counts
    for (cell, neighbours) in neighbour_map {
        if black_cells.contains(&cell) && neighbours >= neighbors_survival.0 && neighbours <= neighbors_survival.1 {
            black_cells_new.insert(cell);
        }
        else if neighbours >= neighbors_spawn.0 && neighbours <= neighbors_spawn.1 {
            black_cells_new.insert(cell);
        } 
    }

    black_cells_new
}

pub fn step_finite(
    black_cells: &HashSet<(isize, isize)>,
    neighbors_survival: &(isize, isize),
    neighbors_spawn: &(isize, isize),
    gridsize: &(isize, isize),
    pass_through: &bool,
) -> HashSet<(isize, isize)> {
    let mut neighbour_map : HashMap<(isize, isize), isize> = HashMap::new();
    let mut black_cells_new : HashSet<(isize, isize)> = HashSet::new();

    // Count neighbors for each black cell
    for &(x, y) in black_cells {
        if *pass_through {increment_neighbours_pass_through(&mut neighbour_map, x, y, gridsize);}
        else {increment_neighbours(&mut neighbour_map, x, y);}
    }

    // Determine the next generation of black cells based on neighbor counts
    for ((x, y), neighbours) in neighbour_map {
        if x >= 0 && y >= 0 && x < gridsize.0 && y < gridsize.1 {
            let cell = (x, y);
            if black_cells.contains(&cell) && neighbours >= neighbors_survival.0 && neighbours <= neighbors_survival.1 {
                black_cells_new.insert(cell);
            }
            else if neighbours >= neighbors_spawn.0 && neighbours <= neighbors_spawn.1 {
                black_cells_new.insert(cell);
            } 
        }
    }

    black_cells_new
}

pub fn increment_neighbours(map: &mut HashMap<(isize, isize), isize>, x: isize, y: isize) {
    for j in (y-1)..=(y+1) { 
        for i in (x-1)..=(x+1) {
            if let Some(val) = map.get_mut(&(i, j)) {
                *val += 1;
            }
            else {
                map.insert((i, j), 1);
            }
        }
    }

    // Adjust to avoid counting the cell itself
    if let Some(val) = map.get_mut(&(x, y)) {
        *val -= 1;
    }
}

pub fn increment_neighbours_pass_through(map: &mut HashMap<(isize, isize), isize>, x: isize, y: isize, grid_size: &(isize, isize)) {
    for j in [positive_modulo(y - 1, grid_size.1), y, positive_modulo(y + 1, grid_size.1)] { 
        for i in [positive_modulo(x - 1, grid_size.0), x, positive_modulo(x + 1, grid_size.0)] {
            if let Some(val) = map.get_mut(&(i, j)) {
                *val += 1;
            }
            else {
                map.insert((i, j), 1);
            }
        }
    }

    // Adjust to avoid counting the cell itself
    if let Some(val) = map.get_mut(&(x, y)) {
        *val -= 1;
    }
}

fn positive_modulo(n: isize, m: isize) -> isize {
    ((n % m) + m) % m
}

pub fn random_color32() -> Color32 {
    let mut rng = rand::thread_rng();
    let r = rng.gen::<u8>();
    let g = rng.gen::<u8>();
    let b = rng.gen::<u8>();

    Color32::from_rgb(r, g, b)
}

