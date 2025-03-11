use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use crate::species::{BattleCard, Species, StepResult};
use game_of_life::*;
use crate::mutations::mutations;
use rand::Rng;

pub struct Ecosystem {
    pub species: Vec<Species>,
    pub dynamics: Vec<Vec<isize>>,
    pub  rand_fill_values: Vec<f32>,
    pub mutations_rate: f32,
}

impl Ecosystem {
    pub fn default() -> Self {
        Self {
            species: vec![Species::default(0, egui::Color32::BLACK)],
            dynamics: vec![vec![1]],
            rand_fill_values: vec![0.25],
            mutations_rate: 0.0,
        }
    }

    pub fn clear_live_cells(&mut self) {
        for species in &mut self.species {
            species.live_cells.clear();
        }
    }

    pub fn randomize_cells(&mut self, grid_size: (isize, isize)) {
        self.clear_live_cells();

        let mut rng = rand::thread_rng();
        for x in 0..grid_size.0 {
            for y in 0..grid_size.1 {
                let rand = rng.gen::<f32>();
                let mut sum : f32 = 0.;
                
                for (id, rand_fill_value) in self.rand_fill_values.iter().enumerate() {
                    sum += rand_fill_value;
                    if sum > rand {
                        self.species.get_mut(id).unwrap().live_cells.insert((x, y));
                        break;
                    }
                }
            }
        }
    }

    pub fn add_species(&mut self) {
        for dynamic in &mut self.dynamics {
            dynamic.push(0);
        }
        self.dynamics.push(vec![0;self.species.len()]);
        self.dynamics.get_mut(self.species.len()).unwrap().push(1);
        self.rand_fill_values.push(0.);
        self.species.push(Species::default(self.species.len(), random_color32()));
    }

    pub fn mutate(&mut self, x : isize, y: isize, original_species : usize) {
        
    }

    pub fn step_infinite(&mut self) {
        let mut all_surviving_cells : HashSet<(isize, isize)> = HashSet::new();
        let mut all_spawned_cells : HashMap<(isize, isize), BattleCard> = HashMap::new();

        for species in &mut self.species {
            species.neighbour_map.clear()
        }

        for (actionner_id, v) in self.dynamics.iter().enumerate() {
            let live_cells = self.species.get(actionner_id).unwrap().live_cells.clone(); // really inneficient, fuck this
            for (receiver_id, effect) in v.iter().enumerate() {
                if *effect != 0 {
                    if let Some(receiver) = self.species.get_mut(receiver_id) {
                        for (x, y) in &live_cells {
                            effect_neighbours(&mut receiver.neighbour_map, *effect, *x, *y);
                        } 
                    }
                }
            }
        }

        for species in &mut self.species {
            let StepResult {surviving_cells, spawned_cells} = species.step();
            all_surviving_cells.extend(&surviving_cells);
            species.live_cells = surviving_cells;
            all_spawned_cells_extend(&mut all_spawned_cells, &spawned_cells);
        }
        
        all_spawned_cells.retain(|k, v| !all_surviving_cells.contains(k) && v.id != usize::MAX);
        for (k, v) in &all_spawned_cells {
            self.species.get_mut(v.id as usize).unwrap().live_cells.insert(*k);
        }
    }
}

fn all_spawned_cells_extend(all_spawned_cells: &mut HashMap<(isize, isize), BattleCard>, spawned_cells: &HashMap<(isize, isize), BattleCard>) {
    for (key, new_value) in spawned_cells {
        
        all_spawned_cells.entry(*key)
            .and_modify(|old_value| { 
                match &new_value.cmp(&old_value) {
                    Ordering::Greater => *old_value = *new_value,
                    // Ordering::Equal => old_value.id = usize::MAX,  // blank if equal
                    _ => {},
                } 
            })
            .or_insert(*new_value); // Insert the new value if the key doesn't exist
    }
}

pub fn effect_neighbours(map: &mut HashMap<(isize, isize), isize>, effect: isize, x: isize, y: isize) {
    for j in (y-1)..=(y+1) { 
        for i in (x-1)..=(x+1) {
            if let Some(val) = map.get_mut(&(i, j)) {
                *val += effect;
            }
            else {
                map.insert((i, j), effect);
            }
        }
    }

    // Adjust to avoid counting the cell itself
    if let Some(val) = map.get_mut(&(x, y)) {
        *val -= effect;
    }
}