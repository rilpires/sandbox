// This is world_grid module
use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::cmp::*;
use std::usize;

use crate::datatype::{GridMap, Vector2};


#[derive(Clone, PartialEq)]
pub struct ParticleData {
    pub speed: (f32, f32),
    pub color: Color,
}

#[derive(Clone)]
pub struct WorldGrid {
    world_rng : ThreadRng,
    width: usize,
    height: usize,
    grid: GridMap<CellType>,
    grid_rooms_hotness: GridMap<usize>,
}


#[derive(Clone, PartialEq)]
pub enum CellType {
    Empty,
    Sand(ParticleData),
    Block(ParticleData),
}

impl WorldGrid {
    pub fn new(width: usize, height: usize) -> WorldGrid {
        WorldGrid {
            world_rng: rand::thread_rng(),
            width,
            height,
            grid: GridMap::new(800, 600, CellType::Empty),
            grid_rooms_hotness: GridMap::new(10, 10, 100),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn room_size(&self) -> Vector2<usize> {
        Vector2 {
            x: self.grid.width(),
            y: self.grid.height(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &CellType {
        self.grid.get(x, y)
    }

    pub fn set(&mut self, x: usize, y: usize, cell_type: CellType) {
        self.grid.set(x, y, cell_type)
    }

    pub fn process_frame(&mut self) {
        let mut tmp_grid = self.grid.clone();
        for room_x in (0..self.grid_rooms_hotness.width()) {
            for room_y in (0..self.grid_rooms_hotness.height()) {
                self.process_room(&mut tmp_grid, room_x, room_y);
            }   
        }
    }

    fn process_room(&mut self, tmp_grid:&mut GridMap<CellType>, room_x:usize, room_y:usize) {
        let room_size = self.room_size();
        let mut xvec : Vec<usize> = (
            (room_x*room_size.x)..min(self.width, (room_x+1)*room_size.x)
        ).collect();
        let mut yvec : Vec<usize> = (
            (room_y*room_size.y)..min(self.height, (room_y+1)*room_size.y)
        ).collect();
        xvec.shuffle(&mut self.world_rng);
        // yvec.reverse();
        yvec.shuffle(&mut self.world_rng);
        for x in xvec {
            for &y in &yvec {
                match tmp_grid.get(x, y) {
                    CellType::Empty => {},
                    CellType::Sand(data) => {
                        let mut dy = f32::floor(data.speed.1) as usize;
                        while dy >= 1 {
                            if y < self.height - dy {
                                if *self.get(x, y + dy ) == CellType::Empty {
                                    let mut new_data = data.clone();
                                    new_data.speed.1 += 0.2;
                                    self.set(x, y, CellType::Empty);
                                    self.set(x, y + dy, CellType::Sand(new_data));
                                    break;
                                } else {
                                    let mut new_data = data.clone();
                                    new_data.speed.1 = 1.0;
                                    let mut fall_right = (x < self.width-1) && (*self.get(x + 1, y + 1) == CellType::Empty);
                                    let mut fall_left = (x > 0) && (*self.get(x - 1, y + 1) == CellType::Empty);  
                                    if (fall_left && fall_right) {
                                        fall_right = self.world_rng.gen_range(-1..=1) > 0;
                                        fall_left = !fall_right;
                                    }
                                    if fall_right {
                                        self.set(x, y, CellType::Empty);
                                        self.set(x + 1, y + 1, CellType::Sand(new_data));
                                    } else if fall_left  {
                                        self.set(x, y, CellType::Empty);
                                        self.set(x - 1, y + 1, CellType::Sand(new_data));
                                    }
                                    break;
                                }
                            }
                            dy -= 1;
                        }
                    },
                    CellType::Block(_) => {},
                }
            
            }
        }
    }

}
