use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use egui::Color32;

pub struct StepResult {
    pub surviving_cells : HashSet<(isize, isize)>,
    pub spawned_cells : HashMap<(isize, isize), BattleCard>,
}

#[derive(Copy, Clone)]
pub struct BattleCard {
    pub neighbours : isize,
    pub priority : isize,
    pub id : usize,  // usizemax for blank
}

impl BattleCard {
    pub fn new(neighbours: isize, priority: isize, id: usize) -> Self {
        Self { neighbours, priority, id }
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        match self.neighbours.cmp(&other.neighbours) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            _ => {},
        }
        
        match self.priority.cmp(&other.priority) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

pub struct Species {
    pub id: usize,
    pub name: String,
    pub live_cells: HashSet<(isize, isize)>,
    pub neighbour_map : HashMap<(isize, isize), isize>,
    pub neighbours_survival: (isize, isize),
    pub neighbours_spawn: (isize, isize),
    pub color: egui::Color32,
    pub priority: isize,
}

impl Species {
    pub fn default(id: usize, color: Color32) -> Self {
        Self {
            id,
            name: String::from("joe"),
            live_cells: HashSet::new(),
            neighbour_map: HashMap::new(),
            neighbours_survival: (2, 3),
            neighbours_spawn: (3,3),
            color,
            priority: 0,
        }
    }

    pub fn step(&self) -> StepResult {
        let mut surviving_cells : HashSet<(isize, isize)> = HashSet::new();
        let mut spawned_cells : HashMap<(isize, isize), BattleCard> = HashMap::new();

        // Determine the next generation of live cells based on neighbor counts
        for (cell, neighbours) in &self.neighbour_map {
            if self.live_cells.contains(&cell) && neighbours >= &self.neighbours_survival.0 && neighbours <= &self.neighbours_survival.1 {
                surviving_cells.insert(*cell);
            }
            else if neighbours >= &self.neighbours_spawn.0 && neighbours <= &self.neighbours_spawn.1 {
                spawned_cells.insert(*cell, BattleCard::new(*neighbours, self.priority, self.id) );
            }
        }

        StepResult {surviving_cells, spawned_cells}
    }
}